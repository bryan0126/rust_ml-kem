# ML-KEM 구현 및 검증

FIPS 203 ML-KEM(512/768/1024)을 네 가지 방식으로 준비하고, 같은 시험 벡터로
교차검증하며 성능을 비교한다.

## 네 구현

| 표기 | 위치 | 설명 |
|---|---|---|
| C 원본 | **저장소에 없음** (외부 liboqs) | mlkem-native 참조 구현. 무수정 |
| C2Rust | `crates/ml-kem-c2rust/` | 위 C 코드를 C2Rust 로 기계 변환한 결과 |
| Rust向 | `crates/ml-kem/` | 기계 변환 결과를 사람이 읽고 쓰는 Rust 로 재작성 |
| ml-kem | (crates.io) | RustCrypto `ml-kem`. 비교 기준선 |

Rust向 구현은 `unsafe` 를 쓰지 않으며(`forbid(unsafe_code)`), 계산 순서와 최적화
(곱셈 사전계산, 축약 지연, 값 배리어)를 C 원본과 같게 맞추었다.

## C 원본에 대하여

이 저장소는 C 참조 구현을 담지 않는다. liboqs 를 그대로 쓰는 것이 요점이므로
사본을 두지 않고, 검증할 때 사용자가 빌드한 liboqs 의 산출물을 링크한다.
저장소에 들어 있는 C 코드는 `c/ml-kem/kat_harness.c` 하나로, 이는 구현이 아니라
liboqs 를 호출하는 시험용 실행기다. 다른 세 구현의 실행기와 같은 형식으로 결과를
출력하므로 네 구현을 같은 기준으로 비교할 수 있다.

liboqs 가 없어도 Rust 세 구현은 그대로 검증·측정된다. C 원본만 비교에서 제외되고,
실행할 때 그 사실을 알려준다.

## 사전 준비

```bash
git clone https://github.com/open-quantum-safe/liboqs ~/liboqs
cmake -S ~/liboqs -B ~/liboqs/build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
cmake --build ~/liboqs/build -j
```

`-DCMAKE_EXPORT_COMPILE_COMMANDS=ON` 이 필요하다. 시험용 실행기가 ML-KEM 참조
구현과 똑같은 컴파일 옵션으로 빌드되어야 하고, 그 옵션을 이 파일에서 읽는다.

검증에 사용한 liboqs 는 `0.16.0-6-g1904ace16` (2026-07-21) 이다. CMake 가 만드는
디렉터리 이름에 의존해 오브젝트를 찾으므로, 버전이 크게 다르면 경로를 못 찾을 수 있다.

## 한 번에 실행

```bash
LIBOQS_DIR=~/liboqs make ml-kem
```

빌드 → KAT 검증 → 성능 측정을 순서대로 수행한다. 단계별로 하려면
`make ml-kem-build`, `make ml-kem-kat`, `make ml-kem-bench` 를 쓴다.
`LIBOQS_DIR` 을 생략하면 `~/liboqs` 를 찾는다.

## 검증 (KAT) — 개별 실행

NIST ACVP 벡터로 네 구현 x 파라미터 3종 x 항목 6가지를 대조한다.

```bash
cd tests/ml-kem
python3 kat.py fetch    # 벡터가 없을 때만
python3 kat.py run
```

검증 항목

1. `KeyGen_internal` — `(d, z)` 에서 `(ek, dk)` 생성
2. `Encaps_internal` — `(ek, m)` 에서 `(c, K)` 생성
3. `Decaps` 정상 — 유효한 암호문 복호
4. `Decaps` 변조 — 변조된 암호문. 오류가 아니라 규격이 정한 의사난수 키를 반환해야 한다
5. `ek` 유효성 검사 — FIPS 203 7.2 (길이 검사 + 모듈러스 검사)
6. `dk` 유효성 검사 — FIPS 203 7.3 (길이 검사 + `H(ek)` 해시 검사)

## 성능 측정 — 개별 실행

```bash
cd benches/ml-kem
python3 bench.py --iters 3000
```

네 구현의 출력을 같은 식으로 접어 대조값을 만들고, 그 값이 모두 같을 때만 결과를
인정한다. 값이 다르면 서로 다른 계산을 한 것이므로 경고와 함께 실패로 끝난다.

반복 횟수가 작으면 편차가 커진다. 300회에서는 ±29%p 까지 흔들렸고 3000회에서
±3% 수준으로 안정되었다.

## 변환 도구

C2Rust 변환을 처음부터 다시 하려면 `tools/ml-kem/` 을 쓴다.

```bash
cd tools/ml-kem
for v in 512 768 1024; do
  LIBOQS_DIR=~/liboqs python3 filter_cc.py $v ref
  mkdir -p out/$v
  c2rust transpile cdb/$v/compile_commands.json --emit-build-files -o out/$v
done
python3 setup_crates.py
```

`config/c2rust_config.h` 는 인라인 어셈블리를 쓰지 않는 경로를 선택하게 한다.
C2Rust 가 어셈블리를 Rust 로 옮기지 못하기 때문이며, liboqs 원본 소스는 수정하지 않는다.
