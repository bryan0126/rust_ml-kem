#!/usr/bin/env python3
"""
build/compile_commands.json 에서 ML-KEM 소스만 골라 C2Rust 용 컴파일 DB를 만든다.

두 가지 일을 한다.
  1) 해당 variant(512/768/1024) 의 ref 구현 .c 파일만 추출
  2) 컴파일 명령을 다시 써서 "인라인 어셈블리 없는 순수 C" 경로로 강제
     - MLK_CONFIG_FILE 을 config/c2rust_config.h 로 교체
       (원본 config_c.h 를 include 하면서 asm 배리어/zeroize 만 순수 C 로 바꿈)
     - 해당 variant 의 integration/liboqs 를 -I 에 추가
       (c2rust_config.h 안의 #include "config_c.h" 가 해결되도록)
     liboqs 원본 소스는 수정하지 않는다.

또한 c2rust(내부 clang 도구)는 파일 이름이 반드시 'compile_commands.json' 이어야
인식하므로, 결과를 cdb/<variant>/compile_commands.json 로 저장한다.

사용법:
    python3 filter_cc.py 768 ref
    python3 filter_cc.py 512 ref
    python3 filter_cc.py 1024 ref
그 다음:
    mkdir -p rust_out/768
    c2rust transpile cdb/768/compile_commands.json --emit-build-files -o rust_out/768
"""
import json
import os
import re
import shlex
import sys

variant = sys.argv[1] if len(sys.argv) > 1 else "768"   # 512 / 768 / 1024
impl    = sys.argv[2] if len(sys.argv) > 2 else "ref"    # ref / x86_64 / aarch64

HERE = os.path.dirname(os.path.abspath(__file__))          # tools/ml-kem

# liboqs 는 이 저장소 밖에 있다. LIBOQS_DIR 로 지정하며, 없으면 ~/liboqs 를 쓴다.
LIBOQS = os.environ.get("LIBOQS_DIR") or os.path.expanduser("~/liboqs")

cc_path = os.path.join(LIBOQS, "build", "compile_commands.json")
if not os.path.isfile(cc_path):
    sys.exit(f"!! 컴파일 DB 가 없습니다: {cc_path}\n"
             f"   liboqs 를 먼저 빌드하고 LIBOQS_DIR 을 지정하세요.")
with open(cc_path) as f:
    entries = json.load(f)

needle = f"mlkem-native_ml-kem-{variant}_{impl}"
picked = [e for e in entries if needle in e["file"] and e["file"].endswith(".c")]
if not picked:
    sys.exit(f"!! '{needle}' 에 맞는 항목이 없습니다. variant/impl 확인.")

# C2Rust 전용 설정 헤더 / 원본 integration 디렉터리
our_config = os.path.join(HERE, "config", "c2rust_config.h")
if not os.path.isfile(our_config):
    sys.exit(f"!! 설정 헤더가 없습니다: {our_config}")
integration_dir = os.path.join(LIBOQS, "src", "kem", "ml_kem", needle,
                               "integration", "liboqs")

out = []
for e in picked:
    # 셸 문자열을 argv 로 파싱 (-DMLK_CONFIG_FILE=\"...\" 의 escape 를 정확히 처리)
    argv = shlex.split(e["command"])

    new_argv = []
    for a in argv:
        if a.startswith("-DMLK_CONFIG_FILE="):
            # #include MLK_CONFIG_FILE 로 쓰이므로 값에 따옴표가 포함돼야 한다
            new_argv.append(f'-DMLK_CONFIG_FILE="{our_config}"')
        else:
            new_argv.append(a)

    # c2rust_config.h 안의 #include "config_c.h" 를 찾을 수 있도록 -I 추가
    # (컴파일러 이름 바로 뒤에 삽입)
    new_argv.insert(1, f"-I{integration_dir}")

    ne = dict(e)
    ne.pop("command", None)          # arguments 형식이 quoting 문제에서 자유롭다
    ne["arguments"] = new_argv
    out.append(ne)

out_dir = os.path.join(HERE, "cdb", variant)
os.makedirs(out_dir, exist_ok=True)
out_path = os.path.join(out_dir, "compile_commands.json")   # 이름 고정!
with open(out_path, "w") as f:
    json.dump(out, f, indent=2)

print(f"[{needle}] {len(out)}개 파일 → {out_path}")
print(f"  MLK_CONFIG_FILE -> {our_config}")
print(f"  추가 include    -> {integration_dir}")
for e in out:
    print("   ", os.path.relpath(e["file"], LIBOQS))
