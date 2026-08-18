# crc-hash-toolkit — HINTS_AND_SOLUTIONS

> 진지하게 시도한 후에만 열 것. 각 미션은 ROADMAP.md를 먼저 읽고, RED → verify fail → GREEN → verify pass를 직접 해본 뒤에 힌트를 본다.

## Mission 00 — CRC32 table-driven

<details>
<summary>힌트 1: stub부터</summary>

```rust
pub fn crc32(_bytes: &[u8]) -> u32 {
    0
}
```
컴파일 에러(레드 아님) → 이 stub으로 assertion fail을 만드는 게 첫 단계다.
</details>

<details>
<summary>힌트 2: table 생성 vs 하드코딩</summary>

하드코딩 256개는 지루하다. `const`로 table을 만들되, 테스트에서 직접 table 전체를 검증하는 대신
"표준 vector 3개"가 통과하는 걸 목표로 하면 충분하다. 직접 생성하고 싶으면 반복문으로 만들면 된다.
</details>

<details>
<summary>해결 방향 (코드 스켈레톤)</summary>

```rust
const CRC32_TABLE: [u32; 256] = { /* 생성하거나 하드코딩 */ };

pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in bytes {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    !crc
}
```
</details>

## Mission 01 — streaming hasher

<details>
<summary>해결 방향</summary>

```rust
#[derive(Default)]
pub struct Crc32 { crc: u32 }

impl Crc32 {
    pub fn new() -> Self { Self { crc: 0xFFFF_FFFF } }
    pub fn update(&mut self, bytes: &[u8]) { /* table update */ }
    pub fn finalize(self) -> u32 { !self.crc }
}
```
</details>

## Mission 02 — 파일 단위

<details>
<summary>힌트: 경로</summary>

테스트에서 `env!("CARGO_MANIFEST_DIR")` + `"/TDD/crc-hash-toolkit/fixtures/sample_a.bin"` 또는
`concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/...")`처럼 조합하라. cwd에 의존하지 않게.
</details>

## Mission 03–04 — RustCrypto

<details>
<summary>힌트: hex 비교</summary>

`Sha256::digest(b"abc")` 결과를 `format!("{:x}", ...)`가 아니라 byte array로 직접 비교하는 게
간단하다. hex 비교가 필요하면 `hex` crate를 쓰거나 직접 인코딩 함수를 만들 것.
</details>

## Mission 05 — trait 추출

<details>
<summary>힌트: 자체 Hasher trait</summary>

```rust
pub trait Hasher {
    fn update(&mut self, bytes: &[u8]);
    fn finalize(self) -> Vec<u8>; // 또는 [u8; N] 대신 Vec
}
```
`Crc32`와 `Sha256Wrapper`에 각각 구현.
</details>

## Mission 06 — CLI

<details>
<summary>힌트: 통합 테스트</summary>

`tests/cli.rs`에서:
```rust
let out = std::process::Command::new(env!("CARGO_BIN_EXE_crc-hash-toolkit"))
    .args(["sha256", "fixture경로"])
    .output()?;
```
`CARGO_BIN_EXE_<name>`은 cargo가 통합 테스트에서만 제공하는 env다.
</details>
