//! Single-shot Plonky2 univariate FRI worker (Goldilocks, Poseidon2).
//!
//! The opening point and every FRI challenge are drawn from the quartic
//! extension of Goldilocks rather than the quadratic extension of Plonky2's
//! shipped configurations. Over the quadratic extension (about 128 bits) no
//! proven FRI bound reaches 100 bits at the benchmarked sizes; see
//! `proven_bits`.

use pcs_bench_core::{
    RunStatus, WorkerOutput, HASH_SECURITY_BITS_100, PLONKY2_CAP_HEIGHT, PLONKY2_FRI_POW_BITS,
    PLONKY2_FRI_QUERIES, PLONKY2_FRI_RATE_BITS,
};
use plonky2::field::extension::quartic::QuarticExtension;
use plonky2::field::extension::{Extendable, FieldExtension};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::field::polynomial::PolynomialValues;
use plonky2::field::types::Field;
use plonky2::fri::oracle::PolynomialBatch;
use plonky2::fri::reduction_strategies::FriReductionStrategy;
use plonky2::fri::structure::{
    FriBatchInfo, FriInstanceInfo, FriOpeningBatch, FriOpenings, FriOracleInfo, FriPolynomialInfo,
};
use plonky2::fri::verifier::verify_fri_proof;
use plonky2::fri::{FriConfig, FriParams};
use plonky2::hash::poseidon2::hash::Poseidon2Hash;
use plonky2::iop::challenger::Challenger;
use plonky2::plonk::config::GenericConfig;
use plonky2::util::timing::TimingTree;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Instant;

/// `Poseidon2GoldilocksConfig` with the quartic instead of the quadratic
/// extension as the challenge field. Hashing is unchanged.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, serde::Serialize)]
struct Poseidon2GoldilocksQuarticConfig;

impl GenericConfig<D> for Poseidon2GoldilocksQuarticConfig {
    type F = GoldilocksField;
    type FE = QuarticExtension<GoldilocksField>;
    type Hasher = Poseidon2Hash;
    type InnerHasher = Poseidon2Hash;
}

type C = Poseidon2GoldilocksQuarticConfig;
type F = GoldilocksField;
const D: usize = 4;

/// `log2` of the challenge-field size, rounded down to two decimals.
const CHALLENGE_FIELD_BITS: f64 = 255.99;
/// Johnson-regime multiplicity parameter `m` of BCIKS20 Theorem 8.3. It is
/// fixed rather than optimised so that the commit-phase term keeps more than
/// 40 bits of margin at every benchmarked size.
const JOHNSON_M: f64 = 64.0;

fn main() -> ExitCode {
    let threads = parse_u32_flag("--threads").unwrap_or(1).max(1);
    init_thread_pool(threads);
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let _ = emit(&WorkerOutput {
                status: RunStatus::Error,
                status_detail: Some(error),
                log2_n: None,
                timings_ns: BTreeMap::new(),
                proof_bytes: None,
                commitment_bytes: None,
                evaluation_bytes: None,
                public_context_bytes: None,
                state_bytes: None,
                peak_rss_bytes: peak_rss_bytes(),
            });
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let log2_n = parse_u32_flag("--log2-n")?;
    emit(&timed_fri(log2_n)?)
}

