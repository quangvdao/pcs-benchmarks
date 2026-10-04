Measurements were collected on a single AMD Ryzen 9 9950X 16-Core Processor (Linux x86_64, 32 logical CPUs, 186~GiB RAM). The CPU advertised AVX-512F, but the executed instruction stream was not independently traced. CPU policy: driver amd-pstate-epp, governor powersave, preference balance_performance.

Our second experiment compares Akita with other high-performance hash-based PCSs
on the same nominal dense payload ladder ($2^{27}$ through $2^{35}$ bits).
This is a measured-configuration survey, not an equivalent-security PCS ranking.
Nominal payload is field-capacity accounting, not a claim about sampled input entropy.
The table below records the accepted native profile and security accounting for every scheme.
KoalaBear univariate FRI/STIR pack into a
$2^{23}\times 2^{n-23}$ matrix when $\log_2 N>23$ (two-adicity 24 at rate $1/2$).
Timing cells report the median and, when supported by the sample count, a
conservative distribution-free 95% confidence interval at **1 and 8 threads**. Scheme names link to the exact git commit
that was measured. Unmeasured roster cells are pending.

The timing comparison separates commitment, opening, and verification, while the
cold total includes setup plus commitment and opening. Point-dependent claim and
transcript work supplied to proving is included in opening.
The resources table reports communication, memory (1-thread and 8-thread peak RSS),
and preprocessing. An OOM entry is a confirmed allocation failure under the 168 GiB virtual-address-space ceiling.

### Security and accepted profiles

| Scheme | Accepted profile | Security accounting |
| --- | --- | --- |
| Akita | Planner-selected direct/offloaded schedules at each native prime | 128-bit Module-SIS and 128-bit classical-ROM transcript target |
| Plonky2 FRI | Quartic challenge field, rate 1/2, 169 queries, 16 work bits | 100-bit proven Johnson-regime bound |
| Plonky3 FRI | Upstream new_benchmark_high_arity with 169 queries: rate 1/2, fold up to 8, 16 query-PoW bits, 10-bit batching grind | 100-bit proven Johnson-regime bound |
| Plonky3 STIR | Upstream PCS benchmark with the Johnson bound: rate 1/2, fold 16 throughout, at most 16 work bits per phase, 16-bit batching grind | 100-bit aggregate target, proven Johnson regime |
| Plonky3 WHIR | Upstream PCS benchmark profile with the Johnson bound: octic extension, rate 1/2, fold 4, 12 work bits | 128-bit round-by-round target, proven Johnson regime |
| Binius64 BaseFold | Product default: rate 1/2, 232 queries, SHA-256 | 96-bit unique-decoding query target |
| Flock Ligerito | Default Fast: rate 1/2, Johnson, two OOD checks, BLAKE3 | 128-bit round-by-round target |
| WorldFnd WHIR | CLI defaults: rate 1/2, fold 4, Johnson, BLAKE3 | 128-bit round-by-round target |
| SP1 BaseFold | Product default: rate 1/4, 124 queries, 16 work bits, stacking height 21 | 100-bit unique-decoding query target |

