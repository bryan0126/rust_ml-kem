#!/usr/bin/env python3
"""
ML-KEM KAT 교차검증 오케스트레이터.

검증 대상 4구현 x 파라미터 3종(512/768/1024) x 6가지 항목
  1) KeyGen_internal   : (d, z) -> (ek, dk)          [ACVP 정답값 대조]
  2) Encaps_internal   : (ek, m) -> (c, K)           [ACVP 정답값 대조]
  3) Decaps 정상       : (dk, c) -> K                [ACVP 정답값 대조]
  4) Decaps 변조 암호문: (dk, c') -> K'              [ACVP 정답값 대조]
  5) ek 유효성 검사    : 비정규 ek 를 거부하는가      [ACVP testPassed 대조]
  6) dk 유효성 검사    : H(ek) 불일치 dk 를 거부하는가 [ACVP testPassed 대조]

  ※ 5,6 은 FIPS 203 7.2(모듈러스 검사) / 7.3(해시 검사) 에 해당한다. 정상 키만으로는
     검사 유무를 구별할 수 없으므로, 부정 키를 넣어 '거부하는지'를 확인해야 한다.

  ※ 4번 주의: FIPS 203 의 ML-KEM 은 변조된 암호문에 "오류를 반환하지 않는다".
     대신 dk 안의 z 로부터 유도한 의사난수 공유키를 돌려준다(암묵적 거부,
     implicit rejection). 따라서 "틀렸다고 반환하는가"의 올바른 판정 기준은
     '에러 코드'가 아니라 '규격이 정한 거부값과 바이트 단위로 일치하는가' 이다.
     ACVP encapDecap 벡터는 reason="modified ciphertext" 케이스를 파라미터별
     5개씩 정답 K 와 함께 제공하므로, 이 항목도 NIST 정답값으로 검증한다.
     (추가로 구현 간 상호일치도 함께 확인한다.)

구현 목록
  c2rust : C2Rust 로 변환한 liboqs ML-KEM ref + sha3/rand crate shim
  c      : liboqs 원본 C (mlkem-native ref, 순정)
  mlkem  : RustCrypto ml-kem crate (순수 Rust 오픈소스)
  rs     : 위 c2rust 산출물을 관용적 safe Rust 로 리팩터링한 구현 (mlkem_rs)

사용법
  python3 kat.py fetch     # ACVP 벡터 다운로드
  python3 kat.py run       # 전체 검증 실행 + 리포트
  python3 kat.py run --limit 20   # 케이스 수 제한(빠른 확인)
"""
import argparse
import json
import os
import subprocess
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))          # tests/ml-kem
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))     # 저장소 루트
VECTORS = os.path.join(HERE, "vectors")
RESULTS = os.path.join(HERE, "results")
# cargo 산출물은 워크스페이스 루트의 target/ 에 모인다
TARGET = os.path.join(ROOT, "target", "debug")
VARIANTS = ["512", "768", "1024"]

# liboqs 자신의 tests/test_acvp_vectors.py 와 동일한 출처/태그
URLROOT = ("https://raw.githubusercontent.com/usnistgov/ACVP-Server/"
           "refs/tags/v1.1.0.42/gen-val/json-files/")
SOURCES = {
    "keygen": URLROOT + "ML-KEM-keyGen-FIPS203/internalProjection.json",
    "encdec": URLROOT + "ML-KEM-encapDecap-FIPS203/internalProjection.json",
}

IMPLS = {
    "c2rust": lambda v: [os.path.join(TARGET, "harness_c2rust"), v],
    "c":      lambda v: [os.path.join(ROOT, "c", "ml-kem", f"kat_harness_{v}")],
    "mlkem":  lambda v: [os.path.join(TARGET, "harness_mlkem"), v],
    "rs":     lambda v: [os.path.join(TARGET, "harness_rs"), v],
}


# ---------------------------------------------------------------- fetch

