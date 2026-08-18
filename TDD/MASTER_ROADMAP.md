# Learn Rust TDD - 마스터 로드맵 (utilforever 커리큘럼 통합)

> **철학**: "라이브러리 5개 만들기" → "게임 엔진 1개 만들기" → "시스템 프로그래머 되기"

---

## Phase 1: 데이터 처리 라이브러리 마스터 (완료 목표: 2026-10)

| 트랙 | 상태 | 핵심 학습 | 산출물 (TDD/<트랙>/) |
|------|------|-----------|-------------------|
| crc-hash-toolkit | 🟡 진행 예정 | 해시 알고리즘, trait, streaming | `TDD/crc-hash-toolkit/` |
| zip-inspector | ⏳ 대기 | 파일 포맷, 제로카피 파싱 | `TDD/zip-inspector/` |
| dat-xml-parser | ⏳ 대기 | 스트리밍 XML, 빠른 파싱 | `TDD/dat-xml-parser/` |
| tree-snapshot | ⏳ 대기 | 트리 직렬화, 스냅샷 테스트 | `TDD/tree-snapshot/` |
| sqlite-repo-practice | ⏳ 대기 | 리포지토리 패턴, 마이그레이션 | `TDD/sqlite-repo/` |

**완료 기준**: 각 트랙 `cargo test && cargo clippy -D warnings` 통과 + README 문서화

---

## Phase 2: 그래픽스/엔진 기초 (시작: Phase 1 완료 후)

| 트랙 | utilforever 매핑 | 핵심 학습 | 산출물 통합 대상 |
|------|------------------|-----------|-----------------|
| rust-fundamentals | Week 1-6 (보강) | 소유권, 제네릭, 라이프타임, 동시성 기초 | `minecraft-clone/core` |
| graphics-pipeline | Week 7-11 | wgpu, 셰이더, 배치 렌더링, 텍스처 아틀라스 | `minecraft-clone/renderer` |
| chunk-system | Week 12-16 | 옥트리, 면 컬링, 심플렉스 노이즈, VBO 최적화 | `minecraft-clone/world` |

**코드 스파인**: 단일 프로젝트 `TDD/minecraft-clone/`로 통합
- 각 트랙 완료 시 해당 모듈이 `minecraft-clone`에 머지됨
- **TDD**: 렌더러는 픽셀 스냅샷 테스트, 청크는 결정론적 생성 테스트

---

## Phase 3: 시스템 프로그래밍 마스터 (Phase 2 완료 후)

| 트랙 | utilforever 매핑 | 핵심 학습 | 산출물 |
|------|------------------|-----------|--------|
| performance-optimization | Week 17-20 | 프로파일링, flamegraph, 자료구조 교체, 벤치마크 | `minecraft-clone` 최적화 + `perf-toolkit/` |
| ecs-architecture | Week 21-25 | bevy_ecs/자체 ECS, 시스템 스케줄링, Archetype | `TDD/ecs-framework/` (추출) |
| parallel-world | Week 26+ | rayon, 스레드 풀, 무한 월드, 클라/서버 분리 | `minecraft-clone` 멀티스레드 + `game-server/` |

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
- **회고**: 트랙 완료 시 `wiki_retro`로 인사이트 저장

---

## utilforever 참고 자료

- `reference/unist-rust-minecraft/` (git submodule) — 커리큘럼 원본
- `reference/focustime/` (git submodule) — 프로덕션급 TUI/CLI 아키텍처 패턴
- `reference/minecraft-rs/` — 렌더링/청크 구현 참조용

---

## 결정 기록

- 2026-08-17: Phase 2/3 트랙 6개 추가 결정 (utilforever 커리큘럼 흡수)
- 2026-08-17: `practice/` 폴더를 `TDD/<트랙>/`로 통합 — 코드 스파인(Cargo package)과 학습 자료를 한 디렉토리에
- Phase 1 완료 전까지 Phase 2 스캐폴딩 하지 않음 (집중 유지)
- 각 Phase 전환 시 마스터 로드맵 재검토