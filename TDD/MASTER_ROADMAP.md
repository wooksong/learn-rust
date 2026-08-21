# Learn Rust TDD - 마스터 로드맵 (utilforever 커리큘럼 통합)

> **철학**: "라이브러리 5개 만들기" → "게임 엔진 1개 만들기" → "시스템 프로그래머 되기"
> **학습 스타일**: project-first, concept-reinforced, TDD-driven — 각 개념은 프로젝트에서 자연스럽게 발생시킨 뒤 복습/퀴즈/구현으로 다진다.
> **약점 타깃**: closures, concurrency — 이 두 가지를 의도적으로 연습하게 트랙을 배치/연결한다.

---

## 난이도 척도 (L1–L5)

| 레벨 | 설명 | 예시 |
|------|------|------|
| **L1** | 문법·기초 타입, 단일 함수, 상태 없음 | `crc32` 테이블 루프, `struct` 초기화 |
| **L2** | 트레이트/제네릭 첫 적용, 파일 I/O, 에러 처리 | `Hasher` 트레이트 추출, `fs::read` |
| **L3** | 라이프타임·제로카피, 스트리밍/이벤트 파싱, 비동기 기초 | ZIP 엔트리 제로카피, HLS 파서, `tokio::time::pause` |
| **L4** | 동시성·채널·이벤트 루프, UI 상태 머신, 복잡한 트레이트 객체 | Ratatui 리듀서, 다운로더 순차 출력, ECS 스케줄링 |
| **L5** | GPU/그래픽스, 무한 월드, 분산 시스템, 프로파일링 주도 최적화 | wgpu 파이프라인, 옥트리 청크, rayon 스레드 풀 |

---

## Phase 1: 데이터 처리 라이브러리 마스터 (완료 목표: 2026-10)

> **순서 근거**: (1) 기초 → 고급으로 난이도 단조 증가, (2) 앞 트랙의 산출물(CRC/다이제스트/트리/리포지토리)을 뒤 트랙이 재사용, (3) **closures/concurrency 약점**을 Phase 1 후반에 의도적으로 건드림.

| 트랙 | 상태 | 난이도 | 도입 개념 (이 트랙에서 처음 다룸) | 전제 개념 (이미 알아야 함) | 핵심 학습 | 산출물 (TDD/<트랙>/) | 약점 연계 |
|------|------|-------|----------------------------------|---------------------------|-----------|-------------------|----------|
| crc-hash-toolkit | 🟡 진행 예정 | L1→L2 | 테이블 기반 알고리즘, `Hasher`/`Digest` 트레이트, 스트리밍 상태 머신, 외부 크레이트 의존성 관리 | 기본 문법, `&[u8]`, `Result`, `for` 루프 | 해시 알고리즘, trait, streaming | `TDD/crc-hash-toolkit/` | — |
| zip-inspector | ⏳ 대기 | L2→L3 | 파일 포맷 파싱, 제로카피 슬라이스(`&[u8]`), 라이프타임 엘리전/명시, `nom` 스타일 파서 콤비네이터 | `Hasher` 트레이트, `std::io`, `Result` 체이닝 | 파일 포맷, 제로카피 파싱 | `TDD/zip-inspector/` | **closures**: 엔트리 필터/맵에 클로저 어댑터 첫 적용 |
| fbneo-dat-curator | 🟠 설계 승인·audit 대기 | L3→L4 | `quick-xml` streaming state machine, `petgraph` typed relation, explainable ranking, TOML/Clap precedence, deterministic writer | `crc-hash-toolkit`의 digest 개념, `BufRead`, iterator, typed `Result` | FBNeo 1G1R 선택, Non-Merged relation projection, manifest provenance | `TDD/fbneo-dat-curator/` | **closures**: ranking key와 provider lookup<br>**주의**: streaming은 concurrency가 아님 |
| tree-snapshot | ⏳ 대기 | L2→L3 | 재귀적 열거형(`Box<Enum>`), 직렬화/역직렬화, 스냅샷 테스트(`insta`), 트리 diff | 트레이트 객체 기초, `serde` 기초 | 트리 직렬화, 스냅샷 테스트 | `TDD/tree-snapshot/` | — |
| sqlite-repo-practice | ⏳ 대기 | L3 | `rusqlite`, 리포지토리 패턴, 마이그레이션 버전 관리, 트레이트 객체(`Box<dyn Repo>`) | `Result` 체이닝, 제네릭, 라이프타임 | 리포지토리 패턴, 마이그레이션 | `TDD/sqlite-repo/` | **concurrency**: 동시 접근 시 `Mutex`/`RwLock` 첫 실습 |

> **Predecessor 경계:** `fbneo-dat-curator`는 사용자가 승인한 ROM Library IR 선행 학습 프로젝트다. Root의 ZMQ Topic Monitor mission을 대체하지 않으며, 결과 package를 다른 저장소에서 영구 dependency로 소비하지 않는다. 검증된 source/test/pattern만 선택적으로 복사·수정할 수 있다.

**Phase 1 완료 기준**: 각 트랙 `cargo test && cargo clippy -D warnings` 통과 + `README.md` 문서화 + **회고(wiki_retro)에서 다음 트랙으로 넘길 개념 명시**