fn timed_fri(log2_n: u32) -> Result<WorkerOutput, String> {
    let degree_bits = log2_n as usize;
    let n = 1usize << degree_bits;
    let mut rng = StdRng::seed_from_u64(configured_seed(0));
    let values = PolynomialValues::new((0..n).map(|_| random_goldilocks(&mut rng)).collect());

    let t0 = Instant::now();
    let fri_config = FriConfig {
        rate_bits: PLONKY2_FRI_RATE_BITS,
        cap_height: PLONKY2_CAP_HEIGHT,
        proof_of_work_bits: PLONKY2_FRI_POW_BITS as u32,
        reduction_strategy: FriReductionStrategy::ConstantArityBits(4, 5),
        num_query_rounds: PLONKY2_FRI_QUERIES,
    };
    let fri_params = fri_config.fri_params(degree_bits, false);
    let proven = proven_bits(degree_bits, &fri_params);
    if proven < f64::from(HASH_SECURITY_BITS_100) {
        return Err(format!(
            "Plonky2 FRI profile proves only {proven:.2} bits at log2 N = {degree_bits}"
        ));
    }
    let mut timing = TimingTree::default();
    let setup_ns = elapsed_ns(t0);

    let t0 = Instant::now();
    let oracle = PolynomialBatch::<F, C, D>::from_values(
        vec![values],
        fri_config.rate_bits,
        false,
        fri_config.cap_height,
        &mut timing,
        None,
    );
    let commit_ns = elapsed_ns(t0);

    // End-to-end opening starts with transcript reconstruction and includes
    // deriving and evaluating the point whose claim is supplied to FRI.
    let t0 = Instant::now();
    let mut prover_challenger = Challenger::<F, <C as GenericConfig<D>>::Hasher>::new();
    prover_challenger.observe_cap(&oracle.merkle_tree.cap);
    let zeta = prover_challenger.get_extension_challenge::<D>();
    let claimed = oracle.polynomials[0].to_extension::<D>().eval(zeta);

    let instance = FriInstanceInfo {
        oracles: vec![FriOracleInfo {
            num_polys: 1,
            blinding: false,
        }],
        batches: vec![FriBatchInfo {
            point: zeta,
            polynomials: vec![FriPolynomialInfo {
                oracle_index: 0,
                polynomial_index: 0,
            }],
        }],
    };
    let openings = FriOpenings {
        batches: vec![FriOpeningBatch {
            values: vec![claimed],
        }],
    };

    // Same transcript order as `plonk::prover`: bind the claimed openings before FRI.
    prover_challenger.observe_openings(&openings);

    let proof = PolynomialBatch::<F, C, D>::prove_openings(
        &instance,
        &[&oracle],
        &mut prover_challenger,
        &fri_params,
        None,
        None,
        &mut timing,
    );
    let open_ns = elapsed_ns(t0);

    // Complete verification includes transcript reconstruction and all
    // proof-dependent Fiat–Shamir challenge derivation.
    let t0 = Instant::now();
    let mut verifier_challenger = Challenger::<F, <C as GenericConfig<D>>::Hasher>::new();
    verifier_challenger.observe_cap(&oracle.merkle_tree.cap);
    let zeta_v = verifier_challenger.get_extension_challenge::<D>();
    if zeta_v != zeta {
        return Err("verifier zeta drifted from prover".into());
    }
    verifier_challenger.observe_openings(&openings);
    let challenges = verifier_challenger.fri_challenges::<C, D>(
        &proof.commit_phase_merkle_caps,
        &proof.final_poly,
        proof.pow_witness,
        degree_bits,
        &fri_config,
        None,
        None,
    );

    verify_fri_proof::<F, C, D>(
        &instance,
        &openings,
        &challenges,
        &[oracle.merkle_tree.cap.clone()],
        &proof,
        &fri_params,
    )
    .map_err(|error| error.to_string())?;
    let verify_ns = elapsed_ns(t0);

    if negative_check_enabled() {
        let altered_openings = FriOpenings {
            batches: vec![FriOpeningBatch {
                values: vec![
                    claimed
                        + <<F as Extendable<D>>::Extension as FieldExtension<D>>::from_basefield(
                            F::ONE,
                        ),
                ],
            }],
        };
        let mut negative_challenger = Challenger::<F, <C as GenericConfig<D>>::Hasher>::new();
        negative_challenger.observe_cap(&oracle.merkle_tree.cap);
        let _ = negative_challenger.get_extension_challenge::<D>();
        negative_challenger.observe_openings(&altered_openings);
        let altered_challenges = negative_challenger.fri_challenges::<C, D>(
            &proof.commit_phase_merkle_caps,
            &proof.final_poly,
            proof.pow_witness,
            degree_bits,
            &fri_config,
            None,
            None,
        );
        if verify_fri_proof::<F, C, D>(
            &instance,
            &altered_openings,
            &altered_challenges,
            &[oracle.merkle_tree.cap.clone()],
            &proof,
            &fri_params,
        )
        .is_ok()
        {
            return Err("Plonky2 verifier accepted an altered opening claim".into());
        }
    }

    let proof_bytes = bincode_len(&proof);
    let commitment_bytes = bincode_len(&oracle.merkle_tree.cap);

    let mut timings_ns = BTreeMap::new();
    timings_ns.insert("setup".into(), setup_ns);
    timings_ns.insert("commit".into(), commit_ns);
    timings_ns.insert("open".into(), open_ns);
    timings_ns.insert("verify".into(), verify_ns);

    Ok(WorkerOutput {
        status: RunStatus::Ok,
        status_detail: Some(format!(
            "plonky2-fri-100-johnson,statement=univariate,distribution=full-field-uniform,point=transcript-extension,challenge_field=goldilocks-ext4,rate=1/{},queries={},pow_bits={},security_model=proven-johnson-bciks20,johnson_m={},proven_bits={:.2}",
            1usize << PLONKY2_FRI_RATE_BITS,
            PLONKY2_FRI_QUERIES,
            PLONKY2_FRI_POW_BITS,
            JOHNSON_M,
            proven
        )),
        log2_n: Some(log2_n),
        timings_ns,
        proof_bytes,
        commitment_bytes,
        evaluation_bytes: bincode_len(&claimed),
        public_context_bytes: Some(0),
        state_bytes: Some(0),
        peak_rss_bytes: peak_rss_bytes(),
    })
}

