# Revue Anti-Pattern Catalog

Revue가 선택한 기술 스택에서 **구조적으로 발생할 수 있는 실패 모드**를 ID로 정리한 카탈로그다.
항목마다 그 실패가 revue에서 실제로 일어났는지(`observed`), 아직 가설인지(`hypothesis`)를 적는다.

## 적용 범위

Revue는 단순한 TUI 라이브러리가 아니라 다음 요소가 결합된 retained-mode reactive UI framework다.

- Rust 기반 retained-mode UI
- `Signal / Computed / Effect` 반응형 상태
- CSS 파서 · selector · cascade · layout
- Flex/Grid 레이아웃
- 이벤트 · 포커스 · 입력 처리
- worker pool 및 비동기 작업
- hot reload
- proc macro
- CLI · template · extension/plugin
- 100개 이상의 widget
- profiler · inspector · snapshot test
- tree-sitter 기반 CSS tooling

따라서 이 카탈로그는 Ratatui 애플리케이션의 안티패턴보다 **브라우저 엔진, React/Vue, 게임 UI 엔진,
async runtime, 컴파일러 프론트엔드에서 나타나는 안티패턴**까지 포함한다. 다른 framework의 약점을
revue가 피하려고 넣은 항목도 있다. 그런 항목은 revue에서 재현될 때까지 `hypothesis`다.

## 왜 문법 체크리스트가 아닌가

Revue에서 가장 위험한 것은 개별 Rust 문법 안티패턴이 아니다. 핵심 위험은 네 subsystem의 곱이다.

```text
Reactive graph  ×  Retained widget tree  ×  CSS/layout engine  ×  Terminal event/render loop
```

이 사이의 **변경 전파 · ownership · invalidation · scheduling 경계가 불분명해지는 것**이
가장 큰 구조적 안티패턴이다. 그래서 안티패턴 검사가 이렇게 끝나면 안 된다.

```text
clone을 많이 쓰지 말 것
Arc<Mutex>를 남용하지 말 것
unwrap을 쓰지 말 것
```

대신 `catalog.yaml`의 `review_questions`에 구조적으로 답할 수 있어야 한다.
그래야 Revue가 "기능이 많은 TUI 라이브러리"가 아니라 안정적인 UI framework가 된다.

## 이 디렉터리의 파일

| 파일 | 대상 | 내용 |
|---|---|---|
| [`catalog.yaml`](catalog.yaml) | 사람 / LLM / 스크립트 | 모든 항목, invariant, 리뷰 질문, 우선순위의 **유일한 원본** |
| `README.md` (이 파일) | 사람 | 사용법, 스키마, 관리 규칙, 검증 방법 |