---

## Phase 1.5: TUI 브리지 — streamlab-downloader (Phase 1 병행 또는 직후)

> **이유**: Phase 1이 "라이브러리"에 집중돼 있어 **ZMQ Topic Monitor TUI**라는 목표 아티팩트와 거리가 멀다. `streamlab-downloader`는 **Tokio 비동기 → 채널/이벤트 리듀서 → Ratatui 대시보드**까지 전 과정을 TDD로 다뤄 **Phase 1 → TUI 프로젝트**를 매끄럽게 잇는다. Phase 1 중 `fbneo-dat-curator` 이후, 또는 Phase 1 완료 직후 별도 우선순위 검토 후 투입한다.

| 트랙 | 상태 | 난이도 | 도입 개념 | 전제 개념 | 핵심 학습 | 산출물 | 약점 직격 |
|------|------|-------|-----------|-----------|-----------|--------|-----------|
| streamlab-downloader | ⏳ 대기 | L2→L4 | `tokio` 시뮬레이션 시간, `mpsc` 채널, 재시도/백오프 정책, 이벤트 리듀서 패턴, Ratatui 위젯/레이아웃/스냅샷 | `Hasher` 트레이트, 파일 I/O, 기본 비동기(`async`/`await`), `anyhow` | HLS 파싱, URL 리졸브, 목 HTTP, 동시 다운로드 순서 보장, Ratatui 대시보드 | `TDD/streamlab-downloader/` | **closures**: 리듀서/이벤트 핸들러 전역 클로저 사용<br>**concurrency**: `tokio::spawn` + 채널 순차 출력, 시뮬레이션 시간으로 결정론 테스트 |

> **핵심 차이**: Phase 1은 "동기 라이브러리", Phase 1.5는 "비동기 애플리케이션". TUI 프로젝트(ZMQ 구독 → 이벤트 루프 → Ratatui)와 **아키텍처가 거의 동일**하다.

---

## Phase 2: 그래픽스/엔진 기초 (시작: Phase 1 완료 후)

> **주의**: Phase 1 → Phase 2는 **난이도 절벽(L3 → L5)**이다. `crc-hash-toolkit` → `wgpu` 배치 렌더링 사이에 중간 단계가 없다. **Phase 1.5(streamlab-downloader)로 `winit`/이벤트 루프/비동기 감을 익히고 나서 진입**할 것. 아래 순서는 utilforever 커리큘럼 순서를 따른다.

| 트랙 | utilforever 매핑 | 난이도 | 도입 개념 | 전제 개념 | 핵심 학습 | 산출물 통합 대상 |
|------|------------------|-------|-----------|-----------|-----------|-----------------|
| rust-fundamentals | Week 1-6 (보강) | L2→L3 | 소유권/이동/빌림 재정립, 제네릭/트레이트 바운드, 라이프타임 엘리전/명시, **동시성 기초(`std::thread`, `Mutex`, `Arc`)** | Phase 1 전 트랙 완료 수준 | 소유권, 제네릭, 라이프타임, 동시성 기초 | `minecraft-clone/core` |
| graphics-pipeline | Week 7-11 | L4→L5 | `wgpu` 표면/장치/큐, 셰이더(WGSL), 파이프라인 레이아웃, 바인드 그룹, 배치 렌더링, 텍스처 아틀라스 | 선형 대수 기초(벡터/행렬), `winit` 이벤트 루프(Phase 1.5에서 익힘) | wgpu, 셰이더, 배치 렌더링, 텍스처 아틀라스 | `minecraft-clone/renderer` |
| chunk-system | Week 12-16 | L5 | 옥트리/청크 분할, 면 컬링(이웃 검사), 심플렉스/퍼린 노이즈, VBO/인덱스 버퍼 최적화, LOD | `wgpu` 버퍼/텍스처, 배치 렌더링, 난수 생성 | 옥트리, 면 컬링, 심플렉스 노이즈, VBO 최적화 | `minecraft-clone/world` |

**코드 스파인**: 단일 프로젝트 `TDD/minecraft-clone/`로 통합
- 각 트랙 완료 시 해당 모듈이 `minecraft-clone`에 머지됨
- **TDD**: 렌더러는 픽셀 스냅샷 테스트(`image` 크레이트 비교), 청크는 결정론적 생성 테스트

---

## Phase 3: 시스템 프로그래밍 마스터 (Phase 2 완료 후)

