# FBNeo DAT Curator — High-Cost Model Audit Brief

**Prepared:** 2026-08-21

**Audit status:** Not run

**Purpose:** 독립 predecessor의 product semantics, Rust architecture, TDD 학습 순서와 다른 프로젝트로의 promotion 전략을 구현 전에 적대적으로 검토한다.

## 1. Auditor role

당신은 다음 역할을 동시에 수행하는 senior reviewer다.

1. FBNeo/Logiqx/ROM-manager semantics reviewer
2. Rust deep-module/API reviewer
3. Practical-strict TDD curriculum reviewer
4. Repository-boundary and provenance reviewer

문서에 적힌 주장을 사실로 가정하지 말고 primary source, synthetic scenarios와 실제 대상 DAT evidence로 반증을 시도하라. 구현 code를 작성하거나 scaffold하지 말고 **계획과 track만 audit**하라.

## 2. Repository roots

Audit 실행자가 두 저장소 root를 먼저 식별해야 한다.

- `<learn-rust>`: 이 파일이 있는 저장소
- `<ideation>`: `RomLibraryIR/`가 있는 설계 저장소

Absolute user path를 report나 patch에 기록하지 않는다.

## 3. Required reading order

### Learn-rust mission and track

1. `<learn-rust>/MISSION.md`
2. `<learn-rust>/PROJECT.md`
3. `<learn-rust>/RESOURCES.md`
4. `<learn-rust>/TDD/MASTER_ROADMAP.md`
5. `<learn-rust>/TDD/fbneo-dat-curator/CONTEXT.md`
6. `<learn-rust>/TDD/fbneo-dat-curator/ROADMAP.md`

### Predecessor design and evidence

7. `<ideation>/RomLibraryIR/docs/plans/2026-08-21-fbneo-dat-curator-design.md`
8. `<ideation>/RomLibraryIR/research_fbneo-rom-set-modes_20260820.ko.md`

### ROM Library IR boundary

9. `<ideation>/RomLibraryIR/AGENTS.md`
10. `<ideation>/RomLibraryIR/README.ko.md`
11. `<ideation>/RomLibraryIR/docs/plans/2026-08-09-fbneo-first-learning-track-design.md`
12. `<ideation>/RomLibraryIR/docs/learning-strategy.md`
13. `<ideation>/RomLibraryIR/docs/stack.md`
14. `<ideation>/RomLibraryIR/docs/learning/AGENT_CONTRACT.md`

`AGENTS.md`가 요구하는 순서와 제약을 지켜라. Historical NES-first 문서를 active design으로 취급하지 마라.

## 4. Fixed user decisions

다음은 audit이 몰래 바꾸면 안 되는 user decision이다. 기술적으로 불가능하거나 위험하면 변경 대신 명시적인 blocking finding을 작성한다.

- 하나의 통합형 track: `TDD/fbneo-dat-curator`
- 기본 target: **1G1R Non-Merged**
- 결과: mode-specific DAT + selection manifest
- 실제 ROM ZIP 생성은 제외
- 기본 region ranking: Korea → US → World → Japan → Other
- TOML과 CLI에서 ranking/policy override 가능
- Precedence: CLI > TOML > built-in
- `cloneof`, `romof`, ROM `merge`는 filter/mode 결과로 projection
- 학습 package를 Cargo dependency로 소비하지 않아도 됨
- Source, test 또는 pattern을 복사·수정하여 새 프로젝트에 녹이는 promotion 허용
- RomLibraryIR와 cross-repository path dependency 금지
- 사용자가 core Rust와 behavior test를 작성하고 agent는 mission/hint/review를 제공

## 5. Reviewable decisions

다음은 evidence에 따라 바꿀 수 있다.

- `petgraph`가 적절한지, explicit map/DFS가 더 깊은 module인지
- Graph node/edge 방향과 raw evidence shape
- Driver status, bootleg/hack/prototype 기본 eligibility
- Revision/tag parser의 scope
- Sample provider projection 방식
- JSON manifest schema와 versioning
- Mission 개수와 순서
- Public module seam
- Benchmark 필요 시점
- Igir/clrmame oracle comparison 방법
- Dependency candidate 목록

## 6. Exact target semantics to verify

이 계획은 igir와 최신 clrmame에서 공통으로 설명되는 다음 Non-Merged 핵심을 의도한다.

- Selected machine의 own game ROM과 필요한 parent-game ROM은 selected set에 존재한다.
- Parent game set은 외부 runtime requirement로 남지 않는다.
- BIOS/device content는 selected game set에 복제하지 않고 external provider로 남는다.
- BIOS/device까지 각 selected set에 복제하면 Full Non-Merged/Standalone이며 다른 mode다.
- 1G1R은 playable selection 수에 관한 정책이고 dependency artifact 수에 관한 정책이 아니다.

다음 source vocabulary를 정확히 사용한다.

- machine-level: `cloneof`, `romof`, `sampleof`
- ROM-level: `merge`
- `mergeof`는 이 DAT의 field가 아니다.

관계 field는 일괄 제거하면 안 된다. Projected catalog에 target이 남는지와 content가 local/external인지에 따라 유지·제거·재지정해야 한다.