항목 목록, 개수, 카테고리별 범위는 이 파일에 옮겨 적지 않는다. 두 곳에 두면 어긋난다.
필요하면 아래 [검증](#검증) 스크립트나 `yq`로 뽑는다.

## 사용법

### 사람: PR 리뷰

리뷰 코멘트를 태그 + ID로 시작한다.

```text
[REACTIVE] REV-REACT-003: 이 effect가 자신이 읽는 signal을 다시 set 한다 → 재진입 가능
[RENDER]   REV-RENDER-005: chars().count()를 폭으로 쓰면 CJK/emoji에서 어긋난다
```

### LLM: 코드 리뷰 프롬프트

`catalog.yaml`을 컨텍스트로 넣고 스코프를 좁혀서 물어본다. 전체를 한 번에 적용하려 하면
정확도가 떨어지므로, 변경된 파일의 subsystem에 해당하는 카테고리만 필터링한다.

```bash
# reactive + render 카테고리만 뽑아서 리뷰 컨텍스트로 사용
yq '.entries[] | select(.category == "reactive" or .category == "render")' \
  docs/anti-patterns/catalog.yaml

# revue에서 확인된 실패만
yq '.entries[] | select(.status == "observed") | .id + " " + .title' \
  docs/anti-patterns/catalog.yaml
```

프롬프트 예시:

```text
아래는 Revue의 anti-pattern 카탈로그 중 reactive/render 항목이다.
diff에서 실제로 해당하는 항목만 골라 id, 파일:줄, 근거를 제시하라.
해당 없으면 "없음"이라고 답하라. 추측으로 항목을 만들지 마라.
```

### 설계 리뷰

설계 결정의 근거로 항목을 쓸 때는 `status`를 본다. `hypothesis` 항목 하나만으로 결정하지 않는다.
코드, 버그, 측정 같은 독립된 근거를 함께 댄다. 결정은 GitHub 결정 이슈(Why / Decision / Not doing / Work)에
남기고, 항목의 `decisions`가 그 이슈를 가리킨다.

## 스키마

최상위에는 `meta`, `statuses`, `assumptions`, `review_tags`, `invariants`, `review_questions`,
`priorities`, `categories`, `entries`가 있다.

각 항목의 필드는 다음과 같다.

| 필드 | 필수 | 의미 |
|---|---|---|
| `id` | ✓ | `REV-<PREFIX>-<NNN>` 안정 식별자. 카테고리의 `prefix`를 쓴다. 번호는 다시 쓰지 않는다 |
| `title` | ✓ | 패턴 이름 (영문) |
| `category` / `review_tag` | ✓ | 소속 subsystem. `review_tag`는 카테고리의 `tag`와 같다 |
| `status` | ✓ | `observed` / `hypothesis` / `rejected` (`statuses` 참고) |
| `assumes` | ✓ | 항목이 전제하는 구조 (`assumptions`의 id 목록, 없으면 `[]`) |
| `decisions` | | 이 주제를 정한 GitHub 결정 이슈 (`"#1142"`) |
| `summary` | ✓ | 이 패턴이 무엇인지 |
| `problem` | | 그래서 무엇이 깨지는지 |
| `detect` | | 어디를 보면 발견되는지 (코드 모양, 검증 입력) |
| `fix` | | 방지 원칙과 revue의 방향 |
| `snippets` | | 코드/도식 |
| `observed` | `observed`일 때 | revue에서 일어난 사례와 근거 (코드 위치, 이슈, PR, 측정). 고친 사례도 남긴다 |
| `checked` | | 확인했지만 재현되지 않은 것, 코드 읽기로만 의심되는 것. 날짜를 붙인다 |
| `rejected_because` | `rejected`일 때 | revue에 맞지 않는 이유 |

근거에 적힌 `docs/refactor/*.md`, `docs/anti-patterns/architecture-review.md`, `docs/ARCHITECTURE.md`는 지웠거나
다시 쓴 문서다. `meta.sources`의 commit 판으로 연다 (`<base>/<경로>`).

`invariants`의 각 규칙에는 `applies`(지금부터 지킬 `3.x`인지, 4.0 구조에서 지켜지는 `4.0`인지)와
`decisions`가 있다. `tests/e2e/invariants.rs`가 INV id로 참조하므로 id는 바꾸지 않는다.

## 관리 규칙

- **근거가 바뀌면 같은 PR에서 고친다.** 이슈나 감사에서 항목을 뒷받침하는 사례가 나오면 `observed`에 더하고
  `status`를 올린다. 항목이 틀렸거나 revue에 맞지 않으면 고치거나 `rejected`로 바꾸고 이유를 남긴다.
  항목은 지우지 않는다.
- **`observed`에는 따라갈 수 있는 근거를 쓴다.** 이슈·PR 번호, 파일 경로, 측정. 코드 읽기만으로 의심되는
  동작은 `checked`에 "코드 읽기로 의심"이라고 적고 `hypothesis`로 둔다.
- **위반을 고칠 때 새 widget별 helper를 만들기 전에** 같은 일을 하는 공용 model이 있는지 항목의 `fix`를 본다.
- **새 항목은 카테고리의 다음 번호로 끝에 붙인다.** 다른 framework의 약점에서 온 항목은 출처를 summary에 적는다.

## 검증

수정한 뒤 저장소 루트에서 실행한다. YAML이 읽히는지, id가 겹치지 않는지, 필수 필드와 값이 맞는지,
본문의 `REV-`/`INV-` 참조가 모두 있는지, 카테고리 번호에 빈칸이 없는지 확인하고 상태별 개수를 출력한다.

```bash
ruby - <<'EOF'
require "yaml"; require "date"
path = "docs/anti-patterns/catalog.yaml"
d = YAML.load_file(path, permitted_classes: [Date])
e = d["entries"]; bad = []
cats = d["categories"].to_h { |c| [c["id"], c] }
st = d["statuses"].map { _1["id"] }; asm = d["assumptions"].map { _1["id"] }
inv = d["invariants"].map { _1["id"] }; ids = e.map { _1["id"] }
keys = %w[id title category review_tag status assumes decisions summary problem detect fix snippets observed checked rejected_because]
bad << ["dup", ids.tally.select { |_, n| n > 1 }.keys] if ids.uniq.size != ids.size
e.each do |x|
  i = x["id"]
  %w[title category review_tag status assumes summary].each { |k| bad << [i, "missing #{k}"] unless x.key?(k) }
  bad << [i, "unknown keys", x.keys - keys] unless (x.keys - keys).empty?
  c = cats[x["category"]] or (bad << [i, "category"]; next)
  bad << [i, "tag"] if x["review_tag"] != c["tag"]
  bad << [i, "prefix"] unless i.match?(/\A#{Regexp.escape(c["prefix"])}-\d{3}\z/)
  bad << [i, "status"] unless st.include?(x["status"])
  bad << [i, "observed vs status"] if (x["status"] == "observed") != !(x["observed"] || []).empty?
  bad << [i, "rejected_because"] if (x["status"] == "rejected") != !(x["rejected_because"] || []).empty?
  bad << [i, "assumes", x["assumes"] - asm] unless (x["assumes"] - asm).empty?
  (x["decisions"] || []).each { |s| bad << [i, "decision", s] unless s.match?(/\A#\d+\z/) }
end
text = File.read(path)
(text.scan(/REV-[A-Z]+-\d{3}/).uniq - ids).each { bad << ["dangling", _1] }
(text.scan(/INV-\d{2}/).uniq - inv).each { bad << ["dangling", _1] }
cats.each_key do |c|
  n = e.select { _1["category"] == c }.map { _1["id"][-3..].to_i }.sort
  bad << [c, "numbering", n] if n != (1..n.size).to_a
end
puts "#{e.size} entries; #{e.map { _1["status"] }.tally}; bad: #{bad}"
EOF
typos docs/anti-patterns
```

`meta.updated`도 고친다.