def fetch():
    os.makedirs(VECTORS, exist_ok=True)
    for name, url in SOURCES.items():
        dest = os.path.join(VECTORS, f"{name}.json")
        if os.path.isfile(dest):
            print(f"[skip] {dest} (이미 있음)")
            continue
        print(f"[get ] {url}")
        with urllib.request.urlopen(url, timeout=120) as r:
            data = r.read()
        with open(dest, "wb") as f:
            f.write(data)
        print(f"       -> {dest} ({len(data)} bytes)")


# ------------------------------------------------------- ACVP 파싱

def acvp_cases(limit=None):
    """ACVP JSON 을 variant 별 (tasks, expected) 로 변환."""
    with open(os.path.join(VECTORS, "keygen.json")) as f:
        kg = json.load(f)
    with open(os.path.join(VECTORS, "encdec.json")) as f:
        ed = json.load(f)

    # variant -> {"tasks":[line...], "expect":{id:{...}}}
    out = {v: {"tasks": [], "expect": {}} for v in VARIANTS}

    def variant_of(group):
        # "ML-KEM-768" -> "768"
        ps = group.get("parameterSet", "")
        return ps.replace("ML-KEM-", "") if ps.startswith("ML-KEM-") else None

    # --- KeyGen: (d, z) -> (ek, dk)
    for g in kg.get("testGroups", []):
        v = variant_of(g)
        if v not in out:
            continue
        tests = g.get("tests", [])
        if limit:
            tests = tests[:limit]
        for t in tests:
            tid = f"kg{t['tcId']}"
            out[v]["tasks"].append(f"KEYGEN {tid} {t['d']} {t['z']}")
            out[v]["expect"][tid] = {"kind": "keygen",
                                     "ek": t["ek"].lower(),
                                     "dk": t["dk"].lower()}

    # --- EncapDecap: function=encapsulation(AFT) / decapsulation(VAL)
    for g in ed.get("testGroups", []):
        v = variant_of(g)
        if v not in out:
            continue
        func = g.get("function", "")
        tests = g.get("tests", [])
        if limit:
            tests = tests[:limit]

        if func == "encapsulation":
            for t in tests:
                tid = f"en{t['tcId']}"
                out[v]["tasks"].append(f"ENCAPS {tid} {t['ek']} {t['m']}")
                out[v]["expect"][tid] = {"kind": "encaps",
                                         "c": t["c"].lower(),
                                         "k": t["k"].lower()}
        elif func == "decapsulation":
            # ACVP 는 두 종류를 함께 준다 (reason 필드로 구분):
            #   "valid decapsulation" -> 정상 복호 (3번 항목)
            #   "modified ciphertext" -> 변조 암호문. k 는 규격상의
            #                            암묵적 거부(implicit rejection) 값이다.
            #                            즉 4번 항목도 NIST 정답값으로 검증된다.
            dk_group = g.get("dk")
            for t in tests:
                dk_t = t.get("dk", dk_group)
                if dk_t is None:
                    continue
                reason = t.get("reason", "")
                tampered = "modified" in reason
                tid = ("dt" if tampered else "dv") + str(t["tcId"])
                out[v]["tasks"].append(f"DECAPS {tid} {dk_t} {t['c']}")
                out[v]["expect"][tid] = {
                    "kind": "decaps_tampered" if tampered else "decaps",
                    "k": t["k"].lower(),
                    "dk": dk_t.lower(),
                    "c": t["c"].lower(),
                }
        elif func in ("encapsulationKeyCheck", "decapsulationKeyCheck"):
            # testPassed=True  -> 유효한 키. 구현은 받아들여야 한다(rc=0)
            # testPassed=False -> 부정 키. 구현은 거부해야 한다(rc=1)
            is_ek = func == "encapsulationKeyCheck"
            op = "CHECKEK" if is_ek else "CHECKDK"
            for t in tests:
                key = t["ek"] if is_ek else t["dk"]
                tid = ("ck" if is_ek else "cd") + str(t["tcId"])
                # 프로토콜은 4개 필드를 요구하므로 두 번째 값은 자리채움(00)
                out[v]["tasks"].append(f"{op} {tid} {key} 00")
                out[v]["expect"][tid] = {
                    "kind": "check_ek" if is_ek else "check_dk",
                    "accept": bool(t["testPassed"]),
                    "reason": t.get("reason", ""),
                }
    return out


