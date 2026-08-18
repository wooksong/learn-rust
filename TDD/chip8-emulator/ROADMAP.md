# Rust + TDD Learning Track: chip8-emulator

**상태:** scaffold 완료 (2026-08-17), 시작 시점 미정 (someday task)
**코드 스파인:** 이 디렉토리 (`Cargo.toml` 루트)
**학습 자료:** `TDD/chip8-emulator/` (이 디렉토리, 코드 스파인 포함)
**개념 소스:** `../100-exercises-to-learn-rust` (JIT 개념 참조), `../Rustlings` (보조)

## 목적

[Tvil 블로그의 CHIP-8 가이드](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)를 따라
**CHIP-8 인터프리터**를 Rust로 TDD로 구현한다. 에뮬레이터 개발의 입문(사실은 interpreter)이면서,
fetch/decode/execute 루프, 명령 디코딩, 타이밍, 스프라이트 렌더링, quirk 처리라는 다른 트랙에 없는
도메인을 다룬다. 자세한 배경은 `~/Work/Research/retro-emulator-development/README.md` 참조.

## 학습 목표

- 인터프리터 vs 에뮬레이터의 경계를 이해하고, CHIP-8 명령 집합 전체를 구현
- fetch/decode/execute 루프를 소유권에 맞게 구조화 (`Chip8` struct + `&mut self`)
- 중첩 `match` 디코딩, nibble 필드(X/Y/N/NN/NNN) 추출과 레지스터 조회 구분
- 표준 test ROM(IBM 로고 → corax89 chip8-test-rom)을 수용 테스트로 사용
- quirk(구현체마다 다른 동작)를 설정으로 토글하는 설계
- IO와 분리된 testable 코어 (renderer trait, 입력은 trait 뒤로)
- 모든 단계에서 TDD: **Red → Verify fail → Green → Refactor**

## 진행 규칙

- 세션 길이: 60–90분
- 진도 단위: TDD 미션 (Phase 단위로 커밋)
- Iron Law: **테스트 없이 production code 없음.** 코드를 먼저 쓰면 지우고 다시.
- RED에서 테스트가 "틀린 이유로" 실패하면 고치고 다시. (컴파일 에러는 red가 아님 — stub을 먼저 만들어 "assertion fail"을 보라)
- GREEN은 테스트를 통과시키는 최소 코드만. YAGNI.
- REFACTOR는 green 유지한 채로만.
- `HINTS_AND_SOLUTIONS.md`는 진지하게 시도한 후에만.
- 각 phase 완료 후: `cargo fmt && cargo clippy -- -D warnings && cargo test`

## Seam (사전 합의)

- Mission 00: `src/lib.rs`(코어) + `src/main.rs`(thin) 분할
- Mission 01–19: `src/lib.rs`의 공개 타입/함수 — `Chip8`, `Memory`, `Display`, `Keypad`
- Mission 22–24: renderer trait — 코어는 디스플레이 구현체(SDL/softbuffer)에 비의존
- Mission 25+: `src/main.rs` CLI (lib 호출만, 로직은 lib에)

테스트 위치: `src/*.rs`의 `#[cfg(test)]` 모듈 (초기) → 통합 테스트는 `tests/`로.

## Architecture target

```text
./
  src/
    main.rs       # CLI: <rom path> 로드 → 실행 루프 (thin)
    lib.rs        # public exports: Chip8
    cpu.rs        # V[16], PC, I, stack, delay/sound timer
    memory.rs     # 4KB RAM, 0x200부터 ROM, 0x050–0x09F 폰트
    display.rs    # 64×32 framebuffer, XOR 스프라이트, clipping, VF
    keypad.rs     # 16키 상태 (trait 뒤로 숨김)
    opcode.rs     # decode + execute dispatch (nested match)
  tests/
    ...
```

## Mission 목록

### Phase 0 — 프로젝트 골격과 TDD 워크플로 (2 sessions)

#### Mission 00 — lib.rs + thin main.rs 분할

