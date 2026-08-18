/*
 * impl #2: liboqs 원본 C (mlkem-native ref) KAT 하네스
 *
 * FIPS 203 의 내부 결정적 함수 4가지를 그대로 호출한다.
 *   KeyGen_internal  : mlk_kem_keypair_derand(pk, sk, coins=d||z)
 *   Encaps_internal  : mlk_kem_enc_derand(ct, ss, pk, coins=m)
 *   Decaps           : mlk_kem_dec(ss, ct, sk)          (정상 / 변조 둘 다 동일 호출)
 *
 * 세 구현(C2Rust / C / ml-kem crate)이 공유하는 텍스트 프로토콜:
 *   입력 한 줄:  KEYGEN <id> <d_hex> <z_hex>
 *                ENCAPS <id> <ek_hex> <m_hex>
 *                DECAPS <id> <dk_hex> <c_hex>
 *   출력 한 줄:  KEYGEN <id> <ek_hex> <dk_hex>
 *                ENCAPS <id> <c_hex> <k_hex>
 *                DECAPS <id> <k_hex> <rc>
 *
 * 사용법: kat_harness_<variant> <tasks.txt>     (결과는 stdout)
 *
 * 이 파일은 해당 variant 의 ref 소스와 "동일한 컴파일 플래그"로 빌드된다.
 * 즉 원본 liboqs C 그대로이며, asm 배리어 등 어떤 것도 바꾸지 않는다.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "mlkem/src/kem.h"

#define MAX_LINE (64 * 1024)

/*
 * mlkem-native 의 mlk_kem_* 는 함수가 아니라 매크로이며, 규격상 항상 마지막에
 * context 인자를 받는다:
 *   #define mlk_kem_dec MLK_NAMESPACE_K(dec) MLK_CONTEXT_PARAMETERS_3
 *   #define MLK_CONTEXT_PARAMETERS_3(a0, a1, a2, context) (a0, a1, a2)
 * MLK_CONFIG_CONTEXT_PARAMETER 가 정의되지 않은 이 빌드에서는 context 가
 * 전개 과정에서 그대로 버려진다(평가되지 않음). 자리만 채워주면 된다.
 */
#define MLK_CTX 0

static int hex2bin(const char *hex, uint8_t *out, size_t out_cap, size_t *out_len)
{
  size_t n = strlen(hex);
  size_t i;
  if (n % 2 != 0 || n / 2 > out_cap)
  {
    return -1;
  }
  for (i = 0; i < n / 2; i++)
  {
    unsigned v;
    if (sscanf(hex + 2 * i, "%2x", &v) != 1)
    {
      return -1;
    }
    out[i] = (uint8_t)v;
  }
  *out_len = n / 2;
  return 0;
}

static void print_hex(const uint8_t *buf, size_t len)
{
  size_t i;
  for (i = 0; i < len; i++)
  {
    printf("%02x", buf[i]);
  }
}


/* ---------------------------------------------------------------- 벤치마크 */

static uint64_t now_ns(void)
{
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec * 1000000000ull + (uint64_t)ts.tv_nsec;
}

/*
 * 네 구현이 반드시 동일한 값을 내도록 하는 체크섬.
 * 출력 바이트를 위치 mod 8 로 XOR 접기 한다. 최적화로 계산이 제거되는 것도 막는다.
 */
static uint64_t bench_acc;

/* 순서에 민감한 롤링 해시. XOR 접기와 달리 같은 값이 짝수 번 들어와도 상쇄되지 않는다.
 * 네 구현 모두 동일한 식을 사용하므로 체크섬이 같아야 한다. */
static void acc_bytes(const uint8_t *b, size_t len)
{
  size_t i;
  for (i = 0; i < len; i++)
  {
    bench_acc = bench_acc * 1000003ull + (uint64_t)b[i];
  }
}

