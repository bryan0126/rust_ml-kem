# fdl-rust-pqc
#
# 사전 준비: 이 저장소에는 C 참조 구현이 들어 있지 않다. C 원본까지 비교하려면
# liboqs 를 따로 받아 빌드한 뒤 LIBOQS_DIR 로 경로를 알려준다.
#
#   git clone https://github.com/open-quantum-safe/liboqs ~/liboqs
#   cmake -S ~/liboqs -B ~/liboqs/build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
#   cmake --build ~/liboqs/build -j
#
# liboqs 없이도 Rust 구현 세 가지는 그대로 검증·측정된다.

LIBOQS_DIR ?= $(HOME)/liboqs
ITERS      ?= 3000

.PHONY: help ml-kem ml-kem-build ml-kem-kat ml-kem-bench clean

help:
	@echo "make ml-kem         빌드 -> KAT 검증 -> 성능 측정"
	@echo "make ml-kem-build   Rust 크레이트 + C 하네스 빌드"
	@echo "make ml-kem-kat     ACVP 벡터로 KAT 검증"
	@echo "make ml-kem-bench   성능 측정 (ITERS=$(ITERS))"
	@echo ""
	@echo "  LIBOQS_DIR=$(LIBOQS_DIR)"

ml-kem: ml-kem-build ml-kem-kat ml-kem-bench

ml-kem-build:
	cargo build --release
	cargo build
	@LIBOQS_DIR=$(LIBOQS_DIR) $(MAKE) -s ml-kem-build-c || \
	  echo "[warn] C 하네스 생략 — liboqs 가 없으면 Rust 세 구현만 비교한다"

ml-kem-build-c:
	cd c/ml-kem && LIBOQS_DIR=$(LIBOQS_DIR) python3 build_c.py

ml-kem-kat:
	cd tests/ml-kem && python3 kat.py fetch && python3 kat.py run

ml-kem-bench:
	cd benches/ml-kem && python3 bench.py --iters $(ITERS) --no-build

clean:
	cargo clean
	rm -f c/ml-kem/kat_harness_*
