---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7113 — 레거시 hwpeq TAB 수식 정규화 검토

**최종 판정: 머지 보류.** F1 [P1, 실행 결함]: `\BAR `를 20000번 중첩한 220002-byte script를 2MiB worker stack에서 tokenize하면 SIGABRT(-6), stack overflow다. 기존 EqParser 깊이 64 제한보다 앞의 parse_sequence가 무제한 재귀한다. F2 [P2, 실행 결함]: 현행 수식 `rm "\TAB"`의 Quoted token 값이 `\TAB`에서 빈 문자열로 바뀐다. `rm "literal \TAB label"`도 `literal  label`이 된다. 기존 Tokenizer 직접 경로는 원문을 보존한다. 방언 판별·정규화의 quoted 상태를 보존하고 재귀/출력 예산을 추가해야 한다.

## 대상과 체리픽

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7113](https://github.com/edwardkim/rhwp/pull/7113) — 수정(renderer): 레거시 hwpeq5 OLE 수식의 \TAB 방언을 현행 문법으로 읽는다 (#7105) |
| 작성자 / reviewer | planet6897 / jangster77 사전 지정 |
| 원 base / 규모 | `devel`; 4 files, +420/-1 |
| source head | `13af3ecacd966792d0c04bbd66bfde2c32f34bdb` |
| 최초 기준 devel | `93ffc3dd59c120bd54df4c2ac6d1ddbe630f8a2d` |
| 검증한 누적 code head | `48afe0f95abc4cd76b002dfc314f3cc044f10b31` — 12 PR / 16 commit |
| 원본·기준 PDF 보존 commit | `6933852a11b7e5998429eeb15708fdaeed7db626` |
| 최신 devel 정렬 | `037e4906a93e99896daa145a5ee5517824bfeaf4`; 정렬 후 로컬 head `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa` |
| 검토 branch / target | `review/planet6897-20260914` / `target/planet6897-review-20260914` |
| 원 PR 상태 snapshot | 2026-09-14T16:10:01.589135+09:00; OPEN, draft=False, merge=UNKNOWN |
| 관련 이슈 | [#7105](https://github.com/edwardkim/rhwp/issues/7105); 부분 반영 범위만 판단, 이번 작업에서 종료하지 않음 |

대상은 조사 시점 open·non-draft 12개다. draft #7098은 제외했다. 최종 재조회에서도 대상과 source SHA는 같았다.
원 PR의 성공/skip CI는 확인했지만 최신 통합 candidate의 GitHub Actions 결과로 재사용하지 않는다.
검토·체리픽은 로컬 작업이며 원격 PR 생성·push·merge·close는 아직 하지 않았다.

| 원 commit | 로컬 적용 commit | 보정 |
| --- | --- | --- |
| `13af3ecacd966792d0c04bbd66bfde2c32f34bdb` | `43e40dcd7bd93b6f410ffb9d11b93945b9c5ef8f` | 충돌 없음; -x·저자 유지 |

### 실제 변경 경로

- `src/renderer/equation/legacy_hwpeq.rs` (+181/-0)
- `src/renderer/equation/mod.rs` (+1/-0)
- `src/renderer/equation/tokenizer.rs` (+6/-1)
- `tests/cases/issue_7105_legacy_hwpeq_tab_dialect.rs` (+232/-0)

## 조판 원칙과 원인 계층

OLE hwpeq5의 TAB 인자 종결 의미는 보존하되 현재 수식 문법의 quoted literal과 기존 재귀 예산을 침범해서는 안 된다. 정규화는 tokenizer/AST 앞에 있으므로 parser의 MAX_EQ_DEPTH만으로 보호되지 않는다.

주요 검토 위치: [src/renderer/equation/legacy_hwpeq.rs](../../../src/renderer/equation/legacy_hwpeq.rs).

| 공통 항목 | 판정 | 근거·제한 |
| --- | --- | --- |
| 구현 근거와 일반성 | 미충족 | OLE hwpeq5의 TAB 인자 종결 의미는 보존하되 현재 수식 문법의 quoted literal과 기존 재귀 예산을 침범해서는 안 된다. 정규화는 tokenizer/AST 앞에 있으므로 parser의 MAX_EQ_DEPTH만으로 보호되지 않는다. |
| 측정·배치 일관성 | 비해당 | 이 PR은 수식 구문 변환 또는 저장/control API/진단 계층이며, 문단·표의 공통 줄 구성 경로를 변경하지 않는다. |
| 줄 소속과 점유 높이 | 비해당 | 이 PR 자체는 줄 소속/점유 높이 계산을 바꾸지 않는다. 글자폭·출력에 따른 누적 줄바꿈 차이는 별도 실제 sweep 결과에서 다뤘다. |
| 사례와 증거의 독립성 | 충족 | 합성 계약과 실제 원본·한컴 PDF/직렬화/API·진단 증거를 분리했다. 실행 결함이 발견된 경우에도 정상 샘플 통과로 상쇄하지 않았다. |
| 기준값 변경 | 비해당 | 이 PR은 rendering baseline·golden·허용치를 변경하지 않는다. |
| 주장과 검증 범위 | 충족 | source SHA·실행 코드·실제 원본·명령·결과를 아래에 기록했다. 실행 회귀, 기존 잔여, 미검증을 구분하고 원 PR CI를 통합 CI로 재사용하지 않았다. |
| 실제 입력 커밋 | 충족 | 개인 다운로드 경로만 남기지 않고 Git object 또는 LFS oid와 실제 SHA-256을 대조했다. 기존 동일 파일은 재추가하지 않았다. |

## 직접 실행·시각 판정

신고 원본 transistor-mosfet.hwp와 신고자 Hancom 2022 PDF 11쪽을 확보했다. 2쪽의 TAB 글자·가로 압착이 줄고 첨자·분수 구조가 복원된다. 같은 바이너리의 실제 rhwp rlib public tokenizer를 호출한 별도 프로세스로 아래 두 결함을 재현했다.

F1 [P1, 실행 결함]: `\BAR `를 20000번 중첩한 220002-byte script를 2MiB worker stack에서 tokenize하면 SIGABRT(-6), stack overflow다. 기존 EqParser 깊이 64 제한보다 앞의 parse_sequence가 무제한 재귀한다. F2 [P2, 실행 결함]: 현행 수식 `rm "\TAB"`의 Quoted token 값이 `\TAB`에서 빈 문자열로 바뀐다. `rm "literal \TAB label"`도 `literal  label`이 된다. 기존 Tokenizer 직접 경로는 원문을 보존한다. 방언 판별·정규화의 quoted 상태를 보존하고 재귀/출력 예산을 추가해야 한다.

**잔여·미검증:** eqalign/RTN 4개와 OLE 실제 편집은 미구현이다. #7116은 진단 개선이므로 #7105를 닫는 근거가 아니다. 정상 샘플 개선과 네 focused 회귀 통과는 위 두 실패를 무효화하지 않는다.

시각 검증 필요: **예**. 전체 문서의 SVG·render tree·문자/레이아웃 ledger를 생성하고, 아래 페이지를 96dpi로 한컴 PDF / base / 통합 세 방향으로 직접 읽었다. 페이지 수·픽셀 점수만으로 통과시키지 않았다.

- transistor-p002: [한컴 / 변경 전 / 통합 비교](../assets/pr7113_transistor-p002_3way.png), [한컴·통합 overlay](../assets/pr7113_transistor-p002_ovl.png)

원본 PDF가 내장하지 않은 글꼴의 기존 매핑, editor-only placeholder, 선 굵기 차이는 전체 일치로 판정하지 않았다.
단일 쪽 SVG가 `_001` 없이 저장되면 fidelity helper의 파일명 기반 쪽수가 0으로 표시되는 경우가 있었다.
실제 SVG·render-tree 파일과 한컴 PDF 1쪽을 확인해 계수 오류와 렌더 실패를 구분했다.

## 입력·기준 출력 보존

확인 tree: `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa`. 다음 SHA-256은 LFS pointer 문자열이 아닌 실제 파일 바이트의 해시이며,
Git object 또는 LFS oid와 일치한다. full/OVR 공통 입력은 아래 재현 명령의 저장소 fixture 집합을 사용했다.

| 저장소 파일 | 출처·역할 | SHA-256 |
| --- | --- | --- |
| [tests/fixtures/issue_7105/transistor-mosfet.hwp](../../../tests/fixtures/issue_7105/transistor-mosfet.hwp) | 한컴 입력; transistor | `ae2f5de257f9666a2bb0b04aa1548d3165e12d5ae6755358bfc57f7a62e3c0aa` |
| [pdf/issue7105/transistor-mosfet-hancom2022.pdf](../../../pdf/issue7105/transistor-mosfet-hancom2022.pdf) | 독립 한컴 PDF 11쪽; 직접 sweep 2,5쪽 | `afdaf0a1ff2ef764511a069e5c6294516cbb3f555a71d8d859639b8e4e72485f` |

OVR 공통 입력도 같은 확인 tree의 파일을 사용했다.

| 저장소 파일 | 역할 | SHA-256 |
| --- | --- | --- |
| [samples/KTX.hwp](../../../samples/KTX.hwp) | OVR 5 공통 입력 | `b6c1492152f53e8dd7d4bbbb4faca88866bb8458e9018c70c936cd469ea6fab3` |
| [samples/exam_math.hwp](../../../samples/exam_math.hwp) | OVR 5 공통 입력 | `e40e3d675373c8efb3a844fc71f209600d3b0db987a04b3808b8e74a6b1671fe` |
| [samples/21_언어_기출_편집가능본.hwp](../../../samples/21_언어_기출_편집가능본.hwp) | OVR 5 공통 입력 | `905454045ca2e236839a7cab59750678116d08af3db31dbf846819af355b8d15` |
| [samples/aift.hwp](../../../samples/aift.hwp) | OVR 5 공통 입력 | `a3e94e613a7d3dad0ee11e2df8f9572a5b7c2d704602960c2075b5fd22df995c` |
| [samples/biz_plan.hwp](../../../samples/biz_plan.hwp) | OVR 5 공통 입력 | `8b786d6824622afae2220b203beeef6e5592157e1896fea055ebc602817113c1` |

신규 자료 출처·변환 영수증은 [새 한컴 PDF manifest](../../../pdf/pr-planet6897-20260914/README.md),
[transistor 원본·신고자 PDF](../../../tests/fixtures/issue_7105/README.md),
[익명화 다단 원본](../../../tests/fixtures/issue_6970/README.md)에 보존했다. 신규 MCP 변환 6건은 engine 2020,
Hancom 12.0.0.4605로 start → status(queued/running/succeeded) → download → SHA 확인까지 수행했다.
신고자 transistor PDF는 Hancom 2022의 기존 11쪽 출력이며 새 MCP 출력으로 오인하지 않는다.
인증 정보·임시 SVG/JSON·중간 로그는 커밋하지 않는다.

## 검증 결과와 재현

검증 코드 `48afe0f95abc4cd76b002dfc314f3cc044f10b31`와 최신 정렬 head의 Rust source·Cargo 입력은 동일하다. 추가 fixture commit은 원본/PDF 보존이며,
나중에 들어온 upstream은 Studio Vite/@types/chrome 의존성과 기존 검토 문서만 변경했다.
Rust 검증을 이 upstream 변경 후 재실행했다고 주장하지 않는다.

| 검증 | 실제 결과 |
| --- | --- |
| fmt / generated manifest / unit tiers | 통과; 생성 suite는 stage하지 않음 |
| focused nextest | 97 passed, 9800 skipped |
| 전체 nextest | 9846 passed, 51 skipped; 실행 528.167초 |
| Native Skia lib | 3930 + 15 + 165 + 2 passed, 13 ignored |
| Native placeholder / direct PDF export | 2 / 4 passed |
| Clippy native / wasm / workspace all-targets | 3종 통과, workspace build 통과 |
| OVR 필수 5문서 | KTX 27, exam_math 20, 언어 기출 15, aift 74, biz_plan 6쪽; 개체 회귀 0 |
| 새 WASM / 실제 Chrome | 빌드 성공; Chrome 152.0.7977.83에서 soil 1쪽, transistor 2쪽, table-text 1쪽, synth 1쪽을 열고 렌더. Native와 752/560/140/883 Text element 속성·텍스트 모두 동일 |
| WASM 배포 빌드 제한 | Docker daemon 미가용으로 native wasm-pack `--no-opt` 사용. 표준 Docker/wasm-opt 배포 빌드 완료를 주장하지 않음 |
| 원 PR CI | 위 source head의 Actions에 실패·대기 없음(성공/skip); 최신 통합 PR CI는 미실행 |

```sh
cd /Users/tsjang/rhwp
node scripts/rust-test-suite-manifest.mjs --prepare
cargo fmt --all -- --check
cargo build --locked --target-dir target/planet6897-review-20260914 --profile release-test --bin rhwp
cargo nextest run --locked --target-dir target/planet6897-review-20260914 --cargo-profile release-test --tests --test-threads 6 --no-fail-fast -E 'test(/issue_4680|issue_7097|issue_6970|issue_7092|issue_7105|issue_7080|issue_7081|issue_7059|issue_7051|issue_7130|issue_3820_rowbreak_rowspan_band|issue_6590|issue_7084|issue_1285|svg_snapshot/)'
cargo nextest run --locked --target-dir target/planet6897-review-20260914 --cargo-profile release-test --tests --test-threads 6 --no-fail-fast
cargo test --locked --target-dir target/planet6897-review-20260914 --profile release-test --features native-skia --lib -- --test-threads 6
node scripts/run-rust-test.mjs issue_2225_missing_picture_placeholder -- --cargo-profile release-test --locked --target-dir target/planet6897-review-20260914 --features native-skia
node scripts/run-rust-test.mjs render_p37_direct_pdf_export -- --cargo-profile release-test --locked --target-dir target/planet6897-review-20260914 --features native-skia
cargo clippy --locked --target-dir target/planet6897-review-20260914 -- -D warnings
cargo clippy --locked --target-dir target/planet6897-review-20260914 -p rhwp --lib --target wasm32-unknown-unknown -- -D warnings
cargo build --locked --target-dir target/planet6897-review-20260914 --workspace
cargo clippy --locked --target-dir target/planet6897-review-20260914 --workspace --all-targets -- -D warnings
node scripts/rust-test-suite-manifest.mjs --check
node scripts/rust-unit-test-tiers.mjs --check
scripts/wasm-pack-locked.sh --target web --out-dir /private/tmp/rhwp-planet6897-review-20260914/pkg --no-opt
```

시각 재현 예시(실제 입력·PDF·페이지는 위 표):

```sh
venv/bin/python scripts/visual_sweep.py --file-target <식별자> <입력> <한컴-PDF> \
  --rhwp-bin <해당-SHA에서-빌드한-rhwp> --pages <검토-쪽> --dpi 96 --out <외부-산출-폴더>
```

### 새 WASM browser 증적과 공통 시각 재현

[soil 1쪽](../assets/pr7111_browser_wasm_soil.png),
[transistor 2쪽](../assets/pr7113_browser_wasm_transistor.png),
[table-text 1쪽](../assets/pr7117_browser_wasm_table-text.png),
[synth 1쪽](../assets/pr7104_browser_wasm_synth.png)을 실제 Chrome에서 캡처했다.
`HwpDocument(bytes).renderPageSvg(pageIndex)`와 `pageCount()`를 호출하고
저장소 webfont projection으로 렌더했다. 정적 기존 pkg를 재사용하지 않았다.

전수 문자/레이아웃 검사는 각 base/candidate 바이너리를 `RHWP_BIN`으로 지정해 아래 명령으로 실행했다.

```sh
RHWP_BIN=<해당-SHA-바이너리> venv/bin/python tools/fidelity_compare/fidelity_compare.py \
  0 <마지막-0-based-쪽> --source <입력> --reference-pdf <한컴-PDF> \
  --label <문서명> --reference-grade 'Hancom PDF' --text-only --export-all-svg \
  --layout-ledger --out-dir <외부-산출-폴더>
```

OVR은 `tools/object_visual_regression.py`의 `PRESETS['ovr5']` 다섯 입력을 차례로 실행했다.
module의 `RHWP`를 새 base/candidate 바이너리로, `git_head()`를 해당 검증 code SHA로 지정했다.
각 입력은 base에서 `--no-hwp --save-baseline -o <base-folder>`, candidate에서
`--no-hwp --baseline <base-folder>/baseline.json -o <candidate-folder>`로 호출했고 전부 exit 0이었다.
이 실행은 **devel 대비 개체 회귀 검사**이며 한컴 PDF 동등성 검사를 대신하지 않는다.

## 다음 조건과 merge 후 contributor PR comment 계획

보류 해제 조건은 위 발견 사항의 원인 보정 또는 수용 가능한 독립 증거, 관련 focused·실물 PDF 재검증이다. 현재 묶음을 그대로 merge 대상으로 올리지 않는다.

보정·범위 확정 후 최신 devel 정렬, 최신 code candidate Actions 통과, review·오늘할일 trailing 기록,
최종 head Actions/mergeable 재확인과 작업지시자 merge 승인이 필요하다.
원 source PR을 지금 close하거나 승인을 원격 게시하지 않는다.

시각 근거를 사용한 PR은 [Visual Sweep 정본](../../manual/verification/visual_sweep_guide.md) direct link와
위 실제 페이지·지표·사람 판정, `mydocs/pr/assets/`의 대표 PNG를 merge 후 comment에 포함한다.
raw image 링크 형식은 `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/<위-PNG>`다.
merge SHA·devel asset 반영을 확인한 다음 승인된 범위에서 UTF-8 Markdown `--body-file`로 게시하고
API로 본문·한글·고정 이미지 링크를 재확인한다. 이 문단은 게시 계획이며 게시 완료가 아니다.

[적용·후속 단계](pr_7113_review_impl.md)

## 별도 프로세스의 실제 tokenizer 반례

아래 코드를 임시 `repro.rs`로 저장한다. 프로덕션 rlib의 public API이며 구현 복제나 stub이 아니다.

```rust
use rhwp::renderer::equation::tokenizer;
fn main() {
 for script in [r#"rm "\TAB""#, r#"rm "literal \TAB label""#, r"a_{b} + c"] {
  let old = tokenizer::Tokenizer::new(script).tokenize();
  let new = tokenizer::tokenize(script);
  println!("script={script:?}\nbaseline={old:?}\ncandidate={new:?}");
 }
 if let Some(n) = std::env::args().nth(1) {
  let n: usize = n.parse().unwrap();
  let script = "\\BAR ".repeat(n) + "A " + &"\\TAB ".repeat(n);
  std::thread::Builder::new().stack_size(2 * 1024 * 1024).spawn(move || {
   println!("tokens={}", tokenizer::tokenize(&script).len());
  }).unwrap().join().unwrap();
 }
}
```

```sh
rustc --edition 2021 -O repro.rs \
  --extern rhwp=target/planet6897-review-20260914/release-test/deps/librhwp.rlib \
  -L dependency=target/planet6897-review-20260914/release-test/deps -o repro
./repro
./repro 20000
```

첫 실행에서 Quoted 값 손실, 두 번째에서 220002-byte 입력의 stack overflow/SIGABRT를 확인했다.
2MiB stack은 별도 worker의 현실적 제한을 명시한 것이며 기본 main stack에서도 같은 임계값이라고 주장하지 않는다.
