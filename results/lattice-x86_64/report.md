Measurements were collected on a single AMD Ryzen 9 9950X 16-Core Processor (Linux x86_64, 32 logical CPUs, 186~GiB RAM). The CPU advertised AVX-512F, but the executed instruction stream was not independently traced. CPU policy: driver amd-pstate-epp, governor powersave, preference balance_performance.

Our first experiment compares Akita with prior lattice-based PCSs on dense
polynomial data at the target volumes above. Akita samples uniform full-field
coefficients and uniform extension-field opening points. This table is a declared
native-workload survey unless all displayed security and statement assumptions match.
For each input, Akita
uses the validated planner schedule selected for that field and size.
The pinned catalogs omit $n_v=22$ and $n_v=24$; those rows are generated
with that same planner at the measured commit.
Akita appears twice: the direct `fp32-dense` catalog, and the same pin
with recursive setup offloading (`fp32-dense-recursive`).
The comparison is exclusively single-threaded: RoKoKo has no native multithreaded
prover, and Greyhound is pinned to `LATTICE_DOGS_THREADS=1` even though the
reference can parallelize extension products. Greyhound uses the
`l2-quantum128-adps16` Euclidean SIS policy and reports contextual proof bytes.
Timing cells report the median and, when supported by the sample count, a
conservative distribution-free 95% confidence interval. Scheme names link to the exact
git commit that was measured.

The timing comparison separates commitment, opening, and verification, while the
cold total includes setup plus commitment and opening. If an implementation embeds
reusable setup inside commitment, that cost remains in its cold total and its separate
setup entry is reported as unknown; reusable state size is reported when measurable.
The resources table reports communication, memory, and preprocessing.

RoKoKo uses the field $\mathbb{F}_{2^{50}-2687}$ and fixed native parameter
sets corresponding to each target's coefficient count. An OOM entry
is a confirmed allocation failure under the 168 GiB virtual-address-space ceiling. RoKoKo's native field has about 50 bits, but its sampler is bounded to
31-bit coefficients. The displayed payload is nominal field capacity and must not
be interpreted as sampled information content or used to rescale throughput.

### Security targets and accounting

| Scheme | Security bits | Accounting |
| --- | --- | --- |
| Akita (direct and offload) | 128-bit target | Planner-validated Module-SIS and classical-ROM transcript targets. |
| Greyhound | 128-bit SIS target | Euclidean SIS under ADPS16 quantum core-SVP (l2-quantum128-adps16). This is a lattice-hardness policy, not a validated end-to-end transcript bound. |
| RoKoKo | < 100 bits | Fixed native profiles; heuristic soundness accounting. |

These are reported security categories with different accounting scopes, not equivalent end-to-end security guarantees. RoKoKo is reported as a below-100-bit category, not a precise validated estimate.

