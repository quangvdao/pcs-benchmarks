#![allow(missing_docs)]

use akita_algebra::poly::multilinear_eval;
use akita_config::proof_optimized::fp128;
use akita_config::CommitmentConfig;
use akita_cpu_backend::{CommitmentHandle, CpuBackend, DensePoly, GroupContext};
use akita_params::{BasisMode, OpeningScheduleSelection};
use akita_pcs::AkitaCommitmentScheme;
use akita_prover::SelectedProverOpeningData;
use akita_types::{CommittedGroup, GroupBatchStatement, OpeningClaims, PolynomialGroupClaims};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use jolt_field::{CanonicalEncoding, Ring};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Duration;

type F = fp128::Field;

const INPUT_SEED: u64 = 0xDEAD_BEEF;
const POINT_SEED: u64 = 0xCAFE_BABE;
const TRANSCRIPT_DOMAIN: &[u8] = b"pcs-benchmark/v1";

fn dense_evaluations<Cfg: CommitmentConfig<Field = F>>(num_vars: usize) -> Vec<F> {
    let mut rng = StdRng::seed_from_u64(INPUT_SEED);
    let decomposition = Cfg::decomposition();
    if decomposition.log_commit_bound >= 128 {
        (0..(1usize << num_vars))
            .map(|_| F::from_u128_reduced(rng.gen::<u128>()))
            .collect()
    } else {
        let half_bound = 1i64 << (decomposition.log_commit_bound.min(62) - 1);
        (0..(1usize << num_vars))
            .map(|_| F::from_i64(rng.gen_range(-half_bound..half_bound)))
            .collect()
    }
}

fn opening_point(num_vars: usize) -> Vec<F> {
    let mut rng = StdRng::seed_from_u64(POINT_SEED);
    (0..num_vars)
        .map(|_| F::from_u128_reduced(rng.gen::<u128>()))
        .collect()
}

fn prover_claims<'a, Cfg>(
    point: &[F],
    opening: F,
    commitment: &CommittedGroup<F>,
    handle: CommitmentHandle<F, F>,
    schedules: &akita_config::TrustedScheduleCatalog<Cfg>,
) -> SelectedProverOpeningData<'a, F, CommitmentHandle<F, F>, F>
where
    Cfg: CommitmentConfig<Field = F, ExtField = F>,
{
    let group = PolynomialGroupClaims::new(point.to_vec(), vec![opening], commitment.clone())
        .expect("benchmark claims are valid");
    let claims = OpeningClaims::from_groups(vec![group]).expect("benchmark claim group is valid");
    SelectedProverOpeningData::from_committed_claims::<Cfg>(claims, vec![handle], schedules)
        .expect("benchmark prover data is valid")
}

fn verifier_claims<'a>(
    selection: OpeningScheduleSelection,
    point: &[F],
    openings: &[F],
    commitment: &'a CommittedGroup<F>,
) -> GroupBatchStatement<'a, F, F> {
    let group = PolynomialGroupClaims::new(point.to_vec(), openings.to_vec(), commitment)
        .expect("benchmark verifier claims are valid");
    let claims = OpeningClaims::from_groups(vec![group]).expect("benchmark claims are valid");
    GroupBatchStatement::new(selection, claims).expect("benchmark statement is valid")
}

#[allow(clippy::too_many_lines)] // Keeping one fixture scope prevents timed-state drift.
fn bench_dense<Cfg>(criterion: &mut Criterion, num_vars: usize)
where
    Cfg: CommitmentConfig<Field = F, ExtField = F>,
{
    let scheme = AkitaCommitmentScheme::<Cfg>::new(
        pcs_bench_akita::schedule_catalog::<Cfg>(num_vars).expect("pinned upstream catalog"),
    );
    let evaluations = dense_evaluations::<Cfg>(num_vars);
    let polynomial =
        DensePoly::<F>::from_field_evals(num_vars, &evaluations).expect("valid dense polynomial");
    let point = opening_point(num_vars);
    let opening = multilinear_eval(&evaluations, &point).expect("valid multilinear evaluation");

    let mut group = criterion.benchmark_group(format!("akita/fp128/dense/nv{num_vars}"));
    group.throughput(Throughput::Elements(1u64 << num_vars));
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(10);

    group.bench_function("setup", |bencher| {
        bencher.iter(|| {
            black_box(
                scheme
                    .setup_prover(black_box(num_vars), black_box(1))
                    .expect("setup succeeds"),
            )
        });
    });

    let setup = scheme.setup_prover(num_vars, 1).expect("setup succeeds");
    let backend =
        CpuBackend::<F, F>::new(setup.expanded.clone()).expect("backend construction succeeds");
    let source = backend
        .import_source(vec![polynomial])
        .expect("source import succeeds");

    group.bench_function("commit", |bencher| {
        bencher.iter(|| {
            black_box(
                backend
                    .commit(
                        scheme.schedules(),
                        black_box(&source),
                        GroupContext::scheduler_without_precommitted_groups(),
                    )
                    .expect("commit succeeds"),
            )
        });
    });

    let output = backend
        .commit(
            scheme.schedules(),
            &source,
            GroupContext::scheduler_without_precommitted_groups(),
        )
        .expect("commit succeeds");
    let verifier = scheme
        .verifier(
            scheme
                .setup_verifier(&setup)
                .expect("verifier setup succeeds"),
        )
        .expect("verifier construction succeeds");

    group.bench_function("prove", |bencher| {
        bencher.iter_batched(
            || output.private_handle.clone(),
            |handle| {
                black_box(
                    scheme
                        .batched_prove(
                            &setup,
                            prover_claims::<Cfg>(
                                &point,
                                opening,
                                &output.committed_group,
                                handle,
                                scheme.schedules(),
                            ),
                            &backend,
                            TRANSCRIPT_DOMAIN,
                            BasisMode::Lagrange,
                        )
                        .expect("proving succeeds"),
                )
            },
            BatchSize::LargeInput,
        );
    });

    let prover_data = prover_claims::<Cfg>(
        &point,
        opening,
        &output.committed_group,
        output.private_handle.clone(),
        scheme.schedules(),
    );
    let selection = prover_data.selection();
    let proof = scheme
        .batched_prove(
            &setup,
            prover_data,
            &backend,
            TRANSCRIPT_DOMAIN,
            BasisMode::Lagrange,
        )
        .expect("proving succeeds");

    group.bench_function("verify", |bencher| {
        bencher.iter(|| {
            verifier
                .batched_verify(
                    black_box(&proof),
                    TRANSCRIPT_DOMAIN,
                    black_box(verifier_claims(
                        selection,
                        &point,
                        &[opening],
                        &output.committed_group,
                    )),
                    BasisMode::Lagrange,
                )
                .expect("verification succeeds");
        });
    });

    group.finish();
}

fn dense_nv14(criterion: &mut Criterion) {
    bench_dense::<fp128::Dense>(criterion, 14);
}

fn dense_nv16(criterion: &mut Criterion) {
    bench_dense::<fp128::Dense>(criterion, 16);
}

criterion_group!(akita, dense_nv14, dense_nv16);
criterion_main!(akita);