**Goal:** Cargo 프로젝트가 `lib.rs`(코어)와 `main.rs`(호출만)로 분리되고 테스트가 돈다.

**Rust focus:** crate 구조, `lib.rs`와 `main.rs` 분리, `#[cfg(test)]` 기본.

**RED:** `tests`에서 lib의 공개 함수를 호출하는 테스트 (`assert_eq!(chip8_version(), "chip8-emulator")` 등 최소 1개).

**Green direction:** `pub fn chip8_version() -> &'static str` stub. `main.rs`는 `fn main() {}`만.

**Refactor:** `cargo fmt && cargo clippy -D warnings` 통과 확인.

**Compiler traps:** 없는 lib를 `use`하면 컴파일 에러 — stub부터.

**Stretch:** `cargo nextest` 사용해 보기.

#### Mission 01 — Memory: 4KB RAM + ROM 로딩

**Goal:** `Memory` 타입: 4KB RAM, `load_rom(&[u8])`은 0x200부터 적재, 나머지는 0.

**Rust focus:** `[u8; 4096]` 또는 `Vec<u8>` 선택, 인덱스 범위, `&[u8]` 슬라이스.

**RED:** 5바이트 ROM을 적재한 뒤 `mem[0x200..0x205]` 값 확인, 0x000–0x1FF이 0인지 확인, 4KB 초과 ROM은 `Err` 또는 클램프.

**Green direction:** `struct Memory { ram: Vec<u8> }` + `fn load_rom(&mut self, rom: &[u8]) -> Result<(), RomTooBig>`.

**Compiler traps:** 범위 초과 인덱스는 panic — `get()` 또는 길이 검사를 테스트에 활용.

**Stretch:** `rom_too_big` 테스트 추가.

#### Mission 02 — 폰트 로딩

**Goal:** 16개 hex digit 스프라이트(4×5, 각 5바이트)를 0x050–0x09F에 적재.

**Rust focus:** `const` 배열, 상수 데이터, `copy_from_slice`.

**RED:** `Memory::new()` 후 0x050 위치부터 5바이트가 0xF0,0x90,0x90,0x90,0xF0('0' 모양)인지, 'F' 위치까지 총 80바이트 검증.

**Green direction:** `const FONT: [u8; 80]` + `load_font()`에서 `copy_from_slice`.

**Compiler traps:** const 배열 타입 명시(`[u8; 80]`) — 리터럴 크기 추론 주의.

**Stretch:** `font_address(digit: u8) -> usize` 헬퍼 (FX29에서 재사용).

### Phase 1 — CPU 상태와 fetch (2 sessions)

#### Mission 03 — Chip8 상태 초기화

**Goal:** `Chip8` struct: `v: [u8; 16]`, `pc: u16`(0x200), `i: u16`, `stack: Vec<u16>`, `delay: u8`, `sound: u8`, `memory: Memory`, `display: Display`.

**Rust focus:** struct, `Default`, `new()` 관례.

**RED:** `Chip8::new()` 후 각 필드 초기값 검증 (pc=0x200, v 전부 0, stack 비어 있음).

**Green direction:** 필드 private + `new()`만 공개. `Default` derive는 불가(private 필드) — 수동 impl.

**Compiler traps:** private 필드에 대한 통합 테스트 접근 — `#[cfg(test)]` 모듈 또는 getter.

**Stretch:** `Display::new()`도 64×32 전부 0으로 초기화.

#### Mission 04 — Fetch: PC에서 2바이트 읽고 PC += 2

**Goal:** `fetch() -> u16`: PC 위치 2바이트를 big-endian으로 합치고, **PC를 정확히 한 번 증가**(fetch에서). execute에서 다시 증가하지 않음.

**Rust focus:** byte 합치기(`(hi as u16) << 8 | lo`), `&mut self` 메서드.

**RED:** ROM에 `0x12, 0x34` 적재 → `fetch()` = 0x1234, 그 후 `pc == 0x202`.

**Green direction:** `fn fetch(&mut self) -> u16 { let ins = ...; self.pc += 2; ins }`.

