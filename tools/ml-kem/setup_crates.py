#!/usr/bin/env python3
"""
c2rust 가 생성한 rust_out/{512,768,1024} 크레이트를 빌드 가능한 상태로 패치하고
전체를 하나의 Cargo 워크스페이스로 묶는다. 여러 번 실행해도 안전(idempotent).

수행 내용
  1) 패키지/라이브러리 이름  "768" -> "mlkem768_c2rust"
     (Rust 크레이트 이름은 숫자로 시작할 수 없음)
  2) lib.rs 의 `#![feature(asm)]` 제거
     (asm! 은 Rust 1.59 에서 안정화되어 feature gate 가 삭제됨 -> 남아 있으면 컴파일 에러)
     `#![feature(raw_ref_op)]` 는 &raw 연산자를 실제로 사용하므로 유지(nightly 필요)
  3) shim 크레이트(oqs_shim) 를 의존성으로 추가하고 `extern crate oqs_shim;` 삽입
     -> OQS_SHA3_* / OQS_randombytes 심볼이 링크된다
  4) 각 크레이트의 `[workspace]` 블록 제거 후, 상위에 워크스페이스 루트 생성

사용법:
    python3 setup_crates.py            # 존재하는 변형 모두 패치
"""
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
VARIANTS = ["512", "768", "1024"]


def patch_cargo_toml(path, variant):
    name = f"mlkem{variant}_c2rust"
    with open(path) as f:
        txt = f.read()

    # 1) [workspace] members = [ ... ] 블록 제거 (루트 워크스페이스로 통합)
    txt = re.sub(r"\[workspace\]\s*\nmembers\s*=\s*\[[^\]]*\]\s*\n", "", txt)

    # 2) 패키지 이름 / lib 이름 교체
    txt = re.sub(r'^name\s*=\s*"[^"]*"', f'name = "{name}"', txt, flags=re.M)

    # 3) shim 의존성 추가
    if "oqs_shim" not in txt:
        if re.search(r"^\[dependencies\]", txt, flags=re.M):
            txt = re.sub(
                r"^\[dependencies\]\s*$",
                '[dependencies]\noqs_shim = { path = "../../shim" }',
                txt,
                count=1,
                flags=re.M,
            )
        else:
            txt += '\n[dependencies]\noqs_shim = { path = "../../shim" }\n'

    with open(path, "w") as f:
        f.write(txt)
    return name


def patch_lib_rs(path):
    with open(path) as f:
        lines = f.readlines()

    # nightly 전용 feature gate 제거 -> stable 툴체인으로 빌드 가능해진다.
    #   feature(asm)        : asm! 은 Rust 1.59 에서 안정화 (gate 삭제됨)
    #   feature(raw_ref_op) : &raw const / &raw mut 는 Rust 1.82 에서 안정화
    # 이렇게 하면 transpile 결과물과 ml-kem crate(최신 rustc 요구)를
    # 같은 워크스페이스/툴체인에서 함께 빌드할 수 있다.
    drop = {"#![feature(asm)]", "#![feature(raw_ref_op)]"}
    out = [ln for ln in lines if ln.strip() not in drop]

    txt = "".join(out)

    # shim 을 강제로 링크시킨다 (#[no_mangle] 심볼 제공)
    if "extern crate oqs_shim" not in txt:
        # 마지막 #![...] 속성 뒤에 삽입
        idx = 0
        for m in re.finditer(r"^#!\[.*\]\s*$", txt, flags=re.M):
            idx = m.end()
        txt = txt[:idx] + "\n\nextern crate oqs_shim;\n" + txt[idx:]

    with open(path, "w") as f:
        f.write(txt)


def main():
    patched = []
    for v in VARIANTS:
        crate = os.path.join(HERE, "rust_out", v)
        cargo = os.path.join(crate, "Cargo.toml")
        lib = os.path.join(crate, "lib.rs")
        if not (os.path.isfile(cargo) and os.path.isfile(lib)):
            print(f"[skip] rust_out/{v} 없음 (아직 transpile 안 함)")
            continue
        name = patch_cargo_toml(cargo, v)
        patch_lib_rs(lib)
        patched.append((v, name))
        print(f"[ok] rust_out/{v} -> 크레이트 이름 '{name}', shim 연결, asm feature 제거")

        # c2rust 가 심어놓은 nightly 핀(rust-toolchain.toml)을 제거한다.
        # feature gate 를 없앴으므로 최신 stable 로 빌드된다(검증: rustc 1.97).
        tc = os.path.join(crate, "rust-toolchain.toml")
        if os.path.isfile(tc):
            os.remove(tc)
            print("      rust-toolchain.toml(nightly 핀) 제거 -> stable 사용")

    if not patched:
        print("\n패치할 크레이트가 없습니다. 먼저 transpile 하세요.")
        return

    # 워크스페이스 루트 생성
    members = ['    "shim",'] + [f'    "rust_out/{v}",' for v, _ in patched]
    root = "[workspace]\nresolver = \"2\"\nmembers = [\n" + "\n".join(members) + "\n]\n"
    with open(os.path.join(HERE, "Cargo.toml"), "w") as f:
        f.write(root)
    print(f"\n[ok] 워크스페이스 루트 생성: {os.path.join(HERE, 'Cargo.toml')}")
    print("     members:", ", ".join(["shim"] + [f"rust_out/{v}" for v, _ in patched]))
    print("\n다음: cargo build")


if __name__ == "__main__":
    main()