## 7. Empirical evidence snapshot

조사 당시 user-provided FBNeo DAT에서 streaming scan으로 관찰한 값이다. Auditor는 local input을 사용할 수 있을 때 재현하되 이 값을 parser constant로 제안하면 안 된다.

| Observation | Count/result |
|---|---:|
| machines | 8,570 |
| `cloneof` | 5,933 |
| `romof` | 6,351 |
| BIOS entries | 16 |
| `sampleof` | 224 |
| ROM rows | 169,063 |
| ROM rows with `merge` | 95,599 |
| `isdevice` | 0 |
| comma-separated `romof` | 0 |

추가 관찰:

- 모든 clone의 `romof`는 `cloneof`와 같았다.
- 나머지 418 `romof`는 BIOS entry를 가리켰다.
- 모든 `romof` target이 존재했다.
- Neo Geo 형태에서는 selected clone → parent game → BIOS transitive chain이 실제로 나타난다.
- Parent game row의 BIOS ROM과 clone row의 parent/BIOS ROM 모두 `merge`를 가질 수 있으므로 blanket `merge` 제거는 틀리다.

이 evidence는 한 snapshot의 사실이며 일반 Logiqx/MAME schema 전체의 사실로 확대하지 않는다.

## 8. Primary sources to consult

가능하면 최신 primary source를 직접 확인하고 URL/section/date를 audit report에 남긴다.

- FBNeo Command Line wiki (`-listinfo`/MAME XML 설명)
- Logiqx ROM Management Datafile DTD
- MAME: About ROMs and Sets
- MAME: How does MAME look for files?
- Current clrmame CLI readme: `split|full|standalone|nonmerged`
- Igir Arcade ROM Sets: `merged|split|nonmerged|fullnonmerged`
- `quick-xml` Reader/Writer docs
- `petgraph` graph/visit/algo docs
- Clap derive and validation docs
- Rust API Guidelines

Community post만으로 core semantics를 확정하지 않는다. Source 간 terminology가 다르면 dialect별로 나누어 기록한다.

## 9. Required adversarial scenarios

각 scenario에 대해 expected selected entries, external providers, `cloneof`, `romof`, `sampleof`, ROM `merge`와 manifest evidence를 표로 작성하라.

1. Independent parent, no BIOS
2. Parent selected while clones are filtered
3. Clone selected, parent game only
4. Clone selected → parent game → BIOS
5. Direct BIOS consumer selected
6. Selected machine with `sampleof` whose provider is otherwise filtered
7. Missing parent/BIOS/sample target
8. `cloneof` and `romof` cycle
9. Same CRC with conflicting size/name
10. ROM row with missing CRC or nodump status
11. Korean/US/World/Japan/Other candidates with same revision
12. Unknown region/revision tag
13. Explicit override to a normally ineligible candidate
14. Policy excludes every family candidate
15. Existing output plus a transformation/reparse failure
16. Unsupported disk/CHD input

특히 scenario 4와 6에서 projected DAT가 standard consumer에게 유효한지 primary-source 또는 oracle experiment로 검증하라. 검증할 수 없으면 추측하지 말고 blocking/open finding으로 남긴다.

## 10. Audit axes

### A. ROM semantics

- 1G1R selection, DAT projection과 physical archive mode를 다시 섞고 있지 않은가?
- Parent-game content와 BIOS/sample external requirement 구분이 정확한가?
- Selected clone의 transitive BIOS relation을 `romof` 하나로 표현하는 것이 유효한가?
- Dependency-only entry가 standard DAT consumer에서 playable로 노출되는 문제가 있는가?
- ROM name/merge/hash evidence로 provider를 찾는 규칙이 충분한가?
- Sibling union 또는 CRC-only dedup이 다시 숨어들지 않았는가?

### B. Determinism and provenance

- Input bytes + effective policy가 execution identity를 충분히 정의하는가?
- Ordering, map iteration, XML attribute order와 manifest map order가 명시되었는가?
- Failed paired DAT/manifest write가 partial commit을 만들 수 있는가?
- Raw evidence와 normalized/projected value가 모두 남는가?

### C. Rust module depth

- Parser/graph/policy/projection/writer seam이 complexity를 숨기는가, pass-through layer인가?
- `petgraph`가 domain model을 단순화하는가, `NodeIndex` plumbing을 퍼뜨리는가?
- Public API가 caller에게 XML event와 graph traversal을 노출하는가?
- Trait이 두 번째 implementation 없이 speculative하게 도입되는가?
- Raw/projected model ownership이 불필요한 clone 또는 lifetime complexity를 만드는가?

### D. TDD quality

- 각 mission의 첫 RED가 behavior failure로 작게 만들 수 있는가?
- 한 mission이 60–90분을 명백히 초과하지 않는가?
- Fixture가 한 rule만 격리하고 실제 data를 복사하지 않는가?
- Parser, policy, projection, writer, CLI와 filesystem failure가 적절한 test layer에 있는가?
- Snapshot이 semantic assertion을 대체하지 않는가?
- Local real-DAT acceptance가 normal test completion과 분리되는가?

