---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7104 — 다단 합성 LineSeg의 흐름 높이 검토

**최종 판정: 머지 보류.** F1 [P2, 실물 증거]: 같은 3쪽이라는 사실로 #6970 시각 통과를 선언할 수 없다. 1쪽 Square 교차 후보는 1→3개다. candidate 그림 bbox (576.5,231.7,96.5,120.5)에 TextLine x=595.0이 들어온다. 한컴의 대응 그림은 x=575.9..672.2, 본문은 x=672.2 이후에서 감싸 돈다. 익명화로 그림 자체가 흰색이어도 물리 영역과 본문 흐름 계약은 남는다. 단 채움 수정과 그림 exclusion/페이지 소속 잔여를 분리해 추가 교차가 회귀가 아닌지 입증하거나 보정해야 한다.

## 대상과 체리픽

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7104](https://github.com/edwardkim/rhwp/pull/7104) — 수정: 다단 합성 사다리 문단이 단 채움을 짧게 세어 단을 넘기지 않는다 (#6970) |
| 작성자 / reviewer | planet6897 / jangster77 사전 지정 |
| 원 base / 규모 | `devel`; 2 files, +183/-1 |
| source head | `add3a01b4a786da31663cc35746576495a96d357` |
| 최초 기준 devel | `93ffc3dd59c120bd54df4c2ac6d1ddbe630f8a2d` |
| 검증한 누적 code head | `48afe0f95abc4cd76b002dfc314f3cc044f10b31` — 12 PR / 16 commit |
| 원본·기준 PDF 보존 commit | `6933852a11b7e5998429eeb15708fdaeed7db626` |
| 최신 devel 정렬 | `037e4906a93e99896daa145a5ee5517824bfeaf4`; 정렬 후 로컬 head `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa` |
| 검토 branch / target | `review/planet6897-20260914` / `target/planet6897-review-20260914` |
| 원 PR 상태 snapshot | 2026-09-14T16:10:01.589135+09:00; OPEN, draft=False, merge=UNKNOWN |
| 관련 이슈 | [#6970](https://github.com/edwardkim/rhwp/issues/6970); 부분 반영 범위만 판단, 이번 작업에서 종료하지 않음 |

대상은 조사 시점 open·non-draft 12개다. draft #7098은 제외했다. 최종 재조회에서도 대상과 source SHA는 같았다.
원 PR의 성공/skip CI는 확인했지만 최신 통합 candidate의 GitHub Actions 결과로 재사용하지 않는다.
검토·체리픽은 로컬 작업이며 원격 PR 생성·push·merge·close는 아직 하지 않았다.

| 원 commit | 로컬 적용 commit | 보정 |
| --- | --- | --- |
| `add3a01b4a786da31663cc35746576495a96d357` | `0eedc515e34ebdd0301edd9f6bd71c07013ea5e0` | 충돌 없음; -x·저자 유지 |

### 실제 변경 경로

- `src/renderer/typeset.rs` (+17/-1)
- `tests/cases/issue_6970_multicolumn_synth_ladder_fill.rs` (+166/-0)

## 조판 원칙과 원인 계층

저장 사다리의 좌표 복원에 의존하는 trailing 간격 trim은 authoritative LineSeg에만 적용한다. 합성 사다리는 total_height를 실제 단 채움과 공유해야 한다.

주요 검토 위치: [src/renderer/typeset.rs](../../../src/renderer/typeset.rs).

| 공통 항목 | 판정 | 근거·제한 |
| --- | --- | --- |
| 구현 근거와 일반성 | 충족 | 저장 사다리의 좌표 복원에 의존하는 trailing 간격 trim은 authoritative LineSeg에만 적용한다. 합성 사다리는 total_height를 실제 단 채움과 공유해야 한다. |
| 측정·배치 일관성 | 미검증 | 공통 측정·배치 값과 한컴의 실제 위치를 다음 절에서 대조한다. #7104와 #7115의 미해결 줄/그림 경계는 통과로 보지 않는다. |
| 줄 소속과 점유 높이 | 미검증 | 저장 LineSeg·합성 재조판의 적용 조건과 실제 뒤 내용 위치를 함께 확인했다. 보류 항목은 원인과 해제 조건을 다음 절에 기록했다. |
| 사례와 증거의 독립성 | 미검증 | 합성 계약과 새 한컴 PDF 모두 확보했으나 그림 감싸기·추가 교차 후보의 수용 근거는 부족하다. |
| 기준값 변경 | 비해당 | 이 PR은 rendering baseline·golden·허용치를 변경하지 않는다. |
| 주장과 검증 범위 | 충족 | source SHA·실행 코드·실제 원본·명령·결과를 아래에 기록했다. 실행 회귀, 기존 잔여, 미검증을 구분하고 원 PR CI를 통합 CI로 재사용하지 않았다. |
| 실제 입력 커밋 | 충족 | 개인 다운로드 경로만 남기지 않고 Git object 또는 LFS oid와 실제 SHA-256을 대조했다. 기존 동일 파일은 재추가하지 않았다. |

## 직접 실행·시각 판정

합성 90문단 계약 2개가 통과한다. 보고서의 243200-byte 익명화 HWP도 확보·커밋하고 새 한컴 2020 PDF 3쪽과 전후 대조했다. 단 하단 유출과 큰 줄 순서 drift는 감소한다. 1→2쪽 조기 이동 문자 후보는 156→10, 2→3쪽은 363→114로 줄지만 남는다. 2·3쪽 raster도 직접 읽었으며 제목·본문의 단/페이지 소속은 한컴과 여전히 크게 달랐다.

F1 [P2, 실물 증거]: 같은 3쪽이라는 사실로 #6970 시각 통과를 선언할 수 없다. 1쪽 Square 교차 후보는 1→3개다. candidate 그림 bbox (576.5,231.7,96.5,120.5)에 TextLine x=595.0이 들어온다. 한컴의 대응 그림은 x=575.9..672.2, 본문은 x=672.2 이후에서 감싸 돈다. 익명화로 그림 자체가 흰색이어도 물리 영역과 본문 흐름 계약은 남는다. 단 채움 수정과 그림 exclusion/페이지 소속 잔여를 분리해 추가 교차가 회귀가 아닌지 입증하거나 보정해야 한다.

**잔여·미검증:** 원 PR은 관련 #6970으로 부분 범위를 명시했다. 기존 결함 전체를 이 PR이 만들었다고 판단하지 않는다. 현재 보류는 실제 입력에서 그림·본문 흐름의 수용 근거가 부족한 점이며 합성 회계 테스트 실패가 아니다.

시각 검증 필요: **예**. 전체 문서의 SVG·render tree·문자/레이아웃 ledger를 생성하고, 아래 페이지를 96dpi로 한컴 PDF / base / 통합 세 방향으로 직접 읽었다. 페이지 수·픽셀 점수만으로 통과시키지 않았다.

- synth-no-lineseg-p001: [한컴 / 변경 전 / 통합 비교](../assets/pr7104_synth-no-lineseg-p001_3way.png), [한컴·통합 overlay](../assets/pr7104_synth-no-lineseg-p001_ovl.png)
- synth-no-lineseg-p002: [한컴 / 변경 전 / 통합 비교](../assets/pr7104_synth-no-lineseg-p002_3way.png), [한컴·통합 overlay](../assets/pr7104_synth-no-lineseg-p002_ovl.png)
- synth-no-lineseg-p003: [한컴 / 변경 전 / 통합 비교](../assets/pr7104_synth-no-lineseg-p003_3way.png), [한컴·통합 overlay](../assets/pr7104_synth-no-lineseg-p003_ovl.png)

원본 PDF가 내장하지 않은 글꼴의 기존 매핑, editor-only placeholder, 선 굵기 차이는 전체 일치로 판정하지 않았다.
단일 쪽 SVG가 `_001` 없이 저장되면 fidelity helper의 파일명 기반 쪽수가 0으로 표시되는 경우가 있었다.
실제 SVG·render-tree 파일과 한컴 PDF 1쪽을 확인해 계수 오류와 렌더 실패를 구분했다.

## 입력·기준 출력 보존

확인 tree: `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa`. 다음 SHA-256은 LFS pointer 문자열이 아닌 실제 파일 바이트의 해시이며,
Git object 또는 LFS oid와 일치한다. full/OVR 공통 입력은 아래 재현 명령의 저장소 fixture 집합을 사용했다.

| 저장소 파일 | 출처·역할 | SHA-256 |
| --- | --- | --- |
| [tests/fixtures/issue_6970/synth_no_ls_square_wrap.hwp](../../../tests/fixtures/issue_6970/synth_no_ls_square_wrap.hwp) | 한컴 입력; synth-no-lineseg | `31a5b76148718d92e3fd150f4d68b84b5005b02d16863c4735a6178eb02f2799` |
| [pdf/pr-planet6897-20260914/synth-no-lineseg-hwp-2020.pdf](../../../pdf/pr-planet6897-20260914/synth-no-lineseg-hwp-2020.pdf) | 독립 한컴 PDF 3쪽; 직접 sweep 1-3쪽 | `70b63c11514d3d927418c1f116a0cc421614e7db5e9de312bd0d9f964b34e9a3` |

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

[적용·후속 단계](pr_7104_review_impl.md)
