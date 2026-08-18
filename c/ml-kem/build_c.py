#!/usr/bin/env python3
"""
impl #2 (liboqs 원본 C) 하네스 빌드.

핵심: liboqs 가 이미 빌드해 둔 오브젝트 파일과 "원본 컴파일 플래그"를 그대로 재사용한다.
      -> 우리가 손댄 것 없는 순정 liboqs C 를 검증 대상으로 삼는다.
         (C2Rust 용으로 만든 c2rust_config.h 는 여기서 쓰지 않는다)

링크 구성:
  kat_harness.c                                  <- 우리 하네스
  build/.../ml_kem_<v>_ref.dir/**.o               <- ML-KEM ref 구현 (원본)
  build/src/common/CMakeFiles/common.dir/**.o     <- SHA3(FIPS202), AES, RNG 등

사용법:
    python3 build_c.py            # 512/768/1024 전부
    python3 build_c.py 768
결과: harness_c/kat_harness_<variant>
"""
import json
import os
import shlex
import subprocess
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))          # c/ml-kem

# liboqs 는 이 저장소 밖에 있다. 빌드가 끝난 liboqs 트리의 경로를
# 환경변수 LIBOQS_DIR 로 알려준다. 지정하지 않으면 ~/liboqs 를 쓴다.
LIBOQS = os.environ.get("LIBOQS_DIR") or os.path.expanduser("~/liboqs")
BUILD = os.path.join(LIBOQS, "build")
if not os.path.isdir(BUILD):
    sys.exit(f"!! liboqs 빌드 디렉터리가 없습니다: {BUILD}\n"
             f"   LIBOQS_DIR 환경변수로 경로를 지정하세요.")

VARIANTS = sys.argv[1:] or ["512", "768", "1024"]


def ref_flags(variant):
    """해당 variant ref 소스의 원본 컴파일 플래그를 compile_commands.json 에서 추출."""
    with open(os.path.join(BUILD, "compile_commands.json")) as f:
        entries = json.load(f)
    needle = f"mlkem-native_ml-kem-{variant}_ref"
    hit = next((e for e in entries
                if needle in e["file"] and e["file"].endswith("kem.c")), None)
    if hit is None:
        sys.exit(f"!! {needle} 의 컴파일 명령을 찾을 수 없습니다.")

    argv = shlex.split(hit["command"])
    cc = argv[0]
    flags, skip = [], False
    for a in argv[1:]:
        if skip:
            skip = False
            continue
        if a == "-o":
            skip = True          # 출력 경로 제거
            continue
        if a == "-c" or a.endswith(".c"):
            continue             # 컴파일 대상 제거
        flags.append(a)
    return cc, flags, hit["directory"]


def build(variant):
    cc, flags, workdir = ref_flags(variant)

    ref_objs = glob.glob(os.path.join(
        BUILD, "src", "kem", "ml_kem", "CMakeFiles",
        f"ml_kem_{variant}_ref.dir", "**", "*.o"), recursive=True)

    # common.dir : SHA3 디스패처, FIPS202 shim, AES, RNG 등
    # internal.dir 은 common.dir 과 같은 소스를 중복 컴파일한 것이므로 제외
    # (같이 넣으면 중복 심볼 링크 에러)
    common_objs = glob.glob(os.path.join(
        BUILD, "src", "common", "CMakeFiles", "common.dir", "**", "*.o"),
        recursive=True)

    # 저수준 Keccak 구현들은 별도 CMake 타겟에 있다.
    # SHA3 디스패처가 런타임에 CPU 기능을 보고 고르므로 전부 링크해야 한다
    # (avx2 / avx512vl / plain64 / times4).
    low_objs = glob.glob(os.path.join(
        BUILD, "src", "common", "sha3", "**", "CMakeFiles", "**", "*.o"),
        recursive=True)

    if not ref_objs:
        sys.exit(f"!! ml_kem_{variant}_ref 오브젝트가 없습니다. liboqs 를 먼저 빌드하세요.")
    if not common_objs:
        sys.exit("!! common 오브젝트가 없습니다. liboqs 를 먼저 빌드하세요.")

    out = os.path.join(HERE, f"kat_harness_{variant}")
    # -fvisibility=hidden 은 정적 링크에는 영향 없음.
    # -lcrypto : common.c 의 OQS_MEM_* 가 OpenSSL 을 참조한다.
    cmd = ([cc] + flags +
           [os.path.join(HERE, "kat_harness.c")] +
           ref_objs + common_objs + low_objs +
           ["-o", out, "-lm", "-lcrypto"])

    r = subprocess.run(cmd, cwd=workdir, capture_output=True, text=True)
    if r.returncode != 0:
        print(f"[FAIL] ML-KEM-{variant}")
        print(r.stderr[:4000])
        return False
    print(f"[ok] {out}")
    return True


ok = True
for v in VARIANTS:
    ok &= build(v)
sys.exit(0 if ok else 1)
