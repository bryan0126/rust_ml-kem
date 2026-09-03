# rust_ml-kem

Converting the C implementation of **ML-KEM** (FIPS 203) to Rust, and verifying
that the result is both memory-safe and byte-for-byte correct.

The goal is not just to make the code compile as Rust. C2Rust produces a
transliteration built on raw pointers, which leaves `unsafe` on nearly every
function and forfeits the guarantees that motivated the port in the first place.
This repository therefore carries the work one step further: the machine output
is rewritten as idiomatic Rust with **no `unsafe` at all**, and the rewrite is
held to bit-exact equivalence with the original C code.

## Status

- **Correctness** — 4 implementations × 3 parameter sets × 6 checks,
  **240 cases, all passing** against NIST ACVP vectors.
- **Safety** — the idiomatic implementation declares `forbid(unsafe_code)`;
  the compiler rejects any future change that reintroduces it.
- **Performance** — all four implementations benchmarked under identical
  conditions, with the source of the gap identified.

## Platform requirements

> [!IMPORTANT]
> **The verification and benchmark pipeline requires Linux or macOS.**
> On Windows, run it under **WSL** — that is the environment it was developed
> and measured in. Native Windows is not supported.

The Rust code itself carries no platform-specific dependencies: `sha3` and
`rand` are the only crates it pulls in, and there is no direct use of OS
facilities. The tooling around it is another matter.

| | Native Windows |
| --- | --- |
| Building the Rust crates (`cargo build`) | expected to work, but untested |
| KAT and benchmark scripts | **no** — harness paths are built without the `.exe` suffix |
| Comparison against the C original | **no** — an MSVC build of liboqs does not emit `compile_commands.json` |
| Re-running the C2Rust transpilation | **no** — C2Rust targets Linux and macOS |

The scripts also assume a POSIX environment in smaller ways: GNU `make`, `rm`
in the `clean` target, and `python3` rather than `python` or `py`.

## The four implementations

