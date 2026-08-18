# chip8-emulator — HINTS_AND_SOLUTIONS

> 진지하게 시도한 후에만 열 것. 각 미션은 ROADMAP.md를 먼저 읽고, RED → verify fail → GREEN → verify pass를 직접 해본 뒤에 힌트를 본다.

## Mission 04 — Fetch의 PC 증가

<details>
<summary>힌트 1: 증가는 fetch에서만</summary>

```rust
pub fn fetch(&mut self) -> u16 {
    let hi = self.memory.read(self.pc) as u16;
    let lo = self.memory.read(self.pc + 1) as u16;
    self.pc += 2;              // 여기서 정확히 한 번
    (hi << 8) | lo
}
```
execute에서 PC를 다시 증가시키면 skip 계열에서 이중 증가 버그가 난다.
</details>

## Mission 08 — DXYN의 wrap vs clip

<details>
<summary>힌트 1: 시작 좌표만 wrap</summary>

시작 `x = v[x] % 64`, `y = v[y] % 32` (또는 `& 63`, `& 31`). 그 **후**에 픽셀을 그릴 때는 각 줄이 오른쪽/아래 가장자리를 넘으면 그 줄을 중단한다. 반대편으로 이어 그리지 않는다.
</details>

<details>
<summary>힌트 2: XOR와 VF</summary>

```rust
let collision = self.display.set(x + px, y, on);
if collision { self.v[0xF] = 1; }
```
`set`은 기존 픽셀이 1이고 새 값이 1이면 true를 반환(꺼짐)하도록 만든다. `v[0xF]`는 DXYN 시작 시 0으로 초기화.
</details>

## Mission 15 — shift quirk

<details>
<summary>힌트: 원래 동작 vs 현대 동작</summary>

원래(COSMAC VIP): `v[x] = v[y]` 먼저 복사 후 `v[x] >>= 1`, 밀려난 비트는 `v[x] & 1`. 현대(CHIP-48+): v[y] 무시, `v[x]` 제자리. quirk struct로 토글:

```rust
let val = if self.quirks.shift_uses_vx { v[x] } else { v[y] };
v[x] = val >> 1;              // 8XY6
v[0xF] = val & 1;
```
</details>

## Mission 17 — FX0A 블로킹

<details>
<summary>힌트: PC를 되돌리기보다 대기 상태로</summary>

fetch가 이미 PC+2를 했으므로, 대기 중에는 `self.pc -= 2`를 하거나 `WaitingForKey` 상태를 둔다. 후자가 더 깔끔하다:

```rust
if let Some(k) = self.keypad.first_pressed() {
    v[x] = k;
} else {
    self.pc -= 2;   // 또는 상태 enum
}
```
</details>

## Mission 20 — corax89 test ROM

<details>
<summary>힌트: 통과 표시는 픽셀 패턴</summary>

test ROM은 실패한 명령을 스크린 좌측에, 통과를 우측에 표시한다. 스크린 전체를 도트 매트릭스로 덤프하고 왼쪽 열이 다 꺼져 있으면 통과. 실패 시 어떤 명령이 남았는지 좌표로 식별.

```rust
// 테스트 실패 시:
// println!("{}", display.dump());
```
</details>

## Mission 22 — Renderer trait

<details>
<summary>힌트: core는 Display만 노출</summary>

```rust
pub trait Renderer {
    fn present(&mut self, display: &Display);
}
```
SDL/softbuffer 의존성은 `main.rs` 또는 별도 crate에만. 테스트용 `RecordingRenderer`가 버퍼를 기록하도록 하면 IO 없이 통합 검증 가능.
</details>
