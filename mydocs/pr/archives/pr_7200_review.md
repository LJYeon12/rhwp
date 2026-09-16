---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7200_review.md
last_verified: 2026-09-16
---

# PR #7200 검토

## 최종 판정

**머지 보류 — 빈 선행 문단 뒤 중첩 표의 Center/Bottom 정렬 회귀 1건(P2).**
기존 코드에서는 통과하는 동일 입력의 정렬 불변식이 PR 적용 Native와 fresh WASM에서 모두 실패했다.
원 PR focused 3개와 원 head CI 성공을 이 경계의 무회귀 근거로 확대하지 않는다.
이번 작업은 검토와 증적 보존이다. 엔진 보정·원격 push·GitHub review/comment 게시·merge는 하지 않았다.

사용자가 비공개 실제 7쪽 양식 대신 **함께 커밋된 HWP 파일로 대체**하도록 지정했다.
이에 `width-top.hwp`, `width-center.hwp`, `width-bottom.hwp`를 한컴 PDF로 변환해 전후 비교했다.
비공개 원본 부재 자체를 이번 보류 사유로 삼지 않는다. 그 원본 7쪽 개선은 reviewer 미검증이다.

## 검토 대상과 계보

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7200](https://github.com/edwardkim/rhwp/pull/7200), 저장 위치가 초기화된 중첩 표 정렬과 표 뒤 문단 흐름 수정 |
| 작성자 / reviewer | LJYeon12 / jangster77 사전 할당 완료 |
| 기여 이력 | #7107 병합 이력이 있어 첫 기여자 처리 대상 아님 |
| 원 head | `2543382e757967183be1938893ac2c5be2c0c16e` |
| 검토 base | `222c8c4819de414898bf5b15608b0d0d394a3748` |
| 로컬 branch | `codex/pr7200-review-20260916` |
| 통합 code head | `25e0d8c747ff6f38117c5ef1899b4caad50f3a52` |
| 체리픽 | `a2c1f814e` → `4e8c087a0`, `2543382e7` → `25e0d8c74`, 모두 `-x`, 충돌 없음 |
| 범위 | 엔진 4파일, focused 1파일/3함수, 합성 입력 28개와 README; baseline 변경 없음 |
| 원격 재확인 | 원 head 불변, OPEN/non-draft, CLEAN; maintainer_can_modify=true |

route: collaborator_external_pr + intake_and_review + local_validation + visual_fixture_evidence.
fork 직접 push 가능 여부는 이번 검토에서 dry-run하지 않았다. maintainer 수정 허용과 실제 push 성공은 구분한다.

## P2 — 실제 배치가 사용하지 않는 빈 문단 높이를 정렬 측정에 더함

변경 위치: `src/renderer/layout/table_layout.rs:8408–8419`.
새 `calc_nested_controls_bottom_height`는 저장 vpos 사다리가 무너진 셀에서 일반 문단 높이를
`flow_y`에 무조건 더한다. 그러나 같은 파일의 실제 중첩 표 배치 `:6787–6793`은
`has_preceding_text == false`이면 누적 `para_y` 대신 `inner_area.y`를 사용한다.
따라서 첫 문단이 빈 문단이면 측정에만 그 줄의 높이 16px가 포함된다.
이 과대 측정이 Center 여유를 8px, Bottom 여유를 16px 줄인다.

[재현 입력 3개와 생성 내역](../../../tests/fixtures/pr7200_empty_leading_paragraph/README.md)은
원 PR `cell-*-collapsed.hwpx`의 첫 `Start` 텍스트만 비웠다. 저장 줄과 표 구조는 유지했다.
Top의 실제 중첩 표 하단이 179.3px, 부모 셀 하단이 286.0px이므로 여유는 106.7px이다.
정렬 이외에는 동일한 입력이므로 Center는 53.35px, Bottom은 106.7px 이동해야 한다.
이는 원 PR이 사용한 Top 점유 영역 기반 기하 계약이며, 합성 파일의 한컴 외형 기대값이 아니다.

| 실행 | Top 표 y | Center 표 y / 이동량 | Bottom 표 y / 이동량 | 판정 |
| --- | ---: | ---: | ---: | --- |
| base Native | 126.0 | 179.4 / 53.4 | 232.7 / 106.7 | PASS, exit 0 |
| PR Native | 126.0 | 171.4 / 45.4 | 216.7 / 90.7 | FAIL, exit 1 |
| PR fresh WASM | 126.0 | 171.4 / 45.4 | 216.7 / 90.7 | FAIL, exit 1 |

[재현 검사 스크립트](../assets/pr7200_review/check_empty_leading_paragraph.py)는 성공 시 0,
이 회귀 시 1을 반환한다. 허용 오차 0.6px는 기존 PR assertion과 같다. 예외 허용치를 늘리지 않았다.
보류 해제에는 측정과 배치가 같은 실제 콘텐츠 배치 결과를 사용하도록 보정하고 이 경계의
수정 전 FAIL / 보정 후 PASS 및 기존 원 PR 경계의 보존을 확인해야 한다.
`has_preceding_text` 제한을 새 측정 휴리스틱으로 단순 복제하는 것으로 공통 결과를 대체하지 않는다.

## 검증 실행

| 항목 | 결과 |
| --- | --- |
| 원 head 전체 CI | [35088688931](https://github.com/edwardkim/rhwp/actions/runs/35088688931), 정확한 head `2543382e7`, Full 성공; Archive A/B/C/D, Lint, Native Skia 성공 |
| 동반 CI | [CodeQL](https://github.com/edwardkim/rhwp/actions/runs/35088688936), [Render Diff](https://github.com/edwardkim/rhwp/actions/runs/35088688794), [Adapter](https://github.com/edwardkim/rhwp/actions/runs/35088688970), [Proptest](https://github.com/edwardkim/rhwp/actions/runs/35088689038) 성공 |
| 로컬 Native | `cargo build --locked --bin rhwp`, exit 0 |
| 원 PR focused | `cargo test --locked --test regression_suite_007 stored_nested_content_flow -- --nocapture`, 3 PASS, 0 FAIL, 194 filtered, exit 0 |
| fresh WASM | `scripts/wasm-pack-locked.sh --target web --out-dir /private/tmp/rhwp-pr7200-review-20260916/wasm-pkg`, release + wasm-opt, exit 0 |
| WASM 실제 실행 | `export-wasm-for-sweep.mjs`, Chrome/153.0.8010.47, 3개 대체 HWP와 3개 빈 문단 경계 |
| Visual Sweep | 같은 PDF에 base Native / PR Native / PR WASM 각각 3문서×1쪽, compare·standalone overlay·review 생성, 누락 0, 직접 판독 |
| 경계 검사 | 위 표: base PASS / PR Native·WASM FAIL |

환경: macOS, `DEVELOPER_DIR=/Library/Developer/CommandLineTools`, 전용
`CARGO_TARGET_DIR=/Users/tsjang/rhwp/target/pr7200-review-20260916`.
Rust 빌드는 순차 실행했다. 검토 중 엔진 코드를 변경하지 않았고 전체 회귀·Clippy·Native Skia는
성공한 원 head CI를 재사용했다. 통합 head 전체 CI를 새로 실행했다고 표현하지 않는다.
upstream WASM job은 skip이므로 이를 성공한 fresh WASM 빌드라고 세지 않았으며 위 로컬 빌드를 사용했다.

base 비교 CLI는 앞선 작업에서 보존한 `/private/tmp/rhwp-6611-20260916/rhwp-base`다.
그 빌드 source `78340390d`와 검토 base `222c8c4819`의 `src`, `crates`, `Cargo.toml`,
`Cargo.lock` 차이가 없음을 확인했다. Sweep manifest의 작업공간 HEAD를 이 base 바이너리의
빌드 SHA로 혼동하지 않는다.

| 산출물 | SHA256 |
| --- | --- |
| base CLI | `70b624cdeedf30857434bbdb4e13dbb82883e55a795b6a2b6c20ba7b4cf6a46a` |
| PR CLI | `b9a3959536be04e578ecc5c126636613c3b4836c26e6d45d25d01a9eb1ba3f10` |
| fresh rhwp.js | `707049ae519de26779c842eec40f94e016b09bda2688fa721a37d2ae670c2f26` |
| fresh rhwp_bg.wasm | `103d2a2b09bad9580c984018cb97f08def722a869a89c094acef4fafb46e081c` |

## 대체 HWP의 한컴 PDF와 Visual Sweep

기존 Git 입력 `samples/stored-nested-content-flow/width-{top,center,bottom}.hwp`를 재사용했다.
`rhwp info --json`의 저장 메타데이터는 `hancom-office-2010 / 8.0.0.466`이나 빈 문서에서 합성한
입력이므로 실제 해당 제품으로 저장했다는 증거로 삼지 않는다. 정책상 구버전 호환 engine `2020`을
선택했고 실제 응답은 **Hancom 12.0.0.4605**, `input_preprocess=none`, 폰트 등록 실패 0이었다.
비동기 start → status(succeeded 확인) → download를 사용했다. PDF는 모두 1쪽이다.

| 입력 | PDF | 변환 job |
| --- | --- | --- |
| width-top.hwp | [Top](../../../pdf/pr7200/pr7200-width-top-2020.pdf) | `18f5c1c6-fcec-4431-8dd6-50ecc410c759` |
| width-center.hwp | [Center](../../../pdf/pr7200/pr7200-width-center-2020.pdf) | `9c0ecf97-f437-4042-aa73-7547949cbe93` |
| width-bottom.hwp | [Bottom](../../../pdf/pr7200/pr7200-width-bottom-2020.pdf) | `2f7facd2-3111-4c9d-92bd-52988fbcade0` |

96 DPI, Chrome webfont raster, threshold 32. PR Native와 WASM의 각 페이지 PNG는 SHA256이 같다.
따라서 같은 페이지 이미지를 중복 보관하지 않고 fresh WASM 대표 증적과 base 증적을 보존했다.
원 HWP를 복사/이름 변경해 추가하지 않았다. 새 PDF 3개와 새 경계 입력 3개만 추가한다.

| 입력 1쪽 | base pixel / ink match (%) | PR Native·WASM pixel / ink match (%) | PR 증적 | base 증적 |
| --- | --- | --- | --- | --- |
| Top | 94.83490 / 6.62398 | 94.83490 / 6.62398 | [compare](../assets/pr7200_review/wasm_width_top_compare_001.png) · [overlay](../assets/pr7200_review/wasm_width_top_overlay_001.png) · [review](../assets/pr7200_review/wasm_width_top_review_001.png) | [compare](../assets/pr7200_review/base_width_top_compare_001.png) · [overlay](../assets/pr7200_review/base_width_top_overlay_001.png) · [review](../assets/pr7200_review/base_width_top_review_001.png) |
| Center | 94.54531 / 6.01274 | 94.65260 / 6.17747 | [compare](../assets/pr7200_review/wasm_width_center_compare_001.png) · [overlay](../assets/pr7200_review/wasm_width_center_overlay_001.png) · [review](../assets/pr7200_review/wasm_width_center_review_001.png) | [compare](../assets/pr7200_review/base_width_center_compare_001.png) · [overlay](../assets/pr7200_review/base_width_center_overlay_001.png) · [review](../assets/pr7200_review/base_width_center_review_001.png) |
| Bottom | 94.78802 / 6.54651 | 94.49635 / 5.47878 | [compare](../assets/pr7200_review/wasm_width_bottom_compare_001.png) · [overlay](../assets/pr7200_review/wasm_width_bottom_overlay_001.png) · [review](../assets/pr7200_review/wasm_width_bottom_review_001.png) | [compare](../assets/pr7200_review/base_width_bottom_compare_001.png) · [overlay](../assets/pr7200_review/base_width_bottom_overlay_001.png) · [review](../assets/pr7200_review/base_width_bottom_review_001.png) |

**직접 판정: PDF 시각 일치 미충족.** 한컴 PDF 자체에서도 `Wide text`가 한 줄에 중복 인쇄된 것처럼
겹친다. rhwp는 이를 여러 줄로 재조판하며 표의 가로 위치·테두리·문단 위치도 다르다.
이 차이는 base에도 존재하고, PR은 Center/Bottom의 콘텐츠 이동량을 바꾼다. Top은 변하지 않았다.
숫자가 조금 좋아지거나 자동 후보가 0이라는 이유로 한컴 재현 성공이라고 판정하지 않는다.
합성 HWP의 비정상 저장 줄을 한컴의 정상 문서 조판 기준으로 일반화할 수 없다.
이 자료의 한계는 앞서 실행으로 확인한 새 빈 문단 정렬 회귀와 분리한다.

## 조판 원칙 및 범위 심사

| 주장 / 소비 경로 | 판정과 근거 |
| --- | --- |
| 초기화된 저장 위치에서 앞 문단 높이 계상 | 기존 3함수의 공개 경계 PASS. 그러나 빈 문단에서 실제 배치와 불일치: 위 P2 **미충족** |
| 배경·어울림 표 흐름 공유 | `nested_table_is_overlay`, `nested_table_flow_advance`를 측정과 실제 중첩 표 배치가 공유. 공개 overlay/square/flow 경계 PASS |
| 글 폭에 따른 앞 문단 재조판 | 기존 composed lines 사용, width 기하 계약 PASS. 한컴 시각 일치는 위와 같이 미충족 |
| HWP5 본문 표 소속 줄 선택 | `layout.rs`의 `projected_seg`가 HWP5 경로로 확대됨. 가시 선행 객체·표 높이 가드 유지. 음수/0 줄간격 및 순수 HWPX 통제 focused PASS |
| 일반 셀·부분 셀·continuation | `table_layout.rs`, `table_cell_content.rs`, `table_partial.rs`의 caller가 구성 문단을 전달. 구성 길이가 다른 continuation은 새 sequential 분기를 타지 않음. 실제 다쪽 분할·이어받기 경계는 **미검증**, 1쪽 합성으로 대체하지 않음 |
| baseline·문서별 예외 | 변경 없음. 특정 문서 ID 분기 없음. 다만 helper 공유만으로 모든 측정/배치 일치가 입증되지는 않음 |
| 실제 비공개 7쪽 / 저장 후 재열기 | 작성자 주장으로만 남김. reviewer는 사용자 지정 대체 HWP 3개로 검증 |

추가 탐색에서 같은 호스트의 표 2개는 이번 변경으로 정렬이 개선됐다. 뒤 텍스트가 있는 별도 합성
변형의 기존 넘침은 base에서도 재현돼 새 회귀로 세지 않았다. 최종 보류 사유는 위 P2 1건이다.

## 재현 명령과 후속 범위

```bash
python3 mydocs/pr/assets/pr7200_review/check_empty_leading_paragraph.py \
  --rhwp-bin target/pr7200-review-20260916/debug/rhwp \
  --trees /private/tmp/pr7200-check

venv/bin/python scripts/visual_sweep.py \
  --file-target width-top samples/stored-nested-content-flow/width-top.hwp pdf/pr7200/pr7200-width-top-2020.pdf \
  --file-target width-center samples/stored-nested-content-flow/width-center.hwp pdf/pr7200/pr7200-width-center-2020.pdf \
  --file-target width-bottom samples/stored-nested-content-flow/width-bottom.hwp pdf/pr7200/pr7200-width-bottom-2020.pdf \
  --rhwp-bin target/pr7200-review-20260916/debug/rhwp \
  --wasm-pkg /private/tmp/rhwp-pr7200-review-20260916/wasm-pkg \
  --pages 1 --out /private/tmp/pr7200-sweep
```

Native 비교는 `--wasm-pkg`를 생략하고 base는 위 base CLI를 지정했다.
전체 실행 원시 자료는 `/private/tmp/rhwp-pr7200-review-20260916`에 있으며 raw log/JSON/TSV/SVG는
커밋하지 않는다. 현재 검토 branch와 전용 target은 후속 보정을 위해 유지한다.
보정·최종 head CI·원격 조치·병합 후 comment 계획 확정은 다음 단계이며 완료로 기록하지 않는다.