/// Proven soundness, in bits, of one opening in the Johnson regime.
///
/// Hand-derived from the FRI soundness theorem of Ben-Sasson, Carmon, Ishai,
/// Kopparty and Saraf, "Proximity Gaps for Reed-Solomon Codes" (eprint
/// 2020/654), Theorem 8.3. With rate `rho`, initial domain `D0`, folding
/// arities `a_i`, `s` queries, challenge field `K`, and an integer `m >= 3`,
///
/// ```text
/// alpha   = sqrt(rho) * (1 + 1/(2m))
/// eps_C   = (m + 1/2)^7 * |D0|^2 / (2 * rho^(3/2) * |K|)
///         + (2m + 1) * (|D0| + 1) * (sum a_i) / (sqrt(rho) * |K|)
/// eps_FRI = eps_C + alpha^s
/// ```
///
/// The query term is multiplied by `2^-pow_bits` for the grind that precedes
/// the query indices. The out-of-domain quotient is charged
/// `L^2 * 2^n / |K|` with the Johnson list size `L = (m + 1/2) / sqrt(rho)`.
/// The result is `-log2` of the sum of all terms. No conjecture on proximity
/// gaps or list decoding beyond the Johnson radius is used.
fn proven_bits(degree_bits: usize, fri_params: &FriParams) -> f64 {
    let m = JOHNSON_M;
    let rate_bits = fri_params.config.rate_bits as f64;
    let log_domain = degree_bits as f64 + rate_bits;
    let arity_sum: f64 = fri_params
        .reduction_arity_bits
        .iter()
        .map(|&bits| (1u64 << bits) as f64)
        .sum::<f64>()
        .max(1.0);
    let log_list = (m + 0.5).log2() + rate_bits / 2.0;
    let log_alpha = -rate_bits / 2.0 + (1.0 + 1.0 / (2.0 * m)).log2();
    let terms = [
        7.0 * (m + 0.5).log2() + 2.0 * log_domain - 1.0 + 1.5 * rate_bits,
        (2.0 * m + 1.0).log2()
            + (log_domain.exp2() + 1.0).log2()
            + arity_sum.log2()
            + rate_bits / 2.0,
        2.0 * log_list + degree_bits as f64,
    ];
    let algebraic: f64 = terms
        .iter()
        .map(|log_numerator| (log_numerator - CHALLENGE_FIELD_BITS).exp2())
        .sum();
    let query = (fri_params.config.num_query_rounds as f64 * log_alpha
        - f64::from(fri_params.config.proof_of_work_bits))
    .exp2();
    -(algebraic + query).log2()
}

