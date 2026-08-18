#!/usr/bin/env python3
"""
ML-KEM 4구현 x 3파라미터 벤치마크 오케스트레이터.

측정 대상 (KAT 과 동일한 결정적 내부 함수)
  keygen : KeyGen_internal(d, z)
  encaps : Encaps_internal(ek, m)
  decaps : Decaps(dk, c)          — 정상 암호문
  reject : Decaps(dk, c')         — 변조 암호문(암묵적 거부 경로)

구현
  c       : liboqs 원본 C (mlkem-native ref, -O3)
  c2rust  : C2Rust 로 변환한 Rust (unsafe/raw pointer) + sha3 shim
  rs      : 위를 관용적 safe Rust 로 리팩터링한 구현 (mlkem_rs)
  mlkem   : RustCrypto ml-kem crate (독립 구현)

공정성을 위해 네 구현이 동일하게 맞춘 것
  * 입력 패턴(고정 시드/메시지, 반복마다 첫 바이트만 변경)
  * 워밍업 횟수 = iters/10 + 1
  * 계측 구간은 연산만 (체크섬 접기는 구간 밖)
  * 체크섬 식(순서 민감 롤링 해시) → 네 구현이 같은 값이어야 한다
  * Rust 는 반드시 release 빌드로 측정 (C 는 liboqs 의 -O3 플래그)

사용법
  python3 bench.py                 # release 빌드 후 iters=1000 으로 측정
  python3 bench.py --iters 200
  python3 bench.py --no-build      # 이미 빌드했다면 생략
  python3 bench.py --debug         # debug 빌드로 측정(비교 의미 없음, 디버깅용)
"""
import argparse
import json
import os
import subprocess
import sys
import unicodedata

HERE = os.path.dirname(os.path.abspath(__file__))          # benches/ml-kem
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))     # 저장소 루트
RESULTS = os.path.join(HERE, "results")
VARIANTS = ["512", "768", "1024"]
OPS = ["keygen", "encaps", "decaps", "reject"]
OP_LABEL = {
    "keygen": "KeyGen_internal",
    "encaps": "Encaps_internal",
    "decaps": "Decaps (정상)",
    "reject": "Decaps (변조/거부)",
}
# 표시 순서: C 를 기준(1.00x)으로 둔다
IMPL_ORDER = ["c", "c2rust", "rs", "mlkem"]
IMPL_LABEL = {
    "c": "C (liboqs 원본)",
    "c2rust": "C2Rust 변환",
    "rs": "Rust向 코드",
    "mlkem": "ml-kem crate",
}


def cargo_bin(profile, name):
    # cargo 산출물은 워크스페이스 루트의 target/ 에 모인다
    return os.path.join(ROOT, "target", profile, name)


def cmd_for(impl, variant, profile):
    if impl == "c":
        return [os.path.join(ROOT, "c", "ml-kem", f"kat_harness_{variant}")]
    binname = {"c2rust": "harness_c2rust", "rs": "harness_rs", "mlkem": "harness_mlkem"}[impl]
    return [cargo_bin(profile, binname), variant]


def available_impls(profile):
    """실행 파일이 있는 구현만 고른다. C 원본은 외부 liboqs 가 필요하다."""
    found, missing = [], []
    for impl in IMPL_ORDER:
        (found if os.path.isfile(cmd_for(impl, VARIANTS[0], profile)[0])
         else missing).append(impl)
    return found, missing