| Nominal payload | Scheme | Security | Field | log₂ N | Commit (s) | Open (s) | Cold total (s) | Verify (ms) |
| ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| 2^{27} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 22 | 0.039 [0.038, 0.039] | 0.510 [0.506, 0.513] | 0.555 [0.550, 0.558] | 6.1 [6.0, 6.2] |
| 2^{27} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | —(1) | —(1) | —(1) | —(1) | —(1) |
| 2^{27} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 128-bit SIS target | $2^{32}-99$ | 22 | 0.111 [0.110, 0.113] | 0.187 [0.184, 0.192] | 0.300 [0.295, 0.303] | 77.4 [73.4, 78.3] |
| 2^{27} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | < 100 bits | $2^{50}-2687$ | 22 | 0.038 [0.037, 0.038] | 0.144 [0.143, 0.145] | 0.256 [0.255, 0.257] | 4.1 [4.0, 4.1] |
| 2^{29} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 24 | 0.141 [0.140, 0.141] | 0.704 [0.701, 0.705] | 0.850 [0.848, 0.851] | 7.2 [7.2, 7.7] |
| 2^{29} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 24 | 0.139 [0.139, 0.140] | 1.07 [1.06, 1.07] | 1.22 [1.21, 1.22] | 6.7 [6.7, 6.8] |
| 2^{29} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 128-bit SIS target | $2^{32}-99$ | 24 | 0.442 [0.441, 0.445] | 0.393 [0.378, 0.404] | 0.837 [0.821, 0.849] | 148 [147, 152] |
| 2^{29} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | < 100 bits | $2^{50}-2687$ | 24 | 0.129 [0.129, 0.130] | 0.246 [0.244, 0.249] | 0.533 [0.530, 0.535] | 3.9 [3.9, 4.1] |
| 2^{31} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 26 | 0.536 [0.535, 0.537] | 1.35 [1.35, 1.36] | 1.90 [1.90, 1.91] | 11.5 [11.0, 11.6] |
| 2^{31} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 26 | 0.534 [0.533, 0.534] | 1.86 [1.86, 1.86] | 2.42 [2.42, 2.43] | 9.9 [9.8, 10.5] |
| 2^{31} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 128-bit SIS target | $2^{32}-99$ | 26 | 2.10 [2.10, 2.12] | 1.04 [1.00, 1.10] | 3.15 [3.11, 3.20] | 329 [329, 332] |
| 2^{31} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | < 100 bits | $2^{50}-2687$ | 26 | 0.471 [0.470, 0.477] | 0.668 [0.665, 0.671] | 1.57 [1.56, 1.57] | 4.6 [4.5, 4.6] |
| 2^{33} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 28 | 2.07 [2.07, 2.09] | 2.32 [2.32, 2.33] | 4.43 [4.41, 4.44] | 18.1 [17.8, 18.5] |
| 2^{33} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 28 | 2.22 [2.18, 2.22] | 3.34 [3.33, 3.34] | 5.66 [5.62, 5.67] | 11.4 [11.2, 11.9] |
| 2^{33} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 128-bit SIS target | $2^{32}-99$ | 28 | 11.6 [11.6, 11.7] | 4.60 [4.56, 4.63] | 16.2 [16.2, 16.3] | 715 [708, 722] |
| 2^{33} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | < 100 bits | $2^{50}-2687$ | 28 | 1.83 [1.83, 1.84] | 1.63 [1.58, 1.64] | 4.33 [4.29, 4.34] | 4.3 [4.2, 4.3] |
| 2^{35} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 30 | 8.42 [8.40, 8.46] | 5.69 [5.65, 5.71] | 14.2 [14.1, 14.2] | 32.2 [31.7, 32.5] |
| 2^{35} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128-bit target | $2^{32}-99$ | 30 | 10.7 [10.6, 10.7] | 8.35 [8.33, 8.35] | 19.2 [19.1, 19.3] | 14.1 [13.9, 14.4] |
| 2^{35} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 128-bit SIS target | $2^{32}-99$ | 30 | 54.1 [54.1, 54.8] | 20.1 [19.8, 20.5] | 74.4 [74.0, 74.6] | 1537 [1532, 1545] |
| 2^{35} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | < 100 bits | $2^{50}-2687$ | 30 | 8.90 [8.84, 9.03] | 4.70 [4.64, 4.74] | 15.8 [15.6, 15.9] | 7.2 [7.1, 7.4] |

**(1)** The recursive `fp32-dense` planner produced a schedule for this $n_v$ with no setup-prefix edge, so the offload variant would not offload setup.