**Compiler traps:** endianness — 리틀엔디언 read는 금지. byte 단위로 직접.

**Stretch:** fetch를 두 번 연속 호출해 연속된 명령 읽기 확인.

#### Mission 05 — Opcode 필드 추출

**Goal:** 16-bit 명령에서 `x = (ins >> 8) & 0x0F`, `y = (ins >> 4) & 0x0F`, `n = ins & 0x0F`, `nn = ins & 0xFF`, `nnn = ins & 0xFFF` 추출 함수.

**Rust focus:** bit shift/mask, tuple 또는 struct 반환.

**RED:** `0x8XY4`류 여러 샘플로 각 필드 값 검증. **레지스터 조회는 여기서 하지 않음** — X/Y는 이후 execute에서 `v[x]`로.

**Compiler traps:** `>> 8` 후 mask 순서, u16 타입 유지.

**Stretch:** `DecodedFields` struct + `From<u16>`.

### Phase 2 — 기초 opcode와 IBM 로고 마일스톤 (3 sessions)

#### Mission 06 — 00E0 clear, 1NNN jump

**Goal:** `00E0` 디스플레이 클리어, `1NNN` PC = NNN (jump, 이후 증가 없음).

**Rust focus:** `match` 첫 문법 도입.

**RED:** 명령 2개를 차례로 실행하는 시나리오 테스트.

**Green direction:** `fn execute(&mut self, ins: u16)`에서 `match ins >> 12` … 단, `00E0`는 상위 nibble 0인데 0000/00EE와 구분.

**Compiler traps:** `1NNN`에서 fetch가 이미 PC+2 했으므로 **여기서 PC를 NNN으로 덮어쓰기** — 추가 증가 금지.

**Stretch:** unknown opcode는 `Err` 반환 설계 시작 (`Result<(), Chip8Error>`).

#### Mission 07 — 6XNN set, 7XNN add

**Goal:** `6XNN`: `v[x] = nn`. `7XNN`: `v[x] += nn` (carry 없음, VF 불변).

**Rust focus:** 배열 인덱스, `wrapping_add`.

**RED:** set/add 후 레지스터 값, 7XNN에서 255 초과 시 wrap 검증 (VF 안 바뀜).

**Green direction:** `v[x as usize] = nn` 등.

**Compiler traps:** `x`를 인덱스로 쓸 때 `as usize` — 배열 인덱스는 usize.

**Stretch:** `7XNN` overflow wrap 테스트.

#### Mission 08 — ANNN set index, DXYN display

**Goal:** `ANNN`: `i = nnn`. `DXYN`: I 주소부터 N바이트 스프라이트를 (v[x], v[y])에 XOR로 그림, 좌표는 시작점만 wrap, 가장자리는 clip, 지워진 픽셀이 있으면 VF=1.

**Rust focus:** 2D 인덱스(`y * 64 + x`), XOR, 경계 조건 — **가장 어려운 명령**.

**RED:** 3개 시나리오: (a) 단순 그리기, (b) 겹쳐서 XOR로 지워짐 + VF=1, (c) 가장자리 clip(반대편 재등장 없음).

**Green direction:** `display.set(x, y, on)` 내부 `buffer[y*64+x] ^= on`; N줄 루프에서 x는 modulo 64, y는 modulo 32로 시작하고, 각 줄은 가장자리에서 중단.

**Compiler traps:** "시작 좌표 wrap vs 드로잉 clip" 구분이 관건 — 시작점만 모듈로, 픽셀은 경계에서 멈춤.

**Stretch:** 스프라이트가 VF flag를 바꾸는 조건을 별도 테스트로 문서화.

#### Mission 09 — IBM 로고 통합 테스트

**Goal:** IBM 로고 프로그램(공개 ROM)을 로드해 제한된 스텝 수 실행 후 display에 로고 패턴이 그려졌는지 검증. **첫 마일스톤.**

**Rust focus:** 통합 테스트(`tests/`), 외부 ROM fixture 사용, `CARGO_MANIFEST_DIR` 경로.