# ---------------------------------------------------------------- run

def available_impls():
    """실행 파일이 있는 구현만 고른다.

    C 원본은 이 저장소에 들어 있지 않다. 외부 liboqs 를 빌드하고
    c/ml-kem/build_c.py 를 돌려야 생기므로, 없으면 건너뛴다.
    """
    found, missing = {}, []
    for name, make_cmd in IMPLS.items():
        # variant 는 실행 파일 경로에만 쓰이므로 아무 값이나 넣어 존재만 본다
        if os.path.isfile(make_cmd(VARIANTS[0])[0]):
            found[name] = make_cmd
        else:
            missing.append(name)
    return found, missing


def run_impl(name, variant, tasks_path):
    cmd = IMPLS[name](variant) + [tasks_path]
    if not os.path.isfile(cmd[0]):
        return None, f"실행파일 없음: {cmd[0]}"
    r = subprocess.run(cmd, capture_output=True, text=True)
    if r.returncode != 0:
        return None, f"rc={r.returncode}: {r.stderr.strip()[:300]}"

    parsed = {}
    for line in r.stdout.splitlines():
        f = line.split()
        if len(f) < 4:
            continue
        op, tid = f[0], f[1]
        if op == "KEYGEN":
            parsed[tid] = {"ek": f[2].lower(), "dk": f[3].lower()}
        elif op == "ENCAPS":
            parsed[tid] = {"c": f[2].lower(), "k": f[3].lower()}
        elif op == "DECAPS":
            parsed[tid] = {"k": f[2].lower(), "rc": f[3]}
        elif op in ("CHECKEK", "CHECKDK"):
            # rc 0 = 키를 받아들임, 1 = 거부
            parsed[tid] = {"accepted": f[2] == "0"}
    return parsed, None


def judge(variant, expect, results):
    """4개 항목별로 (통과, 전체, 실패상세) 집계."""
    kinds = ["keygen", "encaps", "decaps", "decaps_tampered", "check_ek", "check_dk"]
    tally = {k: {"pass": 0, "total": 0, "fail": []} for k in kinds}

    for tid, e in expect.items():
        kind = e["kind"]
        t = tally[kind]
        t["total"] += 1

        got = {impl: results[impl].get(tid) if results[impl] else None
               for impl in IMPLS}
        if any(g is None for g in got.values()):
            missing = [i for i, g in got.items() if g is None]
            t["fail"].append(f"{tid}: 결과 없음 ({','.join(missing)})")
            continue

        problems = []

        if kind == "keygen":
            for impl, g in got.items():
                if g["ek"] != e["ek"]:
                    problems.append(f"{impl}.ek≠ACVP")
                if g["dk"] != e["dk"]:
                    problems.append(f"{impl}.dk≠ACVP")
        elif kind == "encaps":
            for impl, g in got.items():
                if g["c"] != e["c"]:
                    problems.append(f"{impl}.c≠ACVP")
                if g["k"] != e["k"]:
                    problems.append(f"{impl}.K≠ACVP")
        elif kind in ("decaps", "decaps_tampered"):
            # 변조 케이스도 ACVP 가 정답 K(암묵적 거부값)를 주므로 동일하게 대조한다.
            for impl, g in got.items():
                if g["k"] != e["k"]:
                    problems.append(f"{impl}.K≠ACVP")
            # 추가로 3구현 상호일치까지 확인 (ACVP 대조와 독립적인 안전망)
            ks = {impl: g["k"] for impl, g in got.items()}
            if len(set(ks.values())) != 1:
                problems.append("구현간 불일치: " +
                                ", ".join(f"{i}={k[:16]}" for i, k in ks.items()))
        elif kind in ("check_ek", "check_dk"):
            # ACVP 의 testPassed 와 구현의 수락/거부가 일치해야 한다
            for impl, g in got.items():
                if g["accepted"] != e["accept"]:
                    want = "수락" if e["accept"] else "거부"
                    problems.append(f"{impl}: {want}해야 하는데 반대로 처리")

        if problems:
            t["fail"].append(f"{tid}: " + "; ".join(problems))
        else:
            t["pass"] += 1
    return tally