def build(profile):
    print(f"[build] cargo build ({profile}) ...", flush=True)
    args = ["cargo", "build", "--quiet"]
    if profile == "release":
        args.append("--release")
    r = subprocess.run(args, cwd=ROOT, capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit("!! cargo build 실패\n" + r.stderr[-3000:])

    # C 하네스는 외부 liboqs 가 있어야 빌드된다. 없으면 경고만 하고 넘어간다.
    print("[build] C 하네스 ...", flush=True)
    r = subprocess.run([sys.executable, "build_c.py"],
                       cwd=os.path.join(ROOT, "c", "ml-kem"),
                       capture_output=True, text=True)
    if r.returncode != 0:
        print("[warn] C 하네스를 빌드하지 못해 C 원본은 비교에서 제외됩니다.")
        print("       " + (r.stdout + r.stderr).strip().splitlines()[-1][:160])


def run_one(impl, variant, iters, profile):
    cmd = cmd_for(impl, variant, profile) + ["--bench", str(iters)]
    if not os.path.isfile(cmd[0]):
        return None, f"실행파일 없음: {cmd[0]}"
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        return None, f"rc={r.returncode}: {r.stderr.strip()[:300]}"

    out = {}
    for line in r.stdout.splitlines():
        f = line.split()
        if len(f) != 6 or f[0] != "BENCH":
            continue
        _, op, it, total, best, csum = f
        n = int(it)
        out[op] = {
            "iters": n,
            "mean_ns": int(total) / n,
            "best_ns": int(best),
            "checksum": csum,
        }
    if not out:
        return None, "BENCH 출력 없음"
    return out, None


def dwidth(s):
    """터미널 표시 폭. 한글/한자 등 전각 문자는 두 칸으로 센다."""
    return sum(2 if unicodedata.east_asian_width(c) in "WF" else 1 for c in s)


def pad(s, width, right=False):
    """표시 폭 기준 정렬. str.format 은 전각 문자를 한 칸으로 세어 열이 어긋난다."""
    space = " " * max(0, width - dwidth(s))
    return space + s if right else s + space


def fmt_cell(ns, base_ns, is_base):
    """시간과 C 대비 비율을 한 칸에 담는다. 예: '18.13 (142%)'"""
    us = ns / 1000.0
    if is_base or not base_ns:
        return f"{us:.2f}"
    return f"{us:.2f} ({round(ns / base_ns * 100)}%)"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--iters", type=int, default=1000)
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("--debug", action="store_true",
                    help="debug 빌드로 측정 (최적화 없음 — 성능 비교 목적으로는 무의미)")
    a = ap.parse_args()

    profile = "debug" if a.debug else "release"
    if not a.no_build:
        build(profile)
    if a.debug:
        print("\n[경고] debug 빌드입니다. Rust 는 최적화가 꺼져 있어 C(-O3)와의")
        print("       비교는 의미가 없습니다. 성능 비교는 release 로 실행하세요.\n")

    global IMPL_ORDER
    IMPL_ORDER, missing = available_impls(profile)
    if not IMPL_ORDER:
        sys.exit("!! 측정 가능한 구현이 없습니다. 먼저 `cargo build --release` 를 실행하세요.")
    print("측정 대상:", ", ".join(IMPL_ORDER))
    if missing:
        print("건너뜀:", ", ".join(missing),
              "(C 원본은 외부 liboqs 필요 — docs/ml-kem/README.md 참고)")

    os.makedirs(RESULTS, exist_ok=True)
    data = {}
    problems = []

    print(f"\n측정 중 (반복 {a.iters}회, 프로파일 {profile}) ...", flush=True)
    for v in VARIANTS:
        data[v] = {}
        for impl in IMPL_ORDER:
            res, err = run_one(impl, v, a.iters, profile)
            data[v][impl] = res
            status = "ok" if res else f"실패({err})"
            if err:
                problems.append(f"ML-KEM-{v} / {impl}: {err}")
            print(f"  ML-KEM-{v:<4} {impl:<7} {status}", flush=True)

    # ---------------- 체크섬 교차검증 ----------------
    print("\n" + "=" * 78)
    print("체크섬 교차검증 — 네 구현이 같은 계산을 했는가")
    print("=" * 78)
    csum_ok = True
    for v in VARIANTS:
        for op in OPS:
            sums = {i: data[v][i][op]["checksum"]
                    for i in IMPL_ORDER
                    if data[v].get(i) and op in data[v][i]}
            if not sums:
                continue
            uniq = set(sums.values())
            if len(uniq) == 1:
                print(f"  [OK]   ML-KEM-{v:<4} {op:<7} {next(iter(uniq))}")
            else:
                csum_ok = False
                print(f"  [DIFF] ML-KEM-{v:<4} {op:<7} " +
                      ", ".join(f"{i}={c[:12]}" for i, c in sums.items()))

    # ---------------- 성능 표 ----------------
    for v in VARIANTS:
        print("\n" + "=" * 78)
        print(f"ML-KEM-{v}   (반복 {a.iters}회, 단위 마이크로초)")
        print("=" * 78)
        OPW, CW = 22, 18
        header = "  " + pad("연산", OPW)
        for impl in IMPL_ORDER:
            header += pad(IMPL_LABEL[impl], CW, right=True)
        print(header)
        print("  " + "-" * (OPW + CW * len(IMPL_ORDER)))

        base = data[v].get("c")
        for op in OPS:
            base_ns = base[op]["mean_ns"] if base and op in base else None
            row = "  " + pad(OP_LABEL[op], OPW)
            for impl in IMPL_ORDER:
                d = data[v].get(impl)
                cell = (fmt_cell(d[op]["mean_ns"], base_ns, impl == "c")
                        if d and op in d else "-")
                row += pad(cell, CW, right=True)
            print(row)
        print()

    # ---------------- 저장 ----------------
    out_path = os.path.join(RESULTS, "bench.json")
    with open(out_path, "w") as f:
        json.dump({"iters": a.iters, "profile": profile, "data": data}, f,
                  indent=2, ensure_ascii=False)

    print("=" * 78)
    print("값은 평균(mean) 마이크로초이고, 괄호는 C 원본을 100% 로 본 비율입니다.")
    print("best(최소) 값은 bench.json 에 함께 저장됩니다.")
    print(f"저장: {out_path}")
    if problems:
        print("\n문제:")
        for p in problems:
            print("  -", p)
    if not csum_ok:
        print("\n[경고] 체크섬이 다릅니다 — 구현들이 서로 다른 결과를 냈습니다.")
    print("=" * 78)
    return 0 if (csum_ok and not problems) else 1


if __name__ == "__main__":
    sys.exit(main())