**RED:** IBM 로고 ROM을 `fixtures/ibm_logo.ch8`로 두고, 실행 후 일부 픽셀 좌표가 켜져 있는지 검증 (전체 해시보다 좌표 몇 개).

**Green direction:** `load_rom` → 특정 횟수만큼 `fetch`+`execute` 반복하는 헬퍼 → display 상태 확인.

**Compiler traps:** 무한 루프 프로그램 — 스텝 횟수 제한 필수. ROM fixture는 Git에 커밋 가능한 크기(수십 바이트).

**Stretch:** display 전체를 문자열로 덤프하는 디버그 출력(도트 매트릭스) — 이후 M26에서 재사용.

### Phase 3 — 제어 흐름 (2 sessions)

#### Mission 10 — 2NNN call, 00EE return

**Goal:** `2NNN`: pc를 stack에 push 후 NNN으로. `00EE`: pop해서 pc로.

**Rust focus:** `Vec::push/pop`, stack overflow 방어.

**RED:** 중첩 call 2회 후 return 2회로 복귀 시나리오.

**Green direction:** `self.stack.push(self.pc); self.pc = nnn;` return은 `self.pc = self.stack.pop().ok_or(...)?`.

**Compiler traps:** stack underflow 시 panic 대신 `Err` — `Chip8Error::StackUnderflow`.

**Stretch:** 깊은 재귀 프로그램으로 overflow 테스트.

#### Mission 11 — 3XNN/4XNN/5XY0/9XY0 skip

**Goal:** 조건 참이면 **PC를 2 더 증가**(다음 2바이트 명령 skip).

**Rust focus:** fetch의 PC 증가와의 상호작용 — 여기서 `pc += 2` (fetch에서 이미 +2 했으므로 총 +4).

**RED:** `3XNN` 참/거짓 두 경로 검증 — 참이면 다음 명령을 건너뛰어 그 다음이 실행됨.

**Green direction:** `if v[x] == nn { self.pc += 2 }`.

**Compiler traps:** "skip = fetch의 +2 + 여기서 +2" — 총 4바이트 이동인지 시나리오 테스트로 고정.

**Stretch:** `5XY0`/`9XY0`도 같은 패턴으로.

#### Mission 12 — BNNN jump with offset (quirk)

**Goal:** `BNNN`: pc = nnn + v[0] (COSMAC VIP 원래 동작). CHIP-48 이후는 BXNN(v[x]) — **quirk 설정으로 토글**.

**Rust focus:** 설정 struct (`Quirks { bnnn_uses_vx: bool }`), 문서화된 비호환성.

**RED:** 기본값(원래 동작) 테스트 + quirk 켰을 때 동작 테스트.

**Green direction:** `let offset = if self.quirks.bnnn_uses_vx { v[x] } else { v[0] };`.

**Compiler traps:** 설정을 나중에 추가하면 테스트 깨짐 — 처음부터 `Quirks` struct 도입.

**Stretch:** Spacefight 2091 같은 의존 게임 사례 주석.

### Phase 4 — 산술·논리 (3 sessions)

#### Mission 13 — 8XY0–8XY3 (set/OR/AND/XOR)

**Goal:** `8XY0` vx=vy, `8XY1` OR, `8XY2` AND, `8XY3` XOR. VF 영향 없음(구현별 정의되지 않음 — 문서화).

**Rust focus:** bit 연산 반복, 테이블 테스트.

**RED:** 각 opcode별 vector 여러 개.

**Green direction:** `match ins & 0x000F` 내부 중첩.

**Compiler traps:** 8XY0–3은 VF를 건드리지 않음 — 함부로 0으로 만들지 말 것 (정의되지 않음).

**Stretch:** `tests`를 `rstest` 파라미터로.

#### Mission 14 — 8XY4 add carry, 8XY5/8XY7 sub borrow

**Goal:** `8XY4`: vx += vy, overflow면 VF=1 (아니면 0). `8XY5`: vx -= vy, **빌림 없으면** VF=1. `8XY7`: vx = vy - vx, 같은 VF 규칙.

