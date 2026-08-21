# FBNeo DAT Curator Context

이 문서는 구현 세부가 아니라 트랙에서 사용하는 domain 용어의 기준이다.

## Catalog language

### Machine

입력 DAT의 하나의 game/machine entry. Playable game뿐 아니라 BIOS와 dependency-only provider도 machine일 수 있다.

### Clone Family

`cloneof` genealogy를 따라 같은 root를 공유하는 playable machine 집합이다. Family root가 자동으로 가장 좋은 release라는 뜻은 아니다.

### Selected Playable

Selection policy가 clone family의 대표로 고른 하나의 machine이다.

### Provider

다른 machine의 content 또는 sample requirement를 제공하는 machine이다.

### Game Provider

Parent-game ROM을 제공하는 provider다. Non-Merged projection에서는 필요한 content가 selected playable 안으로 localized된다.

### External Provider

Non-Merged에서도 selected playable 바깥에 남는 BIOS, device 또는 sample provider다.

## Curation language

### Eligibility

Machine을 selection 후보로 허용할지 결정하는 규칙이다. Ranking보다 먼저 적용한다.

### Ranking

Eligible candidate 사이의 선호 순서를 결정하는 규칙이다. Parent 여부나 ROM 수를 품질로 간주하지 않는다.

### 1G1R

Clone family마다 selected playable 하나를 고르는 정책이다. BIOS와 dependency-only provider의 수를 하나로 제한하지 않는다.

### Non-Merged

Selected playable의 own ROM과 필요한 parent-game ROM은 selected set에 함께 기술하고 BIOS/device content는 external provider로 유지하는 mode다. BIOS/device까지 selected set에 포함하는 Full Non-Merged/Standalone과 다르다.

### Relation Projection

Selection과 target mode가 결정된 뒤 projected catalog에 맞춰 `cloneof`, `romof`, `sampleof`와 ROM-level `merge`를 유지·제거·재지정하는 과정이다.

### Localized ROM

Provider 관계를 추적한 결과 selected playable 안에 기술되는 ROM requirement다.

### External Requirement

Non-Merged output에서도 BIOS/device/sample provider에 남는 requirement다.

### Selection Manifest

Source digest, effective policy, 후보, 선택·탈락 이유, provider locality와 relation projection을 기록한 provenance artifact다.

## Learning and reuse language

### Incubation

`learn-rust/TDD`에서 behavior와 interface를 TDD로 학습·검증하는 단계다.

### Promotion

학습 package를 영구 dependency로 연결하지 않고 유용한 source, test 또는 pattern을 새 프로젝트에 복사하고 domain에 맞게 수정하는 과정이다.

### Promotion Record

원본 commit, transplant한 file/test, 변경한 semantics와 새 프로젝트의 integration evidence를 기록한 문서다.

## Exact source vocabulary

대상 Logiqx DAT의 관계 이름은 다음과 같다.

- machine-level: `cloneof`, `romof`, `sampleof`
- ROM-level: `merge`

이 context에서는 `mergeof`라는 용어를 사용하지 않는다.