| Nominal payload | Scheme | Security target | Statement | Field | log₂ N | Threads | Commit (s) | Open (s) | Cold total (s) | Verify (ms) |
| ---: | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 22 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 22 | 8 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 22 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 22 | 8 | pending | pending | pending | pending |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 21 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 21 | 8 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 21 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 21 | 8 | pending | pending | pending | pending |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 20 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 20 | 8 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 20 | 1 | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 20 | 8 | pending | pending | pending | pending |
| 2^{27} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 21 | 1 | 3.41 [3.39, 3.41] | 2.79 [2.76, 2.88] | 6.19 [6.17, 6.30] | 14.5 [14.4, 14.7] |
| 2^{27} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 21 | 8 | 0.548 [0.543, 0.558] | 1.20 [1.19, 1.20] | 1.74 [1.73, 1.76] | 14.9 [14.9, 15.1] |
| 2^{27} | [Plonky3 FRI](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate | $2^{31}-2^{24}+1$ | 22 | 1 | 0.982 [0.979, 0.985] | 1.15 [1.15, 1.17] | 2.17 [2.16, 2.18] | 7.1 [7.0, 7.1] |
| 2^{27} | [Plonky3 FRI](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate | $2^{31}-2^{24}+1$ | 22 | 8 | 0.205 [0.204, 0.208] | 0.241 [0.238, 0.244] | 0.473 [0.466, 0.476] | 7.3 [7.2, 7.4] |
| 2^{27} | [Plonky3 STIR](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate | $2^{31}-2^{24}+1$ | 22 | 1 | 0.985 [0.984, 0.989] | 1.63 [1.61, 1.64] | 2.64 [2.62, 2.65] | 5.8 [5.8, 5.9] |
| 2^{27} | [Plonky3 STIR](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate | $2^{31}-2^{24}+1$ | 22 | 8 | 0.210 [0.208, 0.212] | 0.430 [0.423, 0.434] | 0.665 [0.660, 0.669] | 6.0 [5.9, 6.0] |
| 2^{27} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 22 | 1 | 0.078 [0.078, 0.078] | 0.520 [0.515, 0.521] | 0.599 [0.594, 0.600] | 3.4 [3.3, 3.4] |
| 2^{27} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 22 | 8 | 0.021 [0.020, 0.021] | 0.146 [0.144, 0.151] | 0.167 [0.166, 0.172] | 3.5 [3.4, 3.6] |
| 2^{27} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 20 | 1 | pending | pending | pending | pending |
| 2^{27} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 20 | 8 | pending | pending | pending | pending |
| 2^{27} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 27 | 1 | pending | pending | pending | pending |
| 2^{27} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 27 | 8 | pending | pending | pending | pending |
| 2^{27} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 21 | 1 | pending | pending | pending | pending |
| 2^{27} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 21 | 8 | pending | pending | pending | pending |
| 2^{27} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 22 | 1 | pending | pending | pending | pending |
| 2^{27} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 22 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 24 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 24 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 24 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 24 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 23 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 23 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 23 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 23 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 22 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 22 | 8 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 22 | 1 | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 22 | 8 | pending | pending | pending | pending |
| 2^{29} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 23 | 1 | 13.8 [13.8, 13.8] | 11.3 [11.2, 11.3] | 25.1 [25.1, 25.1] | 17.9 [17.9, 18.0] |
| 2^{29} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 23 | 8 | 2.37 [2.35, 2.38] | 4.89 [4.87, 4.91] | 7.25 [7.22, 7.28] | 18.3 [18.3, 18.4] |
| 2^{29} | [Plonky3 FRI(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 24 | 1 | 2.24 [2.24, 2.26] | 2.31 [2.28, 2.33] | 4.62 [4.59, 4.64] | 7.8 [7.8, 7.9] |
| 2^{29} | [Plonky3 FRI(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 24 | 8 | 0.469 [0.466, 0.476] | 0.458 [0.455, 0.464] | 0.988 [0.986, 0.993] | 8.0 [7.8, 8.1] |
| 2^{29} | [Plonky3 STIR(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 24 | 1 | 2.24 [2.23, 2.24] | 3.24 [3.22, 3.26] | 5.54 [5.52, 5.56] | 6.0 [6.0, 6.1] |
| 2^{29} | [Plonky3 STIR(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 24 | 8 | 0.465 [0.465, 0.471] | 0.861 [0.851, 0.871] | 1.39 [1.37, 1.40] | 6.2 [6.2, 6.3] |
| 2^{29} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 24 | 1 | 0.347 [0.346, 0.349] | 2.30 [2.29, 2.30] | 2.65 [2.65, 2.65] | 3.9 [3.9, 3.9] |
| 2^{29} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 24 | 8 | 0.098 [0.096, 0.099] | 0.715 [0.710, 0.728] | 0.817 [0.815, 0.830] | 4.0 [4.0, 4.2] |
| 2^{29} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 22 | 1 | pending | pending | pending | pending |
| 2^{29} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 22 | 8 | pending | pending | pending | pending |
| 2^{29} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 29 | 1 | pending | pending | pending | pending |
| 2^{29} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 29 | 8 | pending | pending | pending | pending |
| 2^{29} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 23 | 1 | pending | pending | pending | pending |
| 2^{29} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 23 | 8 | pending | pending | pending | pending |
| 2^{29} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 24 | 1 | pending | pending | pending | pending |
| 2^{29} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 24 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 26 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 26 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 25 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 25 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 25 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 25 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 24 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 24 | 8 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 24 | 1 | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 24 | 8 | pending | pending | pending | pending |
| 2^{31} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 25 | 1 | pending | pending | pending | pending |
| 2^{31} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 25 | 8 | pending | pending | pending | pending |
| 2^{31} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 26 | 8 | pending | pending | pending | pending |
| 2^{31} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 26 | 8 | pending | pending | pending | pending |
| 2^{31} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 26 | 8 | pending | pending | pending | pending |
| 2^{31} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 24 | 1 | pending | pending | pending | pending |
| 2^{31} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 24 | 8 | pending | pending | pending | pending |
| 2^{31} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 31 | 1 | pending | pending | pending | pending |
| 2^{31} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 31 | 8 | pending | pending | pending | pending |
| 2^{31} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 25 | 1 | pending | pending | pending | pending |
| 2^{31} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 25 | 8 | pending | pending | pending | pending |
| 2^{31} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 26 | 1 | pending | pending | pending | pending |
| 2^{31} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 26 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 28 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 28 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 27 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 27 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 27 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 27 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 26 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 26 | 8 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 26 | 1 | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 26 | 8 | pending | pending | pending | pending |
| 2^{33} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 27 | 1 | pending | pending | pending | pending |
| 2^{33} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 27 | 8 | pending | pending | pending | pending |
| 2^{33} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 28 | 8 | pending | pending | pending | pending |
| 2^{33} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 28 | 8 | pending | pending | pending | pending |
| 2^{33} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 28 | 8 | pending | pending | pending | pending |
| 2^{33} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 26 | 1 | pending | pending | pending | pending |
| 2^{33} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 26 | 8 | pending | pending | pending | pending |
| 2^{33} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 33 | 1 | pending | pending | pending | pending |
| 2^{33} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 33 | 8 | pending | pending | pending | pending |
| 2^{33} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 27 | 1 | pending | pending | pending | pending |
| 2^{33} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 27 | 8 | pending | pending | pending | pending |
| 2^{33} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 28 | 1 | pending | pending | pending | pending |
| 2^{33} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 28 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 30 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{32}-99$ | 30 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 29 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 29 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 29 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{64}-59$ | 29 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 28 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 28 | 8 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 28 | 1 | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | 128-bit Module-SIS/ROM | multilinear | $2^{128}-2^{32}+22537$ | 28 | 8 | pending | pending | pending | pending |
| 2^{35} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 29 | 1 | pending | pending | pending | pending |
| 2^{35} | Plonky2 FRI | 100-bit Johnson | univariate | $2^{64}-2^{32}+1$ | 29 | 8 | pending | pending | pending | pending |
| 2^{35} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | Plonky3 FRI(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 30 | 8 | pending | pending | pending | pending |
| 2^{35} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | Plonky3 STIR(1) | 100-bit Johnson | univariate batch | $2^{31}-2^{24}+1$ | 30 | 8 | pending | pending | pending | pending |
| 2^{35} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | WHIR (Plonky3) | 128-bit RBR | multilinear | $2^{31}-2^{24}+1$ | 30 | 8 | pending | pending | pending | pending |
| 2^{35} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 28 | 1 | pending | pending | pending | pending |
| 2^{35} | Binius64 BaseFold | 96-bit UDR query | multilinear | $F_{2^{128}}$ | 28 | 8 | pending | pending | pending | pending |
| 2^{35} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 35 | 1 | pending | pending | pending | pending |
| 2^{35} | Flock Ligerito | 128-bit RBR | packed F128 MLE | $F_2$ | 35 | 8 | pending | pending | pending | pending |
| 2^{35} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 29 | 1 | pending | pending | pending | pending |
| 2^{35} | WHIR (WorldFnd) | 128-bit RBR | multilinear | $2^{64}-2^{32}+1$ | 29 | 8 | pending | pending | pending | pending |
| 2^{35} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 30 | 1 | pending | pending | pending | pending |
| 2^{35} | BaseFold (SP1) | 100-bit UDR query | multilinear | $2^{31}-2^{24}+1$ | 30 | 8 | pending | pending | pending | pending |

**(1)** KoalaBear two-adicity is 24, so a rate-$1/2$ univariate cannot be a single degree-$2^{n}$ polynomial when $\log_2 N>23$. The worker packs the $2^{n}$ coefficients into a trace matrix of height $2^{23}$ and width $2^{n-23}$. That is batched univariate FRI/STIR, not one tall polynomial.


| Nominal payload | Scheme | Commitment (B) | Evaluation (B) | Proof (B) | Total sent (B) | Excluded context (B) | Peak RSS 1-thread (GiB) | Peak RSS 8-thread (GiB) | Prep. (s) | State (GiB) |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 2^{27} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 520 | 32 | 637224 | 637776 | 0 | 0.958 | 0.957 | 0.0000 | 0.0000 |
| 2^{27} | [Plonky3 FRI](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 32 | 20 | 410542 | 410594 | 0 | 1.18 | 1.17 | 0.0285 | unknown |
| 2^{27} | [Plonky3 STIR](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 34 | 20 | 290546 | 290600 | 0 | 1.18 | 1.17 | 0.0289 | unknown |
| 2^{27} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 32 | 0 | 211378 | 211410 | 0 | 0.235 | 0.240 | 0.0013 | unknown |
| 2^{27} | Binius64 BaseFold | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | Flock Ligerito | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | WHIR (WorldFnd) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{27} | BaseFold (SP1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | [Plonky2 FRI](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa) | 520 | 32 | 780288 | 780840 | 0 | 3.82 | 3.82 | 0.0000 | 0.0000 |
| 2^{29} | [Plonky3 FRI(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 32 | 40 | 448972 | 449044 | 0 | 2.41 | 2.41 | 0.0638 | unknown |
| 2^{29} | [Plonky3 STIR(1)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 34 | 40 | 307422 | 307496 | 0 | 2.41 | 2.41 | 0.0645 | unknown |
| 2^{29} | [WHIR (Plonky3)](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d) | 32 | 0 | 252018 | 252050 | 0 | 0.928 | 0.936 | 0.0056 | unknown |
| 2^{29} | Binius64 BaseFold | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | Flock Ligerito | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | WHIR (WorldFnd) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{29} | BaseFold (SP1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Plonky2 FRI | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Plonky3 FRI(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Plonky3 STIR(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | WHIR (Plonky3) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Binius64 BaseFold | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | Flock Ligerito | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | WHIR (WorldFnd) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{31} | BaseFold (SP1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Plonky2 FRI | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Plonky3 FRI(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Plonky3 STIR(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | WHIR (Plonky3) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Binius64 BaseFold | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | Flock Ligerito | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | WHIR (WorldFnd) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{33} | BaseFold (SP1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Akita (offload) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Plonky2 FRI | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Plonky3 FRI(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Plonky3 STIR(1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | WHIR (Plonky3) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Binius64 BaseFold | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | Flock Ligerito | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | WHIR (WorldFnd) | pending | pending | pending | pending | pending | pending | pending | pending | pending |
| 2^{35} | BaseFold (SP1) | pending | pending | pending | pending | pending | pending | pending | pending | pending |

**(1)** KoalaBear two-adicity is 24, so a rate-$1/2$ univariate cannot be a single degree-$2^{n}$ polynomial when $\log_2 N>23$. The worker packs the $2^{n}$ coefficients into a trace matrix of height $2^{23}$ and width $2^{n-23}$. That is batched univariate FRI/STIR, not one tall polynomial.


### Measured commits

- Plonky2 FRI [`e1c2d354`](https://github.com/elliottech/plonky2/commit/e1c2d35450948b88fca6a7e69e2643c3ecad3caa)
- Plonky3 FRI [`3acc8b70`](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d)
- Plonky3 STIR [`3acc8b70`](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d)
- WHIR (Plonky3) [`3acc8b70`](https://github.com/Plonky3/Plonky3/commit/3acc8b70e68d6c2afc03930700c26540bd47458d)


### Reproduction template

Machine, ISA, compiler, executable, lockfile, command, and timestamp provenance
for this dataset are recorded with each observation. The infrastructure
toolchain pin is Rust **1.95** (`rust-toolchain.toml`).
Recorded runner command: `target/release/pcs-bench hash-eval run --scheme plonky2-fri,plonky3-fri,plonky3-stir,whir --payload 27,29,31,33,35 --threads 1,8 --runs 10 --warmups 1 --seed-mode vary --out run-hash-proven`. The commands below are a template,
not reconstructed provenance.
Workers are built from checked-in lockfiles before sampling. Every timed
execution is a fresh process wrapped in `scripts/with-memlimit.sh` with
a 168~GiB virtual-address-space ceiling (`ulimit -v`, numerically
90% of host RAM). Raw records identify
warmup and measured processes separately; warmup rows are stored with
`warmup: true` and excluded from the aggregate. Workload seeds and the
`vary`/`fixed` seed mode are recorded per observation. Recorded worker flags
for this dataset: `-C target-cpu=native`. Every adapter is built with thin LTO and one
codegen unit. Isolated Cargo trees under `benchmarks/`
fetch the pinned git revisions (Plonky3, SP1, plonky2, Binius64, Flock,
WorldFnd/WHIR) so they do not unify with the lattice workspace. Cargo fetches
those revisions on first build.

Non-interactive shells may not put Cargo on `PATH`; `source ~/.cargo/env`
is required in that case. `CARGO_NET_GIT_FETCH_WITH_CLI=true` avoids libgit2 auth
failures when fetching the pinned git dependencies.
Published numbers live in `results/hash-x86_64/`.

```bash
# On an AVX-512 Linux x86_64 host
source "$HOME/.cargo/env"   # if cargo is not on PATH
cd /path/to/akita-benchmark

export CARGO_NET_GIT_FETCH_WITH_CLI=true
export RUSTFLAGS="-C target-cpu=native"

./scripts/fetch-vendors.sh --akita   # Akita pin + nv=22/24 + fp64/fp128 + offload catalogs

# Full 140-cell matrix (14 schemes × 5 payloads × {1,8} threads)
./scripts/hash-eval.sh run --out results/hash-x86_64

# Rebuild Markdown + LaTeX from the JSONL already in that directory
cargo run -p pcs-bench-runner --bin pcs-bench -- hash-eval compare \
  results/hash-x86_64 --out-dir results/hash-x86_64
```

**Sanity-check the harness before trusting a full run.** `hash-eval matrix`
prints the 140-cell plan. A single supported cell should verify and emit JSON
with `status: ok`. Each sample the runner launches is equivalent to the worker
commands below (still under the 90%-of-RAM cap).

```bash
export RUSTFLAGS="-C target-cpu=native"

cargo test -p pcs-bench-core -p pcs-bench-runner --locked
cargo run -p pcs-bench-runner --bin pcs-bench -- hash-eval matrix

# One measured sample of a supported cell (payload 2^31, log2 N = 26, 1 thread)
./scripts/hash-eval.sh run --scheme akita --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme akita-fp64 --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme akita-fp128 --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme akita-offload --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme akita-fp64-offload --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme whir --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme basefold --payload 31 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme plonky2-fri --payload 27 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme plonky3-fri --payload 27 --threads 1 --runs 1 --warmups 0
./scripts/hash-eval.sh run --scheme flock --payload 27 --threads 1 --runs 1 --warmups 0
```

