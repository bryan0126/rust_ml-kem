/*
 * C2Rust 변환 전용 mlkem-native 설정 헤더.
 *
 * 목적: 인라인 어셈블리를 쓰지 않는 "순수 C" 경로로 강제해서,
 *       C2Rust 가 컴파일 가능한 Rust 를 뽑도록 한다.
 *
 * 배경: mlkem-native 는 상수시간 보장을 위해 두 가지 구현을 갖고 있다.
 *   (1) __asm__ 기반 배리어  <- 기본값. C2Rust 가 Rust 인라인 asm 으로 옮기지만
 *                              Rust 의 asm! 규칙(피연산자가 템플릿에 등장해야 함,
 *                              u8 은 reg 클래스 불가)과 맞지 않아 컴파일 실패.
 *   (2) volatile opt-blocker 기반 배리어  <- 순수 C. 동작/값은 동일.
 * 아래 두 스위치로 (2) 경로를 선택한다.
 *
 * 이 헤더는 -DMLK_CONFIG_FILE=... 로 지정되어 mlkem/src/common.h 가 include 한다.
 * liboqs 원본 소스는 전혀 수정하지 않는다.
 */

#ifndef MLK_C2RUST_CONFIG_H
#define MLK_C2RUST_CONFIG_H

/*
 * (1) 값 배리어: __asm__ 대신 volatile 전역(mlk_ct_opt_blocker_u64) 사용.
 *     이 매크로가 정의되면 MLK_USE_ASM_VALUE_BARRIER 가 설정되지 않고,
 *     verify.c 가 opt-blocker 전역을 정의한다.
 */
#define MLK_CONFIG_NO_ASM_VALUE_BARRIER

/*
 * (2) zeroize: __asm__ volatile("" : : "r"(ptr) : "memory") 대신
 *     volatile 포인터로 직접 0을 쓴다. volatile 이므로 컴파일러가
 *     최적화로 제거하지 못한다.
 */
#define MLK_CONFIG_CUSTOM_ZEROIZE

/* liboqs 원본 설정(파라미터 세트, 네임스페이스, FIPS202 glue, randombytes)을 그대로 계승.
 * MLK_CONFIG_FILE 로 지정된 경로 기준이 아니라 이 파일 기준의 상대 경로가 아니므로,
 * 빌드 시 -I 로 해당 variant 의 integration/liboqs 디렉터리를 넣어준다. */
#include "config_c.h"

/* config_c.h 가 sys.h 를 포함하므로 이 지점에서 MLK_INLINE 을 쓸 수 있다. */
#if !defined(__ASSEMBLER__)
#include <stddef.h>
#include <stdint.h>

static MLK_INLINE void mlk_zeroize(void *ptr, size_t len)
{
  volatile uint8_t *volatile_ptr = (volatile uint8_t *)ptr;
  size_t i;
  for (i = 0; i < len; i++)
  {
    volatile_ptr[i] = 0;
  }
}
#endif /* !__ASSEMBLER__ */

#endif /* MLK_C2RUST_CONFIG_H */
