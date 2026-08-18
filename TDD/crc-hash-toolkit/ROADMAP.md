# Rust + TDD Learning Track: crc-hash-toolkit

**상태:** scaffold 완료 (2026-08-17), Mission 00 진행 예정
**코드 스파인:** 이 디렉토리 (`Cargo.toml` 루트)
**학습 자료:** `TDD/crc-hash-toolkit/` (이 디렉토리)

## 목적

ROM Library IR에 들어갈 핵심 primitive인 **CRC32 / SHA-1 / MD5 / SHA-256**를 직접 구현하고, Rust의
trait, iterator, error handling, test-first 사고를 익힌다. ZIP member CRC 검사, DAT lookup, content digest에
필요한 기초를 여기서 닦는다.

## 학습 목표

- CRC32 알고리즘을 table-driven으로 직접 구현 (표준 test vector로 검증)
- RustCrypto `sha2` / `md-5` crate 사용법 (SHA-256, MD5, SHA-1)
- 공통 `Hasher`/`Digest`-like trait 추출 (trait + generic 연습)
- 파일 단위 hashing (fixture 파일, `fs::read`, streaming)
- CLI 얇게 (`clap` 없이 std만으로 시작, 필요 시 추가)
- 모든 단계에서 TDD: **Red → Verify fail → Green → Verify pass**

## 진행 규칙

- 세션 길이: 60–90분
- 진도 단위: TDD 미션
- Iron Law: **테스트 없이 production code 없음.** 코드를 먼저 쓰면 지우고 다시.
- RED에서 테스트가 "틀린 이유로" 실패하면 고치고 다시. (컴파일 에러는 red가 아님 — 함수 시그니처가 없어서 못 찾는 에러는 stub을 먼저 만들어서 "assertion fail"을 보라)
- GREEN은 테스트를 통과시키는 최소 코드만. YAGNI.
- REFACTOR는 green 유지한 채로만.
- `HINTS_AND_SOLUTIONS.md`는 진지하게 시도한 후에만.
- 각 phase 완료 후: `cargo fmt && cargo clippy -- -D warnings && cargo test`

## Seam (사전 합의)

Mission 00–02: `src/lib.rs`의 공개 함수 (`crc32(&[u8]) -> u32`)
Mission 03–05: `src/lib.rs`의 공개 trait/함수 (`digest` 계열)
Mission 06: `src/main.rs` CLI (lib 호출만, 로직은 lib에)

테스트 위치: `src/lib.rs`의 `#[cfg(test)]` 모듈 (초기) → 통합 테스트는 `tests/`로 옮길 수 있음.

## Mission 목록

### Mission 00 — CRC32 직접 구현 (table-driven)

**Goal:** `crc32(bytes: &[u8]) -> u32`를 표준 CRC-32(IEEE 802.3, init 0xFFFFFFFF, final XOR 0xFFFFFFFF)로 구현.

**Rust focus:** 함수 시그니처, `&[u8]` 슬라이스, `u32` bit 연산, `for` 루프, `const` table, unit test 기본.

**RED:** 테스트 코드에서
```rust
#[test]
fn crc32_of_known_vectors() {
    assert_eq!(crc32(b""), 0x0000_0000);
    assert_eq!(crc32(b"hello"), 0x3610_A686);
    assert_eq!(crc32(b"123456789"), 0xCBF4_3926); // 표준 check value
}
```
`crc32`가 없으니 컴파일 에러가 난다. **이 상태가 red가 아니라 "에러"다.** 먼저 `pub fn crc32(_: &[u8]) -> u32 { 0 }` stub을 만들어 컴파일시키고, 테스트가 *assertion fail*로 실패하는 것을 확인하라 (verify fail).

**Green direction:** 256-entry table을 미리 계산하거나 하드코딩하고, `crc = !crc` → 각 byte마다 `crc = (crc >> 8) ^ TABLE[((crc ^ byte) & 0xFF)]` → 마지막에 `!crc`. 

**Compiler traps:**
- `&[u8]` vs `Vec<u8>` — 함수는 `&[u8]`를 받게 해서 테스트에서 `b"hello"`를 그대로 넘길 수 있게.
- `0xFFFF_FFFF` 리터럴은 u32 범위에 들어가지만, `crc ^= ...`에서 type inference 주의.
- byte를 usize로 캐스팅해서 table index로 쓸 때 `byte as usize` 잊지 말 것.

**Verify green:** 테스트 pass 확인. `cargo test`.

**Stretch:** streaming 지원 여부 미리 생각 (Mission 02에서 파일 단위로 확장).

---

### Mission 01 — CRC32 streaming (hasher 스타일)

**Goal:** `Crc32` struct를 만들어 `update(&mut self, bytes: &[u8])` + `finalize(self) -> u32` API로 chunk 단위 갱신.

**Rust focus:** struct, `&mut self` method, 단일 책임, trait 없이 API 설계.

