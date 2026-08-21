# Rust + TDD Learning Track: fbneo-dat-curator

**상태:** 설계 승인, 고비용 모델 audit 대기, Cargo scaffold 전

**코드 스파인:** 이 디렉터리에 단일 Cargo package로 생성 예정

**학습 방식:** project-first, concept-reinforced, practical-strict TDD

**Domain glossary:** [`CONTEXT.md`](CONTEXT.md)

**Audit handoff:** [`AUDIT_BRIEF.md`](AUDIT_BRIEF.md)

## 목적

로컬 FBNeo Logiqx DAT를 streaming parse하고, 명시적인 selection policy로 clone family마다 playable machine 하나를 고른 뒤, **1G1R Non-Merged DAT와 selection manifest**를 결정적으로 생성한다.

이 트랙은 ROM Library IR보다 먼저 완성하는 독립 predecessor다. ROM Library IR가 이 package를 Cargo dependency로 소비하지 않는다. 대신 parser, graph, projection, test 또는 CLI pattern 중 검증된 부분을 이후 프로젝트에 source-level로 복사·수정할 수 있다.

## 성공 조건

- Synthetic fixture만으로 normal test suite가 완결된다.
- 실제 ROM/BIOS/archive를 읽거나 변경하지 않는다.
- `quick-xml` streaming parser가 malformed input 위치를 설명한다.
- `petgraph` graph가 `cloneof`, `romof`, `sampleof` 관계와 cycle/missing target을 표현한다.
- Clone family마다 policy가 고른 playable machine이 정확히 하나다.
- 기본 region priority는 Korea → US → World → Japan → Other다.
- CLI override가 TOML보다, TOML이 built-in default보다 우선한다.
- Parent-game ROM은 selected machine에 localized되고 BIOS/sample은 external로 남는다.
- `cloneof`, `romof`, ROM-level `merge`는 blanket 삭제하지 않고 relation projection으로 결정한다.
- Output DAT를 다시 parse했을 때 dangling relation이 없다.
- 같은 input bytes와 policy는 byte-identical DAT와 JSON manifest를 만든다.
- 실패한 validation은 기존 output을 교체하지 않는다.
- 실제 local DAT 결과를 igir 또는 최신 clrmame oracle과 machine/requirement 수준으로 비교한다.

## Non-goal

- ROM, BIOS 또는 sample ZIP materialization
- Archive copy, rename, repack 또는 rebuild
- Split, merged/full, fullnonmerged/standalone 출력
- 모든 MAME·software-list·CHD dialect 지원
- DAT 다운로드
- 모든 sibling ROM의 union
- CRC만 사용한 content deduplication
- RomLibraryIR M5 또는 M7 대체
- Cross-repository Cargo path dependency
- Agent가 core implementation과 behavior test를 대신 작성하는 방식

## 학습 목표

### Rust

- `BufRead`, borrowed event와 reusable buffer를 사용한 streaming XML
- Raw model과 projected model의 소유권 분리
- `HashMap` index와 `petgraph::NodeIndex` mapping
- Typed edge, DFS/SCC/cycle detection와 deterministic traversal
- Closure를 이용한 ranking key와 iterator pipeline
- `enum`, newtype과 typed error로 invalid state를 드러내기
- TOML/CLI configuration merge와 precedence
- Writer, temporary file, reparse validation과 atomic rename
- `lib.rs` 중심 architecture와 thin `main.rs`

### Domain

- 1G1R selection과 ROM set mode 구분
- `cloneof` genealogy와 `romof` content provider 구분
- ROM-level `merge`를 physical duplication 명령으로 오해하지 않기
- Non-Merged와 Full Non-Merged/Standalone 구분
- Playable selection과 dependency-only provider 구분
- Source evidence와 projected result를 manifest로 분리

### TDD

- Behavior를 먼저 이름 붙이고 실패 이유를 확인
- Synthetic fixture로 relation edge case를 작게 재현
- Table test로 policy precedence와 ranking 검증
- Projection matrix로 local/external provider 조합 검증
- Reparse, determinism과 atomic-write invariant 검증
- 실제 DAT와 oracle evidence를 normal test와 분리

## 진행 계약