| Label | Location | Description |
| --- | --- | --- |
| C original | **not in this repo** (external liboqs) | mlkem-native reference implementation, unmodified |
| C2Rust | `crates/ml-kem-c2rust/` | machine translation of the above, plus a hash/RNG shim |
| Idiomatic Rust | `crates/ml-kem/` (package `mlkem_rs`) | hand-rewritten from the C2Rust output, no `unsafe` |
| Baseline | crates.io | RustCrypto [`ml-kem`](https://crates.io/crates/ml-kem) 0.3 |

Comparing against a pure-Rust baseline turned out to require some care. rustls
was the obvious first candidate, but it delegates cryptography to `aws-lc-rs`,
which calls into the C library `aws-lc` — whose ML-KEM is C code from the same
mlkem-native lineage as our conversion target. Benchmarking against it would
have compared a Rust port of C against a Rust wrapper around C, so RustCrypto
`ml-kem` was used instead.

### On the C original

This repository does **not** vendor a copy of the C reference implementation.
Using liboqs exactly as published is the whole point, so the verification links
against objects from a liboqs tree that you build yourself. The only C file here
is `c/ml-kem/kat_harness.c`, which is a test driver rather than an
implementation: it prints results in the same format as the other three
harnesses so that all four can be scored by one script.

The Rust implementations can be verified and benchmarked without liboqs
present — only the C original drops out of the comparison, and the tooling says
so when it does.

## Quick start

Run these on Linux or macOS — under WSL if you are on Windows.

```bash
git clone https://github.com/open-quantum-safe/liboqs ~/liboqs
cmake -S ~/liboqs -B ~/liboqs/build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
cmake --build ~/liboqs/build -j

LIBOQS_DIR=~/liboqs make ml-kem
```

`-DCMAKE_EXPORT_COMPILE_COMMANDS=ON` is required: the harness must be built with
the same compiler flags as the ML-KEM reference implementation, and those flags
are read from the file CMake generates.

Verified against liboqs `0.16.0-6-g1904ace16` (2026-07-21). Object paths depend
on directory names produced by CMake, so a very different version may not
resolve.

`make ml-kem` runs build → KAT → benchmark in order. Individual stages are
`make ml-kem-build`, `make ml-kem-kat`, `make ml-kem-bench`.

## Verification

```bash
cd tests/ml-kem
python3 kat.py fetch    # only if the vectors are missing
python3 kat.py run
```

The checks target the deterministic internal functions defined by FIPS 203.
Because these take their randomness as input rather than generating it, the same
input must always yield the same output — so any divergence between the four
implementations shows up immediately, down to the byte.

1. `KeyGen_internal` — `(d, z)` → `(ek, dk)`
2. `Encaps_internal` — `(ek, m)` → `(c, K)`
3. `Decaps`, valid ciphertext
4. `Decaps`, modified ciphertext — must return the pseudorandom key the standard
   prescribes, **not** an error (implicit rejection)
5. `ek` validity — FIPS 203 §7.2 (length check, then modulus check)
6. `dk` validity — FIPS 203 §7.3 (length check, then `H(ek)` hash check)

Items 5 and 6 exist because passing only well-formed input says nothing about
whether malformed keys are rejected. Note that the length check has to come
first: the malformed `ek` supplied by ACVP is longer than the specification
allows, so an implementation that skips straight to the content check reads the
wrong bytes and wrongly accepts it.

## Benchmarks

```bash
cd benches/ml-kem
python3 bench.py --iters 3000
```

Before any timing is reported, the outputs of all four implementations are folded
into a checksum and compared; if the values disagree the run fails, since they
would no longer be measuring the same computation.

Representative figures for ML-KEM-768, in microseconds, with the C original
taken as 100%:

| Operation | C original | C2Rust | Idiomatic Rust | RustCrypto |
| --- | --- | --- | --- | --- |
| `KeyGen_internal` | **19.22** | 30.78 (160%) | 31.60 (164%) | 34.96 (182%) |
| `Encaps_internal` | **20.53** | 31.40 (153%) | 35.11 (171%) | 37.32 (182%) |
| `Decaps` | **24.87** | 35.53 (143%) | 42.11 (169%) | 46.39 (187%) |

The gap is not a property of the language. ML-KEM spends much of its time in
SHAKE, and the liboqs C build computes four SHAKE instances at once; the Rust
hash crate used here has no equivalent, so it computes them one at a time. A
Rust hash implementation offering the same batched operation should narrow the
difference considerably.

Iteration count matters: at 300 iterations results swung by as much as ±29
percentage points, settling to roughly ±3% at 3000.

These figures are wall-clock times from a single Linux machine. Absolute values
will differ elsewhere, so read them as relative comparisons between the four
implementations rather than as absolute costs.

## Transpilation

To redo the C2Rust conversion from scratch:

```bash
cd tools/ml-kem
for v in 512 768 1024; do
  LIBOQS_DIR=~/liboqs python3 filter_cc.py $v ref
  mkdir -p out/$v
  c2rust transpile cdb/$v/compile_commands.json --emit-build-files -o out/$v
done
python3 setup_crates.py
```

`filter_cc.py` narrows the compilation database to the eight ML-KEM reference
files, and `config/c2rust_config.h` selects the assembly-free code path —
C2Rust cannot carry liboqs' inline assembly across, because Rust's rules for it
are stricter than C's. **The liboqs sources are never modified.**

`setup_crates.py` then fixes up names, drops the nightly feature markers, and
wires in the shim that fills the 13 hash and RNG functions left undefined by
translating only the ML-KEM core.

## Layout

```
.
├── crates/
│   ├── ml-kem/                 # idiomatic Rust implementation (mlkem_rs)
│   └── ml-kem-c2rust/
│       ├── 512/ 768/ 1024/     # C2Rust output, one crate per parameter set
│       └── shim/               # hash (SHA3/SHAKE) and RNG bindings
├── c/ml-kem/                   # liboqs KAT harness (a driver, not an implementation)
├── tests/ml-kem/               # kat.py, per-implementation harnesses, vectors
├── benches/ml-kem/             # bench.py and shared measurement code
├── tools/ml-kem/               # C2Rust pipeline (filter_cc.py, setup_crates.py)
├── docs/ml-kem/                # detailed notes — start here
└── Makefile
```

`docs/ml-kem/README.md` covers the method and results in more depth.

The workspace is laid out to accommodate further PQC algorithms (ML-DSA,
SLH-DSA, SMAUG-T, NTRU+, HAETAE, AIMer), but **only ML-KEM is implemented at
present**; the remaining directories are placeholders.

## Attribution and license

- `crates/ml-kem-c2rust/` is derived from [liboqs](https://github.com/open-quantum-safe/liboqs),
  which is distributed under the MIT license. Its ML-KEM reference code comes
  from [mlkem-native](https://github.com/pq-code-package/mlkem-native)
  (Apache-2.0 / MIT).
- `crates/ml-kem/` was written for this project, guided by FIPS 203 and by the
  structure of the C2Rust output.
- Test vectors are the public ACVP vectors published by NIST.