**Rust focus:** `overflowing_add/sub`, flag 레지스터 관례.

**RED:** overflow/borrow 양쪽 경계값(0xFF±1) 테스트.

**Green direction:** `let (r, carry) = v[x].overflowing_add(v[y]); v[x] = r; v[0xF] = carry as u8;`.

**Compiler traps:** 뺄셈 VF는 "빌림 없음=1" — 직관과 반대, 주석 명시.

**Stretch:** 8XY7의 vx/vy 방향 혼동 방지 테스트 이름에 명시.

#### Mission 15 — 8XY6/8XYE shift (quirk)

**Goal:** 원래: vx = vy 후 shift, 밀려난 비트를 VF에. CHIP-48 이후: vx를 제자리 shift. **quirk 토글.**

**Rust focus:** shift, 비트 검사, quirk 패턴 재사용.

**RED:** 두 동작 모두 테스트 (quirk off/on).

**Green direction:** `let (val, shifted) = if quirk { (v[x], ...) } else { (v[y], ...) }`.

**Compiler traps:** `>>`는 논리 shift — 음수 없음. 밀려난 비트: `val & 1`.

**Stretch:** 시프트 후 VF 값이 "밀려난 비트"인지 테스트.

### Phase 5 — 타이머·키·기타 opcode (3 sessions)

#### Mission 16 — FX07/FX15/FX18 타이머 + 60Hz tick

**Goal:** `FX07` vx=delay, `FX15` delay=vx, `FX18` sound=vx. `tick()` 메서드: delay/sound가 0이 아니면 1 감소 — **fetch 루프 속도와 독립** (60Hz 기준 별도 호출).

**Rust focus:** 시간 추상화, 테스트 가능한 tick 설계.

**RED:** tick() 1회 감소, 0이면 유지, FX07/15/18 전달.

**Green direction:** `fn tick(&mut self)` 별도 — 루프에서 호출 주기 분리. (실제 60Hz는 M24에서 타이밍으로.)

**Compiler traps:** 타이머를 fetch마다 감소시키면 명령 속도에 비례 — 반드시 별도 tick.

**Stretch:** sound 타이머는 "beep" 상태를 읽는 getter만.

#### Mission 17 — EX9E/EXA1 skip-if-key, FX0A wait-key

**Goal:** `EX9E`: vx 키가 눌렸으면 skip. `EXA1`: 안 눌렸으면 skip. `FX0A`: 키 입력까지 **block** (PC 증가 안 함), 눌린 키의 hex 값을 vx에.

**Rust focus:** `Keypad` 타입, 블로킹 시맨틱 테스트(가짜 입력 주입).

**RED:** keypad 상태를 주입 가능하게(`set_key(key, pressed)`), FX0A는 초기 PC 유지 검증.

**Green direction:** `Keypad { keys: [bool; 16] }`. FX0A는 `if !pressed { return Ok(()) }` — pc 유지, 상태만 대기.

**Compiler traps:** FX0A의 PC 비증가 — fetch에서 이미 +2 했으므로 여기서 -2 하거나, 대기 상태 enum으로 처리.

**Stretch:** 대기 상태를 `WaitingForKey` enum으로 명시.

#### Mission 18 — FX29 폰트, FX33 BCD, FX55/FX65 store/load (quirk)

**Goal:** `FX29`: i = 폰트 주소(vx 마지막 nibble). `FX33`: vx를 BCD 3자리로 i, i+1, i+2에. `FX55`: v0..vx를 i부터 저장. `FX65`: i부터 로드. **FX55/FX65의 I 증가는 quirk** (원래: 증가, 현대: 유지).

**Rust focus:** BCD 변환(div/mod), quirk 패턴.

**RED:** `156 → 1,5,6` 검증, FX55/FX65 왕복, I 유지 여부 quirk 테스트.

**Green direction:** `let (h, rest) = (v / 100, v % 100); ...`.

**Compiler traps:** FX33에서 자리수 — u8이므로 0..255, 3자리 고정.

**Stretch:** FX55에서 x=0이면 v0만.

