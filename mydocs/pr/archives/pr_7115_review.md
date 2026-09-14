---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7115 — 아래아 U+318D 전각 fallback 검토

**최종 판정: 머지 보류.** F1 [P1, 실행 시각 회귀]: 80168 108쪽 오른쪽 셀의 `9. 그 밖에 시ㆍ도조례로 정하는 사항`이 두 줄이 되면서 이후 내용이 33.6px 아래로 밀린다. 표 bbox는 전후 x=75.6,y=107.6,w=640.4,h=910.0으로 동일하다. 한컴/변경 전의 108쪽 끝 `3. 그 밖에 시ㆍ도조례로 정하는 사항`이 candidate 109쪽 맨 위로 이동한다. 유효 저장 줄의 측정·배치/재조판 경계를 점검하고 해당 페이지 소속을 독립 PDF 회귀로 고정해야 한다. 76→77쪽 기존 차이도 커져 함께 검토 대상이다.

## 대상과 체리픽

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7115](https://github.com/edwardkim/rhwp/pull/7115) — 수정: ㆍ(U+318D) 를 반각으로 박던 폴백을 전각으로 되돌린다 (#7080) |
| 작성자 / reviewer | planet6897 / jangster77 사전 지정 |
| 원 base / 규모 | `devel`; 3 files, +212/-60 |
| source head | `fa94b52a07de08af9d4320ab911caefa19e323dd` |
| 최초 기준 devel | `93ffc3dd59c120bd54df4c2ac6d1ddbe630f8a2d` |
| 검증한 누적 code head | `48afe0f95abc4cd76b002dfc314f3cc044f10b31` — 12 PR / 16 commit |
| 원본·기준 PDF 보존 commit | `6933852a11b7e5998429eeb15708fdaeed7db626` |
| 최신 devel 정렬 | `037e4906a93e99896daa145a5ee5517824bfeaf4`; 정렬 후 로컬 head `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa` |
| 검토 branch / target | `review/planet6897-20260914` / `target/planet6897-review-20260914` |
| 원 PR 상태 snapshot | 2026-09-14T16:10:01.589135+09:00; OPEN, draft=False, merge=UNKNOWN |
| 관련 이슈 | [#7080](https://github.com/edwardkim/rhwp/issues/7080); 부분 반영 범위만 판단, 이번 작업에서 종료하지 않음 |

대상은 조사 시점 open·non-draft 12개다. draft #7098은 제외했다. 최종 재조회에서도 대상과 source SHA는 같았다.
원 PR의 성공/skip CI는 확인했지만 최신 통합 candidate의 GitHub Actions 결과로 재사용하지 않는다.
검토·체리픽은 로컬 작업이며 원격 PR 생성·push·merge·close는 아직 하지 않았다.

| 원 commit | 로컬 적용 commit | 보정 |
| --- | --- | --- |
| `fa94b52a07de08af9d4320ab911caefa19e323dd` | `8a90acbabcfa3a1fe503b5a7a9b9dc8e88b74e65` | 충돌 없음; -x·저자 유지 |

### 실제 변경 경로

- `src/renderer/layout/text_measurement.rs` (+42/-11)
- `tests/cases/issue_7080_area_dot_fullwidth.rs` (+121/-0)
- `tests/golden_svg/form-002/page-0.svg` (+49/-49)

## 조판 원칙과 원인 계층

U+318D의 fallback 전진폭은 독립 글꼴·한컴 출력에 따라 정해야 하며, 올바른 glyph 폭을 적용한 뒤에도 유효 저장 줄 소속과 페이지 경계를 보존해야 한다. 글자폭을 다시 줄여 페이지 차이를 숨기는 보정은 수용하지 않는다.

주요 검토 위치: [src/renderer/layout/text_measurement.rs](../../../src/renderer/layout/text_measurement.rs).

| 공통 항목 | 판정 | 근거·제한 |
| --- | --- | --- |
| 구현 근거와 일반성 | 충족 | U+318D의 fallback 전진폭은 독립 글꼴·한컴 출력에 따라 정해야 하며, 올바른 glyph 폭을 적용한 뒤에도 유효 저장 줄 소속과 페이지 경계를 보존해야 한다. 글자폭을 다시 줄여 페이지 차이를 숨기는 보정은 수용하지 않는다. |
| 측정·배치 일관성 | 미충족 | 공통 측정·배치 값과 한컴의 실제 위치를 다음 절에서 대조한다. #7104와 #7115의 미해결 줄/그림 경계는 통과로 보지 않는다. |
| 줄 소속과 점유 높이 | 미충족 | 저장 LineSeg·합성 재조판의 적용 조건과 실제 뒤 내용 위치를 함께 확인했다. 보류 항목은 원인과 해제 조건을 다음 절에 기록했다. |
| 사례와 증거의 독립성 | 충족 | 합성 계약과 실제 원본·한컴 PDF/직렬화/API·진단 증거를 분리했다. 실행 결함이 발견된 경우에도 정상 샘플 통과로 상쇄하지 않았다. |
| 기준값 변경 | 충족 | 원 PR baseline/golden diff와 독립 PDF·실제 진단 증가를 대조했다. #7115의 올바른 글자폭 golden은 다른 페이지 소속 회귀의 승인 근거가 아니다. 메인터너가 실패를 숨기려고 추가 갱신한 baseline은 없다. |
| 주장과 검증 범위 | 충족 | source SHA·실행 코드·실제 원본·명령·결과를 아래에 기록했다. 실행 회귀, 기존 잔여, 미검증을 구분하고 원 PR CI를 통합 CI로 재사용하지 않았다. |
| 실제 입력 커밋 | 충족 | 개인 다운로드 경로만 남기지 않고 Git object 또는 LFS oid와 실제 SHA-256을 대조했다. 기존 동일 파일은 재추가하지 않았다. |

## 직접 실행·시각 판정

form-002의 아래아 전진과 80168의 아래아 run은 전각 쪽으로 개선된다. 동시에 80168 157쪽 전수 문자·layout ledger가 108→109쪽의 새 16문자 이동을 검출했고 두 쪽의 PDF/전후 raster에서 확인했다. baseline은 두 쪽 모두 문자 차이 0, candidate는 각각 reference-only 16 / SVG-only 16이다.

F1 [P1, 실행 시각 회귀]: 80168 108쪽 오른쪽 셀의 `9. 그 밖에 시ㆍ도조례로 정하는 사항`이 두 줄이 되면서 이후 내용이 33.6px 아래로 밀린다. 표 bbox는 전후 x=75.6,y=107.6,w=640.4,h=910.0으로 동일하다. 한컴/변경 전의 108쪽 끝 `3. 그 밖에 시ㆍ도조례로 정하는 사항`이 candidate 109쪽 맨 위로 이동한다. 유효 저장 줄의 측정·배치/재조판 경계를 점검하고 해당 페이지 소속을 독립 PDF 회귀로 고정해야 한다. 76→77쪽 기존 차이도 커져 함께 검토 대상이다.

**잔여·미검증:** 페이지 총수 157과 SVG golden 통과로 이 회귀를 감추지 않는다. #7117과 동시에 바뀐 form-002는 두 효과를 함께 대조했으며 golden을 임의 갱신하지 않았다. U+318D fallback 함수만 되돌린 대조 빌드에서 9번 문장이 한 줄로 복구되고 3번 문장이 108쪽 y=981.9로 돌아왔다. 원인 귀속을 확인했으며 실험 후 source는 동일 해시로 복구했다.

시각 검증 필요: **예**. 전체 문서의 SVG·render tree·문자/레이아웃 ledger를 생성하고, 아래 페이지를 96dpi로 한컴 PDF / base / 통합 세 방향으로 직접 읽었다. 페이지 수·픽셀 점수만으로 통과시키지 않았다.

- form002-p001: [한컴 / 변경 전 / 통합 비교](../assets/pr7115_form002-p001_3way.png), [한컴·통합 overlay](../assets/pr7115_form002-p001_ovl.png)
- reg80168-p108: [한컴 / 변경 전 / 통합 비교](../assets/pr7115_reg80168-p108_3way.png), [한컴·통합 overlay](../assets/pr7115_reg80168-p108_ovl.png)
- reg80168-p109: [한컴 / 변경 전 / 통합 비교](../assets/pr7115_reg80168-p109_3way.png), [한컴·통합 overlay](../assets/pr7115_reg80168-p109_ovl.png)
- reg80168-p076: [한컴 / 변경 전 / 통합 비교](../assets/pr7115_reg80168-p076_3way.png), [한컴·통합 overlay](../assets/pr7115_reg80168-p076_ovl.png)
- reg80168-p077: [한컴 / 변경 전 / 통합 비교](../assets/pr7115_reg80168-p077_3way.png), [한컴·통합 overlay](../assets/pr7115_reg80168-p077_ovl.png)

원본 PDF가 내장하지 않은 글꼴의 기존 매핑, editor-only placeholder, 선 굵기 차이는 전체 일치로 판정하지 않았다.
단일 쪽 SVG가 `_001` 없이 저장되면 fidelity helper의 파일명 기반 쪽수가 0으로 표시되는 경우가 있었다.
실제 SVG·render-tree 파일과 한컴 PDF 1쪽을 확인해 계수 오류와 렌더 실패를 구분했다.

## 입력·기준 출력 보존

확인 tree: `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa`. 다음 SHA-256은 LFS pointer 문자열이 아닌 실제 파일 바이트의 해시이며,
Git object 또는 LFS oid와 일치한다. full/OVR 공통 입력은 아래 재현 명령의 저장소 fixture 집합을 사용했다.

| 저장소 파일 | 출처·역할 | SHA-256 |
| --- | --- | --- |
| [samples/hwpx/form-002.hwpx](../../../samples/hwpx/form-002.hwpx) | 한컴 입력; form002 | `5ab8f7c368e02538f75f1cd2bd82bbd8de2f925a54ba7b38ec9395b2cdb804d4` |
| [pdf/hwpx/form-002-2022.pdf](../../../pdf/hwpx/form-002-2022.pdf) | 독립 한컴 PDF 10쪽; 직접 sweep 1쪽 | `629f1d93be234e4c4c551d319e247c1158d225cfe8a86bb179754a1b6cf2e077` |
| [samples/80168_regulatory_analysis.hwp](../../../samples/80168_regulatory_analysis.hwp) | 한컴 입력; reg80168 | `c8ad10fe9f07be5119cd804278017aefa46e555bbee4f05f0f5132fe4f591a22` |
| [pdf/80168_regulatory_analysis-2022.pdf](../../../pdf/80168_regulatory_analysis-2022.pdf) | 독립 한컴 PDF 157쪽; 직접 sweep 1,29,76-77,108-109쪽 | `7af457d9ec502132b1035582c16b1ba783e7faff71da0feef202690382bb1b95` |
| [samples/86712_regulatory_analysis.hwp](../../../samples/86712_regulatory_analysis.hwp) | 원 PR focused 계약에 사용한 기존 입력 | `32e2ed30e5d744ad747f04f090c022eca8270f9dd2d55e0613e2ad61058099e9` |
| [samples/issue6031/3249937_asset_management_rules.hwpx](../../../samples/issue6031/3249937_asset_management_rules.hwpx) | 원 PR focused 계약에 사용한 기존 입력 | `97b5d6c571a6b7626321c6a53d797e3511978447497bb73309bd23eaa8e7ea77` |

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

[적용·후속 단계](pr_7115_review_impl.md)

## 단일 변수 음성 대조

통합 code head에서 **`area_dot_fallback_width` 함수 하나만** 최초 devel의 구현으로 되돌렸다.
다른 PR의 코드·입력·빌드 profile·feature는 같았다. release-test CLI build와 157쪽 SVG/render-tree export가 모두 exit 0이었다.

| 관측 | 변경 전 devel | 통합 candidate | U+318D 함수만 rollback |
| --- | --- | --- | --- |
| 108쪽 오른쪽 셀 9번 | y=746.7 한 줄, `사항`까지 | y=746.7 `사`까지 + y=780.3 `항` | y=746.7 한 줄, `사항`까지 |
| 3번 문장의 쪽·y | 108쪽 y=981.9 | 109쪽 y=109.9 + 143.5 | 108쪽 y=981.9 |

따라서 이번 누적 변경에서 새 108→109쪽 이동은 #7115의 U+318D fallback 변경으로 유발됨을 단일 변수로 확인했다.
음성 대조의 반각 값을 정식 해결책으로 채택하지 않았다. 한컴의 전각 glyph 폭과 저장 줄 소속을 함께 보존하는 보정이 필요하다.
실험 후 source를 바이트 단위로 원복했다. 복구 전후 파일 SHA-256은
`15e553a2e893cd0b30e9883d887ad3f3aa6b039b8b678570f8595cb910e4a964`로 같다.