fn bincode_len<T: serde::Serialize>(value: &T) -> Option<u64> {
    bincode::serialized_size(value).ok()
}

fn init_thread_pool(threads: u32) {
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(threads.max(1) as usize)
        .stack_size(64 * 1024 * 1024)
        .build_global();
}

fn configured_seed(domain: u64) -> u64 {
    std::env::var("PCS_BENCH_SEED")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map_or(domain, |seed| seed ^ domain)
}

fn negative_check_enabled() -> bool {
    std::env::var("PCS_BENCH_NEGATIVE_CHECK").as_deref() == Ok("1")
}

fn random_goldilocks(rng: &mut StdRng) -> F {
    const MODULUS: u64 = 0xffff_ffff_0000_0001;
    loop {
        let candidate = rng.gen::<u64>();
        if candidate < MODULUS {
            return F::from_canonical_u64(candidate);
        }
    }
}

fn parse_u32_flag(name: &str) -> Result<u32, String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == name {
            return args
                .next()
                .ok_or_else(|| format!("{name} requires a value"))?
                .parse()
                .map_err(|_| format!("invalid {name}"));
        }
        if let Some(value) = arg.strip_prefix(&format!("{name}=")) {
            return value.parse().map_err(|_| format!("invalid {name}"));
        }
    }
    Err(format!("missing {name}"))
}

fn elapsed_ns(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

fn emit(output: &WorkerOutput) -> Result<(), String> {
    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, output).map_err(|error| error.to_string())?;
    stdout.write_all(b"\n").map_err(|error| error.to_string())
}

fn peak_rss_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        let Some(rest) = line.strip_prefix("VmHWM:") else {
            continue;
        };
        let kb: u64 = rest.split_whitespace().next()?.parse().ok()?;
        return Some(kb.saturating_mul(1024));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{proven_bits, FriConfig, FriReductionStrategy};
    use pcs_bench_core::{
        HASH_SECURITY_BITS_100, PLONKY2_CAP_HEIGHT, PLONKY2_FRI_POW_BITS, PLONKY2_FRI_QUERIES,
        PLONKY2_FRI_RATE_BITS,
    };

    fn bits(degree_bits: usize, queries: usize) -> f64 {
        let config = FriConfig {
            rate_bits: PLONKY2_FRI_RATE_BITS,
            cap_height: PLONKY2_CAP_HEIGHT,
            proof_of_work_bits: PLONKY2_FRI_POW_BITS as u32,
            reduction_strategy: FriReductionStrategy::ConstantArityBits(4, 5),
            num_query_rounds: queries,
        };
        proven_bits(degree_bits, &config.fri_params(degree_bits, false))
    }

    #[test]
    fn query_count_is_the_smallest_that_proves_100_bits() {
        let target = f64::from(HASH_SECURITY_BITS_100);
        for degree_bits in 10..=30 {
            assert!(bits(degree_bits, PLONKY2_FRI_QUERIES) >= target);
            assert!(bits(degree_bits, PLONKY2_FRI_QUERIES - 1) < target);
        }
    }

    #[test]
    fn commit_phase_terms_keep_a_wide_margin() {
        // With unbounded queries only the field-size terms remain.
        assert!(bits(30, 4096) > 140.0);
    }
}
