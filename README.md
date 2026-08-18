# fdl-rust-pqc

A unified Rust library for post-quantum cryptography (PQC), covering seven algorithms from the NIST PQC and KpqC standardization efforts.

## Overview

|             |                                                             |
| ----------- | ----------------------------------------------------------- |
| **Project** | Post-Quantum Cryptography Rust Library Development          |
| **Goal**    | A single integrated Rust library providing 7 PQC algorithms |
| **Period**  | August 1, 2026 – February 28, 2027 (7 months)               |
| **Team**    | 2 members                                                   |

## Status

- **ML-KEM** — four implementations cross-verified against NIST ACVP vectors
  (240 cases, all passing) and benchmarked. See
  [`docs/ml-kem/README.md`](docs/ml-kem/README.md).
  Quick start: `LIBOQS_DIR=~/liboqs make ml-kem`

## Algorithms

**NIST PQC (3)**

- **ML-KEM** — key encapsulation mechanism (FIPS 203)
- **ML-DSA** — digital signature algorithm (FIPS 204)
- **SLH-DSA** — stateless hash-based signature algorithm (FIPS 205)

**KpqC (4)**

- **SMAUG-T** — lattice-based KEM
- **NTRU+** — lattice-based KEM
- **HAETAE** — lattice-based signature
- **AIMer** — MPC-in-the-Head signature

## Deliverables

1. C source code
2. Rust source code
3. Manual / documentation
4. Cryptographic operation library — binaries for **Linux** and **Windows**

## Approach

1. **Baseline C code.** Start from the latest publicly available C implementations of the seven algorithms as of August 1, 2026, derived from prior research results.
2. **Idiomatic Rust port.** Reimplement each algorithm as a Rust module that takes advantage of the language's safety and type-system guarantees rather than transliterating C.
3. **Parameter sets as independent ciphers.** Each PQC parameter set, classified by security strength, is treated as a separate, independent cipher.
4. **Shared primitives.** The random number generator and hash functions are factored out into common modules used by every algorithm.
5. **Correctness verification.** Validate the Rust modules against the reference C implementations and official test vectors.

## Repository Layout

```
fdl-rust-pqc/
├── c/                  # Baseline C reference implementations (latest public code)
│   ├── ml-kem/
│   ├── ml-dsa/
│   ├── slh-dsa/
│   ├── smaug-t/
│   ├── ntru-plus/
│   ├── haetae/
│   └── aimer/
├── crates/             # Rust workspace members
│   ├── pqc/            # Umbrella crate re-exporting all seven algorithms
│   ├── pqc-common/     # Shared primitives: RNG, hash functions, common traits
│   ├── ml-kem/
│   ├── ml-dsa/
│   ├── slh-dsa/
│   ├── smaug-t/
│   ├── ntru-plus/
│   ├── haetae/
│   └── aimer/
├── tests/              # Cross-crate integration tests
│   └── kat/            # Known-answer test vectors (.req / .rsp)
├── benches/            # Performance benchmarks
├── examples/           # Usage examples per algorithm
└── docs/               # Manual and design notes
```

Each algorithm crate exposes its parameter sets as independent ciphers, and every crate depends on `pqc-common` for randomness and hashing.