### E. Learning fit

- Beginner-to-intermediate Rust learner지만 senior systems engineer라는 배경에 맞는가?
- Closures를 자연스럽게 연습하지만 streaming을 concurrency로 잘못 가르치지 않는가?
- Ownership/lifetime/iterator/typed error/graph concept의 선행 순서가 맞는가?
- Agent가 complete solution을 먼저 제공하지 않도록 mission이 구성되었는가?
- ZMQ Topic Monitor root mission과 이 predecessor 예외의 기회비용이 설명되는가?

### F. Reuse and promotion

- General Rust pattern과 FBNeo-specific semantics가 구분되는가?
- Source/test transplant 시 원본 commit과 license/provenance를 남길 수 있는가?
- 영구 sync나 shared crate abstraction을 섣불리 만들지 않는가?
- Promoted code가 target project의 integration test 없이 신뢰되지 않도록 gate가 있는가?

### G. Repository boundary

- RomLibraryIR M5/M6/M7 또는 first-milestone non-goal을 침범하는가?
- Curated DAT를 pinned canonical catalog와 몰래 동일시하는가?
- Cross-repository path dependency나 local absolute path가 남아 있는가?
- 실제 DAT, ROM, BIOS 또는 private inventory를 commit하도록 유도하는가?

### H. Scope and YAGNI

- Full XML round-trip, all MAME dialect, device/CHD 또는 benchmark가 필요 이상으로 들어갔는가?
- 반대로 Non-Merged validity에 꼭 필요한 behavior가 누락되었는가?
- Mission 00 전에 확정하면 안 되는 crate version/API detail이 pin되어 있는가?

## 11. Known prior-plan defects that must not return

교체 전 계획에는 다음 문제가 있었다. 새 문서에 같은 문제가 다른 이름으로 다시 나타나는지 확인한다.

- RomLibraryIR M5가 1G1R을 생성한다고 주장
- Split과 Non-Merged 의미 혼동
- Merged/full과 Full Non-Merged 혼동
- 모든 sibling ROM union
- `romof`가 comma-separated parent+BIOS라고 가정
- 대상 DAT에 device relation이 있다고 가정
- Tree-only `indextree`로 전체 graph를 모델링
- Current stable 확인 없이 stale/impossible crate version pin
- Cross-repository relative path dependency
- 실제 DAT 일부를 fixture로 commit
- Local absolute path hard-code
- Complete project implementation을 mission 문서에 제공
- Invalid CLI test type 조합
- Fixture size 설명과 assertion 불일치
- Parser streaming을 concurrency 연습이라고 잘못 표현

## 12. Required report format

Audit 결과는 다음 형식으로 작성한다.

```text
# Verdict
GO | GO WITH REQUIRED CHANGES | NO-GO

# Executive summary
5–10 bullets

# Findings
[Critical|High|Medium|Low] Short title
- Evidence/source:
- Affected document and exact section/line:
- Why it matters:
- Concrete correction:
- Blocks Mission 00: yes/no

# Scenario matrix
Scenario별 selected/external/relation/ROM outcome

# Mission audit
- keep
- split
- merge
- reorder
- remove
- missing mission

# Architecture recommendation
petgraph vs explicit DFS, module/API boundary, error surface

# Learning-fit recommendation
mission size, prerequisite gaps, root-mission opportunity cost

# Unverified assumptions
Primary source 또는 experiment가 더 필요한 항목

# Minimal doc patch plan
파일별 수정 목록. 구현 code는 작성하지 않음.
```

모든 Critical/High finding은 source evidence나 재현 가능한 counterexample을 포함해야 한다. 단순 취향은 severity finding으로 올리지 않는다. 문서가 이미 적절하면 억지 문제를 만들지 않는다.

## 13. Ready-to-paste audit prompt

아래 prompt를 고비용 모델에게 그대로 전달할 수 있다.

> Audit the FBNeo DAT curator design and learn-rust TDD track before any Cargo scaffold or implementation. Read every file in the exact order listed in `TDD/fbneo-dat-curator/AUDIT_BRIEF.md`, then follow that brief completely. Treat user decisions in section 4 as fixed unless you can show a blocking correctness or safety problem. Verify ROM-set semantics against primary sources and the evidence report; do not trust terminology merely because it appears in the plan. Stress-test all adversarial scenarios, audit mission granularity for a beginner-to-intermediate Rust learner with senior systems experience, and check that the predecessor remains separate from RomLibraryIR's active FBNeo-first milestone. Produce the required report format with severity, exact document locations, evidence, and a minimal documentation patch plan. Do not create implementation code, Cargo scaffolding, real-data fixtures, commits, or cross-repository dependencies.

## 14. Audit completion gate

Mission 00 remains blocked until:

- [ ] Audit report exists
- [ ] Every Critical/High finding has an explicit disposition
- [ ] Non-Merged BIOS/sample projection is verified or narrowed
- [ ] Mission ordering and size are accepted
- [ ] RomLibraryIR boundary is rechecked
- [ ] User explicitly approves starting Mission 00