| Nominal payload | Scheme | Commitment (B) | Evaluation (B) | Proof (B) | Total sent (B) | Excluded context (B) | Peak RSS (GiB) | Prep. (s) | State (GiB) |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2^{27} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 57714 | 57858 | 214 | 0.108 | 0.0055 | 0.0020 |
| 2^{27} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | —(1) | —(1) | —(1) | —(1) | —(1) | —(1) | —(1) | —(1) |
| 2^{27} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 2048 | 8 | 60066 | 62122 | 0 | 0.330 | unknown | 0.0064 |
| 2^{27} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | 775 | 776 | 109272 | 110823 | 0 | 0.850 | 0.0743 | 0.428 |
| 2^{29} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 58228 | 58372 | 214 | 0.223 | 0.0057 | 0.0020 |
| 2^{29} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 62588 | 62732 | 214 | 0.232 | 0.0138 | 0.0020 |
| 2^{29} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 2304 | 8 | 60630 | 62942 | 0 | 1.08 | unknown | 0.0133 |
| 2^{29} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | 774 | 774 | 109352 | 110900 | 0 | 1.70 | 0.158 | 0.855 |
| 2^{31} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 59487 | 59631 | 214 | 0.627 | 0.0127 | 0.0049 |
| 2^{31} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 62717 | 62861 | 214 | 0.671 | 0.0331 | 0.0039 |
| 2^{31} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 2304 | 8 | 63261 | 65573 | 0 | 3.93 | unknown | 0.0290 |
| 2^{31} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | 776 | 775 | 114781 | 116332 | 0 | 4.47 | 0.427 | 1.83 |
| 2^{33} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 60894 | 61038 | 214 | 1.21 | 0.0259 | 0.0107 |
| 2^{33} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 63298 | 63442 | 214 | 1.33 | 0.108 | 0.0156 |
| 2^{33} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 2304 | 8 | 64897 | 67209 | 0 | 20.7 | unknown | 0.0521 |
| 2^{33} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | 774 | 776 | 114847 | 116397 | 0 | 12.3 | 0.871 | 3.66 |
| 2^{35} | [Akita](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 61003 | 61147 | 214 | 4.37 | 0.0508 | 0.0215 |
| 2^{35} | [Akita (offload)](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612) | 128 | 16 | 63743 | 63887 | 214 | 4.47 | 0.229 | 0.0313 |
| 2^{35} | [Greyhound](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af) | 2560 | 8 | 67212 | 69780 | 0 | 92.6 | unknown | 0.104 |
| 2^{35} | [RoKoKo](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121) | 774 | 774 | 115010 | 116558 | 0 | 36.9 | 2.16 | 8.56 |

**(1)** The recursive `fp32-dense` planner produced a schedule for this $n_v$ with no setup-prefix edge, so the offload variant would not offload setup.


### Measured commits

- Akita [`12d486b0`](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612)
- Akita (offload) [`12d486b0`](https://github.com/LayerZero-Labs/akita/commit/12d486b0bca2814956d480aa121130e2702f8612)
- Greyhound [`672e7410`](https://github.com/LayerZero-Labs/greyhound-reference/commit/672e74100496f6ef698ba35e241cf7593e3d57af)
- RoKoKo [`5caba472`](https://github.com/lattice-arguments/rokoko/commit/5caba472334f7764645ea2c7c5d612353a670121)


### Reproduction template

Machine, ISA, compiler, executable, lockfile, command, and timestamp provenance
for this dataset are recorded with each observation. The infrastructure
toolchain pin is Rust **1.95** (`rust-toolchain.toml`).
Recorded runner command(s): `target/release/pcs-bench lattice-eval run --scheme akita,akita-offload --payload 27,29,31,33,35 --runs 10 --warmups 1 --seed-mode vary --out run-akita-lattice; target/release/pcs-bench lattice-eval run --scheme greyhound --payload 27,29,31,33,35 --runs 10 --warmups 1 --seed-mode vary --out run-greyhound; target/release/pcs-bench lattice-eval run --scheme rokoko --payload 27,29,31,33,35 --runs 10 --warmups 1 --seed-mode vary --out run-rokoko`. The commands below are a template,
not reconstructed provenance.
RoKoKo uses `rustup` **nightly-2026-09-03**. Workers are built before sampling;
every timed execution is then a fresh process wrapped
in `scripts/with-memlimit.sh` with a 168~GiB virtual-address-space ceiling (`ulimit -v`, numerically 90% of host RAM) and `RAYON_NUM_THREADS=1`.
Raw records identify warmup and measured processes separately; warmup rows
are stored with `warmup: true` and excluded from the aggregate. Workload seeds
and the `vary`/`fixed` seed mode are recorded per observation. Greyhound is
`LayerZero-Labs/greyhound-reference`, built with `-march=native -O3 -flto`,
and run with `LATTICE_DOGS_THREADS=1` and `LABRADOR_SIS_SECURITY=l2-quantum128-adps16`.
Proof sizes are contextual wire bytes. Recorded worker flags for this dataset:
`-C target-cpu=native`.
The Akita adapter is built with thin LTO and one codegen unit; RoKoKo keeps
its own release profile (fat LTO, one codegen unit).
`./scripts/fetch-vendors.sh` clones the pinned implementations and patches
RoKoKo so the executor prints commitment, CRS, and peak RSS. Akita embeds
the pinned upstream schedule artifacts and committed supplemental direct rows;
no Akita patches or schedule generation are needed.

Non-interactive shells may not put Cargo on `PATH`; `source ~/.cargo/env`
is required in that case. `CARGO_NET_GIT_FETCH_WITH_CLI=true` avoids libgit2 auth
failures when fetching the pinned git dependencies.
Published numbers live in `results/lattice-x86_64/`.

```bash
# On an AVX-512 Linux x86_64 host
source "$HOME/.cargo/env"   # if cargo is not on PATH
cd /path/to/akita-benchmark

rustup toolchain install nightly-2026-09-03 -c rustc,cargo   # once, for RoKoKo
export CARGO_NET_GIT_FETCH_WITH_CLI=true
export RUSTFLAGS="-C target-cpu=native"
export RAYON_NUM_THREADS=1

./scripts/fetch-vendors.sh          # Greyhound, RoKoKo, and Akita pins
./scripts/build-greyhound.sh

# Full 20-cell matrix (Akita, Akita offload, Greyhound, RoKoKo)
./scripts/lattice-eval.sh run --out results/lattice-x86_64

# Rebuild Markdown + LaTeX from the JSONL already in that directory
cargo run -p pcs-bench-runner --bin pcs-bench -- lattice-eval compare \
  results/lattice-x86_64 --out-dir results/lattice-x86_64
```

**Sanity-check the harness before trusting a full run.** `lattice-eval matrix`
prints the 20-cell plan (Akita/Greyhound `log2 N`, RoKoKo
`p-22`/`p-24`/`p-26`/`p-28`/`p-30`, and the Akita setup-offload row). A single supported cell should verify and emit
JSON with `status: ok`. Unit tests cover the RoKoKo log parser, OOM
classification, and table tokens. Each sample the runner launches is equivalent
to the worker commands below (still under the 90%-of-RAM cap).

```bash
export RUSTFLAGS="-C target-cpu=native"
export RAYON_NUM_THREADS=1

cargo test --workspace --locked
cargo run -p pcs-bench-runner --bin pcs-bench -- lattice-eval matrix
CARGO_TARGET_DIR=target/akita cargo build --release --locked \
  --manifest-path benchmarks/akita/Cargo.toml --bin lattice-eval

# One measured sample of a supported cell (payload 2^31, log2 N = 26)
./scripts/lattice-eval.sh run --scheme akita --payload 31 --runs 1 --warmups 0
./scripts/lattice-eval.sh run --scheme akita-offload --payload 31 --runs 1 --warmups 0
./scripts/lattice-eval.sh run --scheme greyhound --payload 31 --runs 1 --warmups 0
./scripts/lattice-eval.sh run --scheme rokoko --payload 31 --runs 1 --warmups 0

# Direct workers (what each harness sample wraps with with-memlimit.sh)
./scripts/with-memlimit.sh 180055679385 \
  env RAYON_NUM_THREADS=1 AKITA_PARALLEL=0 PCS_BENCH_SEED=1 \
  target/akita/release/lattice-eval \
    --log2-n 26 --payload-log2 31
./scripts/with-memlimit.sh 180055679385 \
  env RAYON_NUM_THREADS=1 AKITA_PARALLEL=0 PCS_BENCH_SEED=1 \
  target/akita/release/lattice-eval \
    --log2-n 26 --payload-log2 31 --offload
./scripts/with-memlimit.sh 180055679385 target/greyhound/lattice-eval --log2-n 26
```