**RED:** 미션 00과 같은 vector를 chunk 단위(`update(b"hel")`, `update(b"lo")`)로 나눠 넣고 같은 결과가 나오는지.

**Green direction:** 내부에 `crc: u32` 상태를 두고 table update 로직을 옮김. `finalize`에서 `!self.crc`.

**Stretch:** `Default` 구현.

---

### Mission 02 — 파일 단위 CRC32 (fixture 사용)

**Goal:** `crc32_file(path: &Path) -> std::io::Result<u32>` — 작은 fixture 파일을 읽어 CRC32 계산. 이때부터 `TDD/crc-hash-toolkit/fixtures/`에 실제 샘플 파일을 만들어 쓴다.

**Rust focus:** `std::fs`, `Path`, `io::Result`, `?` operator, error propagation.

**RED:** fixture 파일(`fixtures/sample_a.bin`)을 만들고, 그 파일의 기대 CRC32를 Mission 00 함수로 미리 계산해 테스트에 literal로 넣는다. (tautological 금지 — 구현과 같은 방식으로 계산하면 안 되므로, 미리 계산한 값을 literal로.)

**Green direction:** `fs::read(path)?` 후 Mission 01의 hasher에 통째로.

**Compiler traps:**
- 테스트에서 fixture 경로를 `env!("CARGO_MANIFEST_DIR")`로 조합 (상대 경로가 cwd에 따라 깨짐).
- `?` operator는 테스트에서 `#[test]` + `-> Result<()>` 필요.

---

### Mission 03 — SHA-256 (RustCrypto sha2)

**Goal:** `sha256(bytes: &[u8]) -> [u8; 32]` — `sha2` crate를 써서 구현. (직접 구현이 아니라 crate 사용법 익히기.)

**Rust focus:** external crate 의존성, `Digest` trait, `generic_array` 결과 다루기, hex 출력.

**RED:** `sha256(b"abc")` = `ba7816bf 8f01cfea 414140de 5dae2223 b00361a3 96177a9c b410ff61 f20015ad` (NIST vector) 검증. hex로 비교하거나 byte array로 비교.

**Green direction:** `use sha2::{Sha256, Digest};` → `Sha256::digest(bytes)`.

**Stretch:** `hex` crate 없이 직접 hex 인코딩 함수 `to_hex(&[u8]) -> String`.

---

### Mission 04 — MD5, SHA-1 (md-5, sha1 crate)

**Goal:** `md5(bytes) -> [u8; 16]`, `sha1(bytes) -> [u8; 20]`. DAT lookup에서 쓰는 용례에 대비.

**Rust focus:** crate 하나 더, 같은 `Digest` trait이 여러 타입에 적용되는 generic 패턴 관찰.

**RED:** NIST vector: `md5("abc") = 90015098 3cd24fb0 d6963f7d 28e17f72`, `sha1("abc") = a9993e36 4706816a ba3e2571 7850c26c 9cd0d89d`.

**Green direction:** `Digest` trait을 generic 함수로 묶어보기 (선택).

---

### Mission 05 — 공통 trait 추출

**Goal:** `crc32`, `sha256`, `md5`, `sha1`을 하나의 generic 함수(`digest<D: Digest>()`)로 통합하거나, 자체 `Hasher` trait을 만들어 CRC32와 SHA-256이 같은 인터페이스로 쓰이게 한다.

**Rust focus:** trait 정의, trait bound, generics — Rust 학습 목표 중 핵심.

**RED:** "CRC32와 SHA-256을 같은 `Hasher` trait으로 호출할 수 있다"는 테스트.

**Green direction:** trait method만 정의, 각 타입에 구현. 필요하면 `enum DigestKind` 선택.

---

### Mission 06 — CLI (std만으로)

**Goal:** `crc-hash-toolkit <algo> <path>` 형태의 CLI. `main.rs`는 얇게, 로직은 lib.

**Rust focus:** `std::env::args`, `std::process::exit`, `main`에서 error handling, `anyhow` 도입 여부 판단.

**RED:** CLI는 통합 테스트(`tests/cli.rs`)에서 `Command::new(env!("CARGO_BIN_EXE_crc-hash-toolkit"))`로 프로세스 실행해 stdout 검증. 또는 lib 함수 `run(args: &[String]) -> Result<String>`을 먼저 테스트.

**Green direction:** `run` 함수가 args 파싱 → lib 호출 → hex 문자열 반환. `main`은 `run` 호출 + 에러 시 exit(1).

---

### Mission 07 — (선택) SHA-256 직접 구현

**Goal:** Mission 03에서 crate로 돌린 SHA-256을 직접 구현 (K table, message schedule, compression function).

**Rust focus:** bit 연산, `wrapping_add`, `rotate_left`, 배열.

**RED:** NIST vector로 다시. crate 결과와 비교하는 테스트도 가능 (oracle로 사용).

---

## 정리 예정 (완료 후)

- README (아키텍처 + TDD 여정)
- mission checklist
- 전체 quality loop 후 phase commit