def run(limit=None):
    for f in ("keygen.json", "encdec.json"):
        if not os.path.isfile(os.path.join(VECTORS, f)):
            sys.exit("!! ACVP 벡터가 없습니다. 먼저:  python3 kat.py fetch")

    global IMPLS
    IMPLS, missing = available_impls()
    if not IMPLS:
        sys.exit("!! 실행 가능한 구현이 없습니다. 먼저 `cargo build` 를 실행하세요.")
    print("검증 대상:", ", ".join(IMPLS))
    if missing:
        print("건너뜀:", ", ".join(missing))
        if "c" in missing:
            print("  (C 원본은 외부 liboqs 가 필요합니다. docs/ml-kem/README.md 참고)")

    os.makedirs(RESULTS, exist_ok=True)
    cases = acvp_cases(limit)

    label = {"keygen": "1. KeyGen_internal",
             "encaps": "2. Encaps_internal",
             "decaps": "3. Decaps (정상)",
             "decaps_tampered": "4. Decaps (변조 c → 암묵적 거부)",
             "check_ek": "5. ek 유효성 검사 (FIPS 203 7.2)",
             "check_dk": "6. dk 유효성 검사 (FIPS 203 7.3)"}

    all_ok = True
    report = {}

    for v in VARIANTS:
        d = cases[v]
        if not d["tasks"]:
            print(f"\n[ML-KEM-{v}] ACVP 케이스 없음 — 건너뜀")
            continue

        tasks_path = os.path.join(RESULTS, f"tasks_{v}.txt")
        with open(tasks_path, "w") as f:
            f.write("\n".join(d["tasks"]) + "\n")

        print(f"\n{'='*66}")
        print(f"ML-KEM-{v}   케이스 {len(d['tasks'])}개")
        print("=" * 66)

        results, errs = {}, {}
        for impl in IMPLS:
            res, err = run_impl(impl, v, tasks_path)
            results[impl] = res
            if err:
                errs[impl] = err
                all_ok = False
                print(f"  [!] {impl}: {err}")

        tally = judge(v, d["expect"], results)
        report[v] = tally

        for kind in ["keygen", "encaps", "decaps", "decaps_tampered",
                     "check_ek", "check_dk"]:
            t = tally[kind]
            if t["total"] == 0:
                continue
            ok = t["pass"] == t["total"]
            all_ok &= ok
            mark = "PASS" if ok else "FAIL"
            print(f"  [{mark}] {label[kind]:<38} {t['pass']}/{t['total']}")
            for msg in t["fail"][:5]:
                print(f"         - {msg}")
            if len(t["fail"]) > 5:
                print(f"         ... 외 {len(t['fail'])-5}건")

    with open(os.path.join(RESULTS, "report.json"), "w") as f:
        json.dump(report, f, indent=2, ensure_ascii=False)

    print(f"\n{'='*66}")
    print("최종:", "전체 통과 ✅" if all_ok else "실패 있음 ❌")
    print(f"리포트: {os.path.join(RESULTS, 'report.json')}")
    print("=" * 66)
    return 0 if all_ok else 1


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["fetch", "run"])
    ap.add_argument("--limit", type=int, default=None,
                    help="그룹별 케이스 수 제한(빠른 확인용)")
    a = ap.parse_args()
    if a.cmd == "fetch":
        fetch()
    else:
        sys.exit(run(a.limit))