/* 네 구현이 공유하는 고정 입력 패턴 (같은 일을 하도록 보장) */
static void base_coins(uint8_t out[2 * MLKEM_SYMBYTES])
{
  unsigned j;
  for (j = 0; j < 2 * MLKEM_SYMBYTES; j++)
  {
    out[j] = (uint8_t)((j * 7 + 1) & 0xFF);
  }
}

static void base_msg(uint8_t out[MLKEM_SYMBYTES])
{
  unsigned j;
  for (j = 0; j < MLKEM_SYMBYTES; j++)
  {
    out[j] = (uint8_t)((j * 11 + 3) & 0xFF);
  }
}

static void report(const char *op, unsigned iters, uint64_t total, uint64_t best)
{
  printf("BENCH %s %u %llu %llu %016llx\n", op, iters,
         (unsigned long long)total, (unsigned long long)best,
         (unsigned long long)bench_acc);
  bench_acc = 0;
}

static int run_bench(unsigned iters)
{
  static uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES];
  static uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES];
  static uint8_t ct[MLKEM_INDCCA_CIPHERTEXTBYTES];
  static uint8_t ct_bad[MLKEM_INDCCA_CIPHERTEXTBYTES];
  static uint8_t ss[MLKEM_SSBYTES];
  uint8_t coins[2 * MLKEM_SYMBYTES];
  uint8_t msg[MLKEM_SYMBYTES];
  unsigned i, warm;
  uint64_t t0, dt, total, best;

  warm = iters / 10 + 1;

  /* 고정 키쌍과 암호문을 미리 준비 (encaps/decaps 벤치 입력) */
  base_coins(coins);
  base_msg(msg);
  if (mlk_kem_keypair_derand(pk, sk, coins, MLK_CTX) != 0)
  {
    return 1;
  }
  if (mlk_kem_enc_derand(ct, ss, pk, msg, MLK_CTX) != 0)
  {
    return 1;
  }
  memcpy(ct_bad, ct, sizeof(ct));
  ct_bad[0] ^= 0x01; /* 변조 → 암묵적 거부 경로 */

  /* ---- keygen ---- */
  for (i = 0; i < warm; i++)
  {
    base_coins(coins);
    coins[0] = (uint8_t)(i & 0xFF);
    mlk_kem_keypair_derand(pk, sk, coins, MLK_CTX);
  }
  bench_acc = 0;
  total = 0;
  best = ~0ull;
  for (i = 0; i < iters; i++)
  {
    base_coins(coins);
    coins[0] = (uint8_t)(i & 0xFF);
    t0 = now_ns();
    mlk_kem_keypair_derand(pk, sk, coins, MLK_CTX);
    dt = now_ns() - t0;
    total += dt;
    if (dt < best)
    {
      best = dt;
    }
    acc_bytes(pk, sizeof(pk));
    acc_bytes(sk, sizeof(sk));
  }
  report("keygen", iters, total, best);

  /* 이후 벤치는 고정 키쌍을 사용 */
  base_coins(coins);
  base_msg(msg);
  mlk_kem_keypair_derand(pk, sk, coins, MLK_CTX);

  /* ---- encaps ---- */
  for (i = 0; i < warm; i++)
  {
    base_msg(msg);
    msg[0] = (uint8_t)(i & 0xFF);
    mlk_kem_enc_derand(ct, ss, pk, msg, MLK_CTX);
  }
  bench_acc = 0;
  total = 0;
  best = ~0ull;
  for (i = 0; i < iters; i++)
  {
    base_msg(msg);
    msg[0] = (uint8_t)(i & 0xFF);
    t0 = now_ns();
    mlk_kem_enc_derand(ct, ss, pk, msg, MLK_CTX);
    dt = now_ns() - t0;
    total += dt;
    if (dt < best)
    {
      best = dt;
    }
    acc_bytes(ct, sizeof(ct));
    acc_bytes(ss, sizeof(ss));
  }
  report("encaps", iters, total, best);

  /* decaps 입력을 고정 암호문으로 되돌린다 */
  base_msg(msg);
  mlk_kem_enc_derand(ct, ss, pk, msg, MLK_CTX);
  memcpy(ct_bad, ct, sizeof(ct));
  ct_bad[0] ^= 0x01;

  /* ---- decaps (정상) ---- */
  for (i = 0; i < warm; i++)
  {
    mlk_kem_dec(ss, ct, sk, MLK_CTX);
  }
  bench_acc = 0;
  total = 0;
  best = ~0ull;
  for (i = 0; i < iters; i++)
  {
    t0 = now_ns();
    mlk_kem_dec(ss, ct, sk, MLK_CTX);
    dt = now_ns() - t0;
    total += dt;
    if (dt < best)
    {
      best = dt;
    }
    acc_bytes(ss, sizeof(ss));
  }
  report("decaps", iters, total, best);

  /* ---- decaps (변조 → 암묵적 거부) ---- */
  for (i = 0; i < warm; i++)
  {
    mlk_kem_dec(ss, ct_bad, sk, MLK_CTX);
  }
  bench_acc = 0;
  total = 0;
  best = ~0ull;
  for (i = 0; i < iters; i++)
  {
    t0 = now_ns();
    mlk_kem_dec(ss, ct_bad, sk, MLK_CTX);
    dt = now_ns() - t0;
    total += dt;
    if (dt < best)
    {
      best = dt;
    }
    acc_bytes(ss, sizeof(ss));
  }
  report("reject", iters, total, best);

  return 0;
}