| 트랙 | utilforever 매핑 | 난이도 | 도입 개념 | 전제 개념 | 핵심 학습 | 산출물 |
|------|------------------|-------|-----------|-----------|-----------|--------|
| performance-optimization | Week 17-20 | L4→L5 | `perf`/`flamegraph`, `criterion` 벤치마크, 핫 패스 자료구조 교체(`ahash`, `slotmap`), SIMD/벡터화 | `minecraft-clone` 전체 파이프라인 이해 | 프로파일링, 자료구조 교체, 벤치마크 | `minecraft-clone` 최적화 + `perf-toolkit/` |
| ecs-architecture | Week 21-25 | L4 | `bevy_ecs` 또는 자체 Archetype ECS, 시스템 스케줄링(읽기/쓰기 충돌 분석), 쿼리 필터 | 트레이트 객체, 제네릭, 동시성 기초 | bevy_ecs/자체 ECS, 시스템 스케줄링, Archetype | `TDD/ecs-framework/` (추출) |
| parallel-world | Week 26+ | L5 | `rayon` 병렬 이터레이터, 스레드 풀 튜닝, 무한 월드 청크 스트리밍, 클라/서버 상태 동기화 | ECS 아키텍처, `Arc`/`RwLock`, 직렬화(`bincode`/`postcard`) | rayon, 스레드 풀, 무한 월드, 클라/서버 분리 | `minecraft-clone` 멀티스레드 + `game-server/` |

**최종 포트폴리오**:
1. `minecraft-clone` — 풀스택 게임 엔진 클론 (렌더러 + 월드 + ECS + 네트워크)
2. `ecs-framework` — 경량 ECS 라이브러리 (crates.io publish 후보)
3. `perf-toolkit` — Rust 성능 분석 도구 모음

---

## 진행 규칙 (전체 공통)

- **세션**: 60-90분, 단위 = 1 미션
- **Iron Law**: 테스트 없이 프로덕션 코드 없음
- **Quality Gate**: 각 미션 완료 시 `cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test && cargo bench` (bench 있는 경우)
- **문서화**: 각 트랙 `ROADMAP.md` + `HINTS_AND_SOLUTIONS.md` + `README.md`
- **회고**: 트랙 완료 시 `wiki_retro`로 인사이트 저장 — **반드시 "다음 트랙에 넘길 개념/트랩" 기록**

---

## utilforever 참고 자료

- `reference/unist-rust-minecraft/` (git submodule) — 커리큘럼 원본
- `reference/focustime/` (git submodule) — 프로덕션급 TUI/CLI 아키텍처 패턴 (이벤트 리듀서, Ratatui 통합)
- `reference/minecraft-rs/` — 렌더링/청크 구현 참조용

---

## 학습 순서/난이도 설계 근거 (요약)

1. **Phase 1 내부 순서** = **개념 누적 + 선택적 promotion + 난이도 증가**
   `crc-hash-toolkit`의 digest·streaming state → `zip-inspector`의 read-only binary inspection → `fbneo-dat-curator`의 XML state machine·typed graph·policy projection → `tree-snapshot`의 구조 diff → `sqlite-repo`의 영속 저장소로 확장한다. Artifact를 억지로 crate dependency로 연결하지 않고 검증된 source/test/pattern만 transplant한다.

2. **약점(closures, concurrency) 배치**
   - Phase 1: `zip-inspector`의 filter/map과 `fbneo-dat-curator`의 ranking key/provider lookup으로 closure를 점진적으로 연습한다. XML streaming을 concurrency라고 부르지 않는다.
   - `sqlite-repo`의 동시 접근은 `Mutex`/`RwLock`의 첫 노출이다.
   - Phase 1.5: `streamlab-downloader`에서 채널, task와 결정론적 async test를 본격 연습한다.
   - Phase 2 `rust-fundamentals` 보강 세션에서 `std::thread`/`Arc`/`Mutex` 패턴을 정리한다.

3. **Phase 1 → Phase 2 절벽 완화**
   `streamlab-downloader`는 비동기 이벤트 처리와 Ratatui 상태 모델을 다룬다. 그래픽스 진입 전에 실제 `winit` 경험이 필요하면 별도 spike로 검증하며, StreamLab이 `winit`을 가르친다고 간주하지 않는다.

4. **Phase 2 → Phase 3 자연 연결**

   `minecraft-clone`이 이미 렌더러+월드를 가지므로, `performance-optimization`은 실측 프로파일링 대상이 되고, `ecs-architecture`는 기존 월드를 ECS로 리팩터링하며, `parallel-world`는 멀티스레드 청크 생성을 얹는다. 각 트랙이 이전 산출물을 직접 개선하는 **코드 스파인 진화** 형태.

---

## 결정 기록

- 2026-08-17: Phase 2/3 트랙 6개 추가 결정 (utilforever 커리큘럼 흡수)
- 2026-08-17: `practice/` 폴더를 `TDD/<트랙>/`로 통합 — 코드 스파인(Cargo package)과 학습 자료를 한 디렉토리에
- 2026-08-18: **난이도 척도(L1–L5), 도입/전제 개념 컬럼, Phase 1.5(streamlab-downloader) 신설, 약점 타깃 매핑, 학습 순서 설계 근거 섹션 추가** — 로드맵을 "범위 나열"에서 "학습 순서 설계도"로 격상
- 2026-08-21: `dat-xml-parser`를 통합형 `fbneo-dat-curator` predecessor로 교체 — 1G1R Non-Merged DAT + manifest, `petgraph`, configurable policy, source-level promotion과 구현 전 audit gate를 명시
- Phase 1 완료 전까지 Phase 2 스캐폴딩 하지 않음 (집중 유지)
- 각 Phase 전환 시 마스터 로드맵 재검토