- 한 세션 60–90분, 한 번에 한 mission만 진행한다.
- 사용자가 core domain code와 behavior test를 작성한다.
- Agent는 mission primer, 공식 문서, 진단 질문과 단계별 hint를 제공한다.
- 프로젝트 정답을 mission 문서에 미리 넣지 않는다.
- RED가 compilation setup 오류가 아니라 의도한 behavior failure인지 확인한다.
- GREEN은 현재 behavior에 필요한 최소 구현만 작성한다.
- REFACTOR는 green 상태에서만 수행한다.
- Mission 완료 시 사용자가 핵심 rule을 자신의 말로 설명하고 짧은 learning record를 남긴다.
- Dependency version은 Mission 00에서 `cargo add`가 선택하는 current stable을 확인하고 lockfile로 고정한다. 설계 문서에 version을 미리 pin하지 않는다.

Cargo scaffold 이후 기본 verification은 다음과 같다.

```text
cargo fmt --check
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## Target architecture

```text
fbneo-dat-curator/
├── Cargo.toml
├── CONTEXT.md
├── ROADMAP.md
├── AUDIT_BRIEF.md
├── README.md                     # 완료 단계에서 작성
├── src/
│   ├── lib.rs
│   ├── model.rs                  # raw/projected model
│   ├── parser.rs                 # quick-xml streaming
│   ├── index.rs                  # shortname/reference validation
│   ├── graph.rs                  # petgraph typed relations
│   ├── tags.rs                   # region/revision/release classification
│   ├── policy.rs                 # built-in/TOML/CLI merge
│   ├── selection.rs              # family grouping and ranking
│   ├── providers.rs              # ROM provider tracing/locality
│   ├── projection.rs             # Non-Merged relation projection
│   ├── writer.rs                 # deterministic DAT and manifest
│   ├── error.rs
│   ├── cli.rs
│   └── main.rs                   # thin composition root
├── tests/
│   ├── fixtures/synthetic/
│   ├── parser.rs
│   ├── graph.rs
│   ├── policy.rs
│   ├── selection.rs
│   ├── projection.rs
│   ├── writer.rs
│   └── cli.rs
└── learning-records/
```

이 tree는 방향이며 Mission 00 전에는 code file을 생성하지 않는다. 실제 seam은 failing behavior가 요구할 때만 나눈다.

## Dependency candidates

| 역할 | Candidate | 선택 이유 |
|---|---|---|
| XML | `quick-xml` | Event streaming과 explicit buffer control |
| Graph | `petgraph` | Directed typed edge, traversal, cycle/SCC, DOT diagnostic |
| CLI | `clap` derive | Typed option, value enum, generated help |
| Config/model | `serde`, `toml`, `serde_json` | Policy와 manifest |
| Error | `thiserror` | Library boundary의 typed error |
| Source digest | RustCrypto `sha2` | Input/output provenance |
| CLI test | `assert_cmd` | Real process boundary 검증 |
| Filesystem test | `tempfile` | Atomic output와 failure isolation |

`indextree`는 단일-parent tree이므로 전체 typed dependency graph에 사용하지 않는다. `daggy`는 insertion 전에 DAG를 강제하지만 importer는 malformed cycle을 수집·설명해야 하므로 기본 선택이 아니다. `petgraph`도 반드시 필요한지 Mission 04에서 explicit `HashMap + DFS` 대안과 complexity를 비교한다.

## Mission roadmap

### Phase 0 — Contract와 scaffold

#### Mission 00 — Package와 TDD harness

**Product result:** Edition 2024 package, `lib.rs`와 thin `main.rs`, 첫 synthetic fixture와 test command가 준비된다.

**Rust focus:** Cargo package, library/binary boundary, integration test discovery.

**First RED behavior:** 아직 구현되지 않은 library operation을 호출했을 때 의도한 최소 behavior가 실패한다. Dependency wiring 자체는 RED 예외지만 scaffold verification은 실행한다.

**Acceptance:**

- Core logic이 `main.rs`에 들어가지 않는다.
- Current stable dependency를 scaffold 시점에 확인한다.
- Real DAT, absolute path와 ROM content가 없다.
- 전체 quality command가 실행된다.

**Learning record:** 왜 package를 workspace/multi-crate로 시작하지 않았는지 설명한다.

#### Mission 01 — Raw catalog model

**Product result:** Header, Machine, Rom, DriverStatus와 raw relation을 표현하는 최소 model이 생긴다.

**Rust focus:** struct, enum, newtype, `Option`, owned string과 borrowed parser event의 경계.

**First RED behavior:** 독립 machine 하나와 BIOS consumer 하나를 source evidence 손실 없이 표현한다.

**Acceptance:**

- `cloneof`, `romof`, `sampleof`, `isbios`, ROM `merge`를 서로 다른 field로 보존한다.
- Missing hash와 dump status를 valid input state로 표현한다.
- `mergeof`라는 가짜 domain term을 도입하지 않는다.

### Phase 1 — Streaming import와 structural validation

#### Mission 02 — `quick-xml` streaming parser

**Product result:** Synthetic Logiqx DAT를 `BufRead`에서 parse한다.

**Rust focus:** event loop, borrowed bytes, reusable buffer, state machine, `?`.

**First RED behavior:** 여러 machine과 empty element를 가진 fixture를 순서대로 parse하고 malformed XML의 position을 보고한다.

**Acceptance:**

- Whole document `String` load와 derive-only deserialization로 우회하지 않는다.
- Buffer lifetime 때문에 borrowed event를 model에 잘못 저장하지 않는다.
- DOCTYPE가 network/file external entity access를 유발하지 않는다.
- Unknown but ignorable metadata 처리 정책을 test로 고정한다.

#### Mission 03 — Index와 reference validation

**Product result:** Shortname index를 만들고 duplicate/missing relation을 구분한다.

**Rust focus:** `HashMap`, entry API, error accumulation 대 fail-fast trade-off.

**First RED behavior:** duplicate shortname과 dangling `romof`가 서로 다른 diagnostic을 만든다.

**Acceptance:**

- Input insertion order가 output 결정성을 좌우하지 않는다.
- Path가 아닌 machine/source position context를 error에 남긴다.
- Recoverable warning과 transformation-blocking error를 구분한다.

#### Mission 04 — Typed relation graph with `petgraph`

**Product result:** Consumer → provider 방향의 CloneOf, RomOf, SampleOf graph를 만든다.

**Rust focus:** `DiGraph`, `NodeIndex`, edge weight, DFS/SCC와 cycle reporting.

**First RED behavior:** clone → parent → BIOS chain은 resolve되고 synthetic cycle은 정확한 members와 함께 거부된다.

**Acceptance:**

- Machine shortname과 `NodeIndex`를 혼동하지 않는다.
- Graph index를 output ordering으로 사용하지 않는다.
- 같은 behavior를 explicit map/DFS로 만들 때와 비교해 `petgraph`가 숨기는 complexity를 learning record에 쓴다.

### Phase 2 — Policy와 1G1R selection

#### Mission 05 — Description tag classification

**Product result:** Raw description tag를 Region, Revision, ReleaseKind와 Unknown evidence로 분류한다.

**Rust focus:** iterator, pattern matching, 작은 parser, conservative classification.

**First RED behavior:** Korea/US/World/Japan/Other와 unknown tag가 raw text를 잃지 않고 분류된다.

**Acceptance:**

- Known tag만 해석한다.
- Unknown을 임의의 region/revision으로 승격하지 않는다.
- Europe/Asia 등은 default ranking에서 Other bucket으로 비교하되 raw normalized value를 보존한다.

#### Mission 06 — Effective policy merge

**Product result:** Built-in, TOML과 CLI 입력을 하나의 validated policy로 합친다.

**Rust focus:** Serde config, Clap value parser, default와 override merge.

**First RED behavior:** CLI region order가 TOML을 덮고, TOML이 built-in Korea → US → World → Japan → Other를 덮는다.

**Acceptance:**

- 중복/누락 region bucket을 명시적으로 처리한다.
- Invalid family override를 뒤늦게 무시하지 않는다.
- Effective policy 전체가 serialize 가능하다.

#### Mission 07 — Family grouping과 explainable selection

**Product result:** Clone family별로 selected playable 하나와 ordered rationale을 만든다.

**Rust focus:** closure ranking key, stable sort, total ordering, domain result enum.

**First RED behavior:** Parent가 아닌 Korean clone이 기본 policy로 선택되고 sibling 탈락 이유가 결정적으로 기록된다.

**Acceptance:**

- Parent, World 또는 ROM count를 자동 정답으로 사용하지 않는다.
- Explicit override가 가장 높은 우선순위다.
- Eligible candidate가 없으면 `NoEligibleCandidate`를 반환한다.
- Tie-break는 stable shortname order다.

### Phase 3 — Provider tracing과 Non-Merged projection

#### Mission 08 — ROM provider tracing

**Product result:** Selected machine의 ROM requirement가 own, game-provider 또는 external-provider 중 어디에서 오는지 설명한다.

**Rust focus:** graph traversal, visited state, lookup closure, evidence-rich result.

**First RED behavior:** selected clone → parent game → BIOS fixture에서 game ROM과 BIOS ROM의 locality가 다르게 분류된다.

**Acceptance:**

- `merge` name만 또는 CRC만 보고 provider를 확정하지 않는다.
- Available name/size/hash evidence 충돌을 error 또는 ambiguity로 유지한다.
- Sibling ROM을 union하지 않는다.
- Source가 requirement를 설명하지 못하면 추측하지 않는다.

#### Mission 09 — Relation projection matrix

**Product result:** Selection과 provider locality에서 projected `cloneof`, `romof`, `sampleof`, ROM `merge`를 계산한다.

**Rust focus:** pure transformation, exhaustive enum match, table-driven test.

**First RED behavior:** selected clone → parent → BIOS에서 parent-game `merge`는 제거되고 BIOS-backed `merge`는 유지되며 `romof`는 retained BIOS provider로 재지정된다.

**Acceptance:**

- 모든 relation을 일괄 삭제하지 않는다.
- 제거된 target을 가리키는 dangling relation이 없다.
- BIOS/sample provider는 playable count와 분리된다.
- Projection 결과에 각 field 변경 이유가 남는다.

#### Mission 10 — Projected catalog invariant

**Product result:** Output 전용 model과 full invariant validator가 생긴다.

**Rust focus:** smart constructor 또는 validation boundary, aggregate diagnostic.

**First RED behavior:** playable family 중복, missing BIOS, invalid merge target과 output name collision을 각각 차단한다.

**Acceptance:**

- Family당 selected playable 하나다.
- Local game closure와 external provider가 mode rule을 만족한다.
- Disk/CHD처럼 지원하지 않는 construct를 조용히 버리지 않는다.

### Phase 4 — Deterministic artifacts와 CLI

#### Mission 11 — Deterministic DAT writer

**Product result:** Projected catalog를 stable Logiqx XML로 쓰고 즉시 reparse/validate한다.

**Rust focus:** streaming writer, explicit ordering, escaping, round-trip invariant.

**First RED behavior:** 같은 model을 두 번 쓰면 bytes가 같고, 다시 parse한 구조가 projected invariant를 만족한다.

**Acceptance:**

- Exact original whitespace round-trip을 약속하지 않는다.
- Machine/ROM/sample/attribute ordering rule을 test로 고정한다.
- Reparse failure를 성공 output으로 취급하지 않는다.

#### Mission 12 — Explainable JSON manifest

**Product result:** Source/output digest, effective policy, candidate rationale와 relation projection을 JSON으로 기록한다.

**Rust focus:** serialization boundary, schema version, deterministic map/list ordering.

**First RED behavior:** 선택과 탈락 이유, raw region tag, localized/external provider와 rewired relation이 manifest에 나타난다.

**Acceptance:**

- Private path와 inventory를 기록하지 않는다.
- DAT digest와 manifest policy만으로 실행 identity를 비교할 수 있다.
- Unknown/weak evidence를 숨기지 않는다.

#### Mission 13 — Clap CLI와 atomic output

**Product result:** Input DAT, output DAT, manifest, TOML policy와 CLI override를 받는 thin CLI가 생긴다.

**Rust focus:** Clap derive/subcommand, exit code, stderr, tempfile/rename.

**First RED behavior:** CLI region order가 TOML을 override하고, failed validation이 기존 output files를 바꾸지 않는다.

**Acceptance:**

- `main.rs`는 parse/invoke/render-error만 수행한다.
- Output은 temporary sibling에 쓴 뒤 두 artifact 검증 후 commit한다.
- Help에 Non-Merged와 Full Non-Merged 차이가 드러난다.
- CLI process test가 success와 representative failure exit를 확인한다.

### Phase 5 — Private acceptance와 promotion review

#### Mission 14 — Local real-DAT acceptance

**Product result:** Git-excluded 실제 FBNeo DAT에서 scale, policy와 oracle discrepancy report를 얻는다.

**Rust focus:** opt-in acceptance harness, measurement, evidence separation.

**First RED behavior:** synthetic suite와 별개로 local input이 있을 때만 실행되는 명시적 acceptance command를 정의한다.

**Acceptance:**

- 실제 DAT path와 content를 commit하지 않는다.
- Machine/family/BIOS/sample count를 parser constant로 쓰지 않는다.
- Igir 또는 최신 clrmame과 selected machine 및 requirement 차이를 구조적으로 보고한다.
- Discrepancy를 자동으로 implementation bug나 oracle 정답으로 단정하지 않는다.

#### Mission 15 — Documentation과 promotion dossier

**Product result:** README, architecture summary, known limitation과 source-transplant checklist가 완성된다.

**Rust focus:** API explanation, evidence-backed completion claim.

**Acceptance:**

- Quality command fresh output이 있다.
- 사용자가 streaming ownership, graph edge, selection과 projection rule을 설명한다.
- 어떤 module/test/pattern이 promote 가능하고 어떤 FBNeo semantics가 project-specific인지 구분한다.
- Promotion record template에 source commit, copied tests, semantic changes와 target integration evidence가 있다.
- Audit finding이 해결되거나 accepted risk로 기록된다.

## Synthetic fixture matrix

| Fixture | 핵심 behavior |
|---|---|
| `minimal.dat` | 독립 machine parse/write |
| `parent-clone.dat` | family와 game-parent merge |
| `clone-parent-bios.dat` | transitive BIOS projection |
| `direct-bios.dat` | direct external provider |
| `sample-provider.dat` | sample requirement 보존 |
| `cycle.dat` | graph cycle diagnostic |
| `missing-provider.dat` | dangling reference 차단 |
| `duplicate-shortname.dat` | index collision |
| `crc-collision.dat` | CRC-only dedup 금지 |
| `policy-candidates.dat` | region/revision/release ranking |
| `unknown-tag.dat` | conservative tag handling |

모든 fixture 이름과 content는 synthetic이며 실제 DAT에서 entry를 복사하지 않는다.

## Promotion gate

다른 프로젝트로 가져갈 수 있다는 것은 learning package를 그대로 dependency로 사용해야 한다는 뜻이 아니다. Promotion 후보는 다음 조건을 만족해야 한다.

- 작은 public surface 뒤에 실제 complexity를 숨긴다.
- Synthetic behavior test가 함께 이동할 수 있다.
- FBNeo-specific assumption과 general Rust pattern이 문서에서 구분된다.
- Absolute path, private data와 learn-rust 내부 module에 의존하지 않는다.
- Target project에서 새 integration test를 작성한다.
- Source commit과 semantic changes를 기록한다.

예상 재사용 형태:

| 부분 | 기본 promotion 형태 |
|---|---|
| XML event/state pattern | source + parser fixture transplant |
| Typed graph/cycle diagnostic | module/test transplant 또는 재구현 |
| Selection/ranking | policy pattern 재사용, domain rule 재검토 |
| Non-Merged projection | FBNeo-specific; 그대로 generalize하지 않음 |
| Clap/TOML precedence | pattern 또는 CLI test transplant |
| Atomic paired output | filesystem behavior/test transplant |

## Official references

- Rust Book: ownership, enums, error handling, iterators, closures
- `std::io::BufRead` API
- `quick-xml` Reader/Writer documentation
- `petgraph` graph, visit와 algo documentation
- Clap derive tutorial and command validation
- Serde, TOML and `thiserror` documentation
- Logiqx ROM Management Datafile DTD
- MAME documentation: About ROMs and Sets
- Current clrmame CLI mode documentation
- Igir Arcade ROM Sets documentation

각 mission 문서는 실제로 필요한 공식 page 2–5개만 골라 연결한다.

## Start gate

Cargo scaffold와 Mission 00은 다음이 모두 충족된 뒤 시작한다.

- [x] 사용자와 product/output boundary 합의
- [x] 1G1R Non-Merged target 합의
- [x] Region 기본값과 override 합의
- [x] Source transplantation 허용 합의
- [ ] `AUDIT_BRIEF.md`에 따른 고비용 모델 audit 검토
- [ ] Critical/High audit finding 처리
- [ ] 사용자의 명시적인 Mission 00 시작 승인

이 track은 learn-rust root의 ZMQ Topic Monitor mission을 대체하지 않는 **승인된 predecessor 예외**다. 전체 학습 우선순위와 시간 배분은 audit에서 별도로 검토한다.