int main(int argc, char **argv)
{
  FILE *fp;
  char *line;
  static uint8_t pk[MLKEM_INDCCA_PUBLICKEYBYTES];
  static uint8_t sk[MLKEM_INDCCA_SECRETKEYBYTES];
  static uint8_t ct[MLKEM_INDCCA_CIPHERTEXTBYTES];
  static uint8_t ss[MLKEM_SSBYTES];
  static uint8_t bufa[64 * 1024];
  static uint8_t bufb[64 * 1024];

  if (argc < 2)
  {
    fprintf(stderr, "usage: %s <tasks.txt> | --bench <iters>\n", argv[0]);
    return 2;
  }

  if (strcmp(argv[1], "--bench") == 0)
  {
    unsigned iters = (argc >= 3) ? (unsigned)strtoul(argv[2], NULL, 10) : 100u;
    return run_bench(iters);
  }

  fp = fopen(argv[1], "r");
  if (fp == NULL)
  {
    fprintf(stderr, "cannot open %s\n", argv[1]);
    return 2;
  }

  line = (char *)malloc(MAX_LINE);
  if (line == NULL)
  {
    return 2;
  }

  while (fgets(line, MAX_LINE, fp) != NULL)
  {
    char op[32], id[64];
    char *ha, *hb;
    size_t la, lb;
    int rc;

    /* "OP id hexA hexB" 파싱: strtok 으로 4개 필드 분리 */
    char *p = strtok(line, " \t\r\n");
    if (p == NULL || p[0] == '#')
    {
      continue;
    }
    snprintf(op, sizeof(op), "%s", p);
    p = strtok(NULL, " \t\r\n");
    if (p == NULL)
    {
      continue;
    }
    snprintf(id, sizeof(id), "%s", p);
    ha = strtok(NULL, " \t\r\n");
    hb = strtok(NULL, " \t\r\n");
    if (ha == NULL || hb == NULL)
    {
      continue;
    }

    if (hex2bin(ha, bufa, sizeof(bufa), &la) != 0 ||
        hex2bin(hb, bufb, sizeof(bufb), &lb) != 0)
    {
      fprintf(stderr, "bad hex on line: %s %s\n", op, id);
      return 3;
    }

    if (strcmp(op, "KEYGEN") == 0)
    {
      /* coins = d || z (2 * MLKEM_SYMBYTES = 64 bytes) */
      uint8_t coins[2 * MLKEM_SYMBYTES];
      if (la != MLKEM_SYMBYTES || lb != MLKEM_SYMBYTES)
      {
        fprintf(stderr, "KEYGEN %s: d/z must be 32 bytes\n", id);
        return 3;
      }
      memcpy(coins, bufa, MLKEM_SYMBYTES);
      memcpy(coins + MLKEM_SYMBYTES, bufb, MLKEM_SYMBYTES);

      rc = mlk_kem_keypair_derand(pk, sk, coins, MLK_CTX);
      if (rc != 0)
      {
        fprintf(stderr, "KEYGEN %s: returned %d\n", id, rc);
        return 4;
      }
      printf("KEYGEN %s ", id);
      print_hex(pk, sizeof(pk));
      printf(" ");
      print_hex(sk, sizeof(sk));
      printf("\n");
    }
    else if (strcmp(op, "ENCAPS") == 0)
    {
      /* bufa = ek, bufb = m (coins) */
      if (la != MLKEM_INDCCA_PUBLICKEYBYTES || lb != MLKEM_SYMBYTES)
      {
        fprintf(stderr, "ENCAPS %s: size mismatch (ek=%zu m=%zu)\n", id, la, lb);
        return 3;
      }
      rc = mlk_kem_enc_derand(ct, ss, bufa, bufb, MLK_CTX);
      if (rc != 0)
      {
        fprintf(stderr, "ENCAPS %s: returned %d\n", id, rc);
        return 4;
      }
      printf("ENCAPS %s ", id);
      print_hex(ct, sizeof(ct));
      printf(" ");
      print_hex(ss, sizeof(ss));
      printf("\n");
    }
    else if (strcmp(op, "DECAPS") == 0)
    {
      /* bufa = dk, bufb = ct */
      if (la != MLKEM_INDCCA_SECRETKEYBYTES ||
          lb != MLKEM_INDCCA_CIPHERTEXTBYTES)
      {
        fprintf(stderr, "DECAPS %s: size mismatch (dk=%zu ct=%zu)\n", id, la, lb);
        return 3;
      }
      /* ML-KEM 은 변조된 암호문에도 오류를 내지 않는다(암묵적 거부):
       * 규격대로 z 기반 의사난수 공유키를 반환하고 rc=0 이다. */
      rc = mlk_kem_dec(ss, bufb, bufa, MLK_CTX);
      printf("DECAPS %s ", id);
      print_hex(ss, sizeof(ss));
      printf(" %d\n", rc);
    }
    else if (strcmp(op, "CHECKEK") == 0)
    {
      /* FIPS 203 7.2 입력 검증은 두 단계다.
       *   1) 타입 검사: 길이가 규격과 정확히 같은가
       *   2) 모듈러스 검사: 디코드 후 재인코딩이 원본과 같은가
       * 길이가 다르면 내용 검사로 넘어가지 않고 바로 거부한다. */
      if (la != MLKEM_INDCCA_PUBLICKEYBYTES)
      {
        rc = 1;
      }
      else
      {
        rc = mlk_kem_check_pk(bufa, MLK_CTX) != 0 ? 1 : 0;
      }
      printf("CHECKEK %s %d 0\n", id, rc);
    }
    else if (strcmp(op, "CHECKDK") == 0)
    {
      /* FIPS 203 7.3: 길이 검사 후 H(ek) 해시 검사 */
      if (la != MLKEM_INDCCA_SECRETKEYBYTES)
      {
        rc = 1;
      }
      else
      {
        rc = mlk_kem_check_sk(bufa, MLK_CTX) != 0 ? 1 : 0;
      }
      printf("CHECKDK %s %d 0\n", id, rc);
    }
    else
    {
      fprintf(stderr, "unknown op: %s\n", op);
      return 3;
    }
  }

  free(line);
  fclose(fp);
  return 0;
}