### Phase 6 — 랜덤과 테스트 ROM 수용 (2 sessions)

#### Mission 19 — CXNN random & mask

**Goal:** `CXNN`: 난수 & nn → vx.

**Rust focus:** `rand` crate 또는 자체 난수, 테스트 가능성(RNG 주입).

**RED:** `rng`를 주입 가능하게 하고(seed 고정), 결과가 nn 마스크 안에 들어가는지 + 결정적 테스트.

**Green direction:** `Rng: FnMut() -> u8`를 `Chip8::new_with_rng(rng)`로 주입. 기본은 `rand` 또는 간단한 xorshift.

**Compiler traps:** 테스트에서 실제 난수는 비결정적 — 반드시 주입.

**Stretch:** xorshift 자체 구현 (의존성 없음).

#### Mission 20 — corax89 chip8-test-rom 수용 테스트

**Goal:** [corax89/chip8-test-rom](https://github.com/corax89/chip8-test-rom)을 `fixtures/`에 두고, 실행 후 스크린에 통과 상태가 그려지는지 검증. **두 번째 마일스톤.**

**Rust focus:** 외부 fixture, 스크린 상태 어서션, 실패 시 픽셀 덤프.

**RED:** test ROM 실행 → display에 특정 픽셀 패턴(통과 표시) 확인.

**Green direction:** M09의 스텝 헬퍼 재사용 + `display.dump()` 도트 매트릭스로 실패 원인 진단.

**Compiler traps:** quirk 설정이 테스트 기대와 일치해야 함 — 현대 동작 기준으로 기본값 확정.

**Stretch:** 각 실패 명령이 어떤 quirk인지 주석.

#### Mission 21 — (선택) BC_test 수용

**Goal:** BonCoder BC_test도 통과. 동일 패턴, `fixtures/bc_test.ch8`.

**Rust focus:** 수용 테스트 확장.

**RED/Green:** M20과 동일 구조.

**Stretch:** 두 test ROM을 CI 스크립트로 묶기.

### Phase 7 — 디스플레이·입력 통합 (IO 분리) (3 sessions)

#### Mission 22 — Renderer trait (코어는 IO 비의존)

**Goal:** `trait Renderer { fn present(&mut self, framebuffer: &Display); }` — core는 SDL/softbuffer 몰라도 됨. 가짜 renderer로 통합 테스트.

**Rust focus:** trait, 의존성 역전.

**RED:** 가짜 `RecordingRenderer`가 DXYN 실행 후 프레임버퍼를 받았는지 검증.

**Green direction:** `Chip8::run_with_renderer(&mut self, renderer)` 또는 프레임 단위 `render_to(&mut self, r)`.

**Compiler traps:** trait object(`Box<dyn Renderer>`) vs generic — 초기엔 generic으로.

**Stretch:** `trait Input`도 같은 방식으로 도입 가능성 확인.

#### Mission 23 — softbuffer/pixels + winit 통합

**Goal:** 실제 창 표시: `softbuffer` + `winit` (또는 SDL2) — 64×32을 창 크기로 스케일, 키보드 16키 매핑(1-4/Q-R/A-F/Z-V).

**Rust focus:** 외부 crate, 이벤트 루프, 픽셀 버퍼 변환.

**RED:** (통합은 수동 확인 중심) — 최소한 스케일 함수는 단위 테스트.

**Green direction:** `main.rs`에서 winit 이벤트 루프 + `render_to`.

**Compiler traps:** DPI/스케일, 창 크기 vs 논리 64×32 — 배율 정수 고정.

**Stretch:** 키보드 레이아웃 scancode 사용 (AZERTY 대응).

#### Mission 24 — 타이밍 루프 (~700 instr/s, 설정)

**Goal:** 실측 기반 속도: 기본 ~700 명령/초, `--speed` 설정. tick 60Hz 분리 실행.

**Rust focus:** `std::time::Instant`, 루프 타이밍, sleep.

**RED:** 타이밍은 결정적 테스트 어려움 — 최소한 tick 주기 계산 함수만 테스트.

**Green direction:** 루프에서 `elapsed` 측정 → 남은 시간 sleep → 60Hz마다 `tick()`.

**Compiler traps:** 타이머와 명령 속도의 결합 — 게임별 기대 속도가 다름을 README에.

**Stretch:** `--speed <n>` 파싱.

### Phase 8 — CLI와 디버깅 (2 sessions)

#### Mission 25 — CLI: `chip8-emulator <rom>`

**Goal:** thin main: args 파싱, ROM 로드, 에러 시 친절한 메시지 + exit(1).

**Rust focus:** `std::env::args`, `anyhow` 도입 여부 판단, lib는 순수.

**RED:** lib의 `run(args: &[String]) -> Result<(), Chip8Error>` 형태를 통합 테스트로.

**Green direction:** main은 `run()` 호출 + 매핑만.

**Compiler traps:** 경로 처리, `CARGO_BIN_EXE` 통합 테스트.

**Stretch:** `clap` 도입 검토 (std만으로 시작).

#### Mission 26 — 디버그: step, dump, unknown opcode

**Goal:** `--debug` 모드: 명령별 step, register/memory 덤프, unknown opcode는 panic 대신 에러 + 메시지.

**Rust focus:** 로깅, `Display` impl, 에러 변형.

**RED:** unknown opcode(`0x0001` 등) 실행 시 `Chip8Error::UnknownOpcode` 반환 테스트 + 덤프 문자열 형식 테스트.

**Green direction:** `Display`/`Debug` impl for `Chip8`, `dump()` 메서드.

**Compiler traps:** display 덤프를 CLI 출력에 그대로 쓰면 화면 오염 — stderr로.

**Stretch:** step 모드에서 키 입력 대기.

#### Mission 27 — README + mission checklist + quality loop

**Goal:** README(아키텍처 + TDD 여정), mission checklist, `cargo fmt && clippy && test` 전패스, phase commit.

**Rust focus:** 문서화, 릴리즈 준비.

**RED:** — (문서 미션)

**Green direction:** README 작성, 예시 스크린샷/터미널 캡처.

**Stretch:** GitHub Actions CI (test/clippy).

### Phase 9 — Capstone stretch (선택)

#### Mission 28 — SUPER-CHIP (128×64, scroll, quirk)

**Goal:** SUPER-CHIP: 해상도 128×64, 스크롤 명령, 큰 스프라이트, 기존 quirk 일부 변경.

**Rust focus:** 모드 전환, display 크기 파라미터화.

**RED:** SUPER-CHIP 전용 명령 테스트 + 해상도 전환.

**Green direction:** `Display`를 128×64로 확장 또는 별도 모드.

**Stretch:** Mastering Super-CHIP 가이드 참조.

#### Mission 29 — XO-CHIP 또는 Octo 게임

**Goal:** XO-CHIP(2색, 사운드, 64KB) 지원 또는 Octo로 만든 게임을 에뮬레이터에서 구동.

**Rust focus:** 확장성, 통합.

**RED:** 선택한 확장의 최소 테스트.

**Green direction:** 확장 명령 구현, CHIP-8 Archive 게임 실행 확인.

**Stretch:** 게임 하나를 직접 Octo로 작성.

## MVP 완료 정의

- [ ] IBM 로고 표시 (M09)
- [ ] corax89 chip8-test-rom 전 명령 통과 (M20)
- [ ] 실제 창에서 게임 1개 실행 (M24)
- [ ] CLI + 디버그 모드 (M25–26)
- [ ] README + 품질 루프 전패스 (M27)

## 참고 자료

- Tvil CHIP-8 가이드: `https://tobiasvl.github.io/blog/write-a-chip-8-emulator/`
- corax89/chip8-test-rom: `https://github.com/corax89/chip8-test-rom`
- Octo 어셈블러: `https://github.com/JohnEarnest/Octo`
- CHIP-8 Archive: `https://johnearnest.github.io/chip8Archive/`
- 배경 노트: `~/Work/Research/retro-emulator-development/README.md`
