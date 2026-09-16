---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7200_review.md
last_verified: 2026-09-16
---

# PR #7200 검토

## 최종 판정

**머지 보류 — 메인터너 보정 진행 중, PDF 시각 일치 미충족.**
빈 선행 문단의 측정/배치 불일치에 더해, 사용자가 overlay의 표 위치·누락된 테두리 차이도
해결하도록 요청했다. 작은 정렬 불변식의 통과만으로 보류를 해제하지 않는다.
전체 nextest는 사용자 요청으로 중단했다(exit 143). 완료된 전체 검증으로 세지 않는다.
현재 작업 트리 보정으로 focused 9개와 Native/fresh WASM 13쪽 캡처를 완료했다.
전체 회귀·Clippy·Native Skia 검증은 재개하지 않았으며 최종 push 준비 완료가 아니다.
원격 push·GitHub comment·merge는 수행하지 않았다. 아래 원 PR 검토 결과는 수정 전 증거로 보존한다.

진행 기록: [메인터너 보정 1회차](../../working/task_m100_7200_stage1.md).

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
변형의 기존 넘침은 base에서도 재현돼 새 회귀로 세지 않았다. 최초 검토 당시 보류 사유는 위 P2 1건이었다. 사용자 요청 이후 시각 차이도 보류 범위에 포함했다.

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
이하 메인터너 보정 결과를 추가했다. 최종 head CI·원격 조치·병합은 수행하지 않았다.


## 메인터너 보정 — 시각 차이 직접 확인 후 부분 수정

사용자 요청으로 실행 중이던 전체 nextest와 소유 하위 프로세스를 중단했다(exit **143**).
정렬 불변식만 통과한 상태에서 큰 PDF 차이를 남기고 전체 테스트를 진행한 것은 잘못된 순서였다.
이후 영향 페이지의 직접 비교를 우선했고 전체 회귀를 재실행하지 않았다.

### 수정한 원인과 현재 결과

- 빈 줄의 점유 공간과 가시 텍스트 유무가 다르다. 측정/배치가 `SequentialNestedCellLayout`의
  원점·하단을 함께 소비하게 하여 빈 선행 줄 16px를 실제 중첩 표 배치에도 보존했다.
- 좁은 비-TAC 표를 무조건 가운데 놓던 경로가 글 앞으로/뒤로 표의 명시한 가로 앵커도 무시했다.
  Para/Column 기준 overlay의 Left/Center/Right와 offset을 반영했다. 빈 문단 입력의 x는
  **128 → 48px**, 한컴 PDF는 **48.32px**다. 기존 Square 경계 #5787은 통과했다.
- 본문 테두리 큐에 셀 문단이 섞이는 것을 막기 위해 셀의 문단 테두리를 통째로 생략하고 있었다.
  셀별 큐를 분리하고 중첩 셀/부모/본문의 범위를 복원한다. 테두리를 가진 빈 block 표 호스트도
  배치하고, 별도 PageItem인 TAC 표도 본문 문단 테두리 점유 범위에 넣었다.
- `border_connect`를 실제 해소 스타일과 병합 판정에 전달하여 연결이 꺼진 문단을 임의로 합치지 않는다.
  셀 마지막 문단은 정렬 측정에서 제외하는 후행 줄간격까지 테두리로 감싸되, 그 paint 범위를
  흐름 전진에 이중 가산하지 않는다.
- 정상 한컴 저장 줄의 `line_height=1000 / text_height=1056 / spacing=632`에서 56HU를 놓쳤다.
  일반 구성 줄에 실제 텍스트 점유 높이도 포함하여 측정과 paint가 함께 사용한다.

### 원본 합성과 정상 한컴 대조군을 구분

원본 `width-*.hwp`의 200자 문단은 저장 줄이 1개여서 한컴 PDF에서도 겹친다. 원본과 기존 PDF를
바꾸거나 숨기지 않았다. [한컴 재조판 대조군과 생성 절차](../../../tests/fixtures/pr7200_hancom_recomposed/README.md)는
저장 줄 캐시만 제거한 뒤 한컴으로 다시 저장한 **새로운 입력**이다. 텍스트·표·셀/문단 테두리를
보존했고 Top/Center/Bottom 각각의 최종 HWP와 PDF를 함께 보존한다. 단순 이름 변경 복사본이 아니다.
원본의 비정상 PDF를 모방하려고 renderer의 정상 줄 나눔을 일부러 겹치게 하지 않았다.
**대조군의 개선은 원본 HWP/PDF의 일치 증거가 아니다.**

| 정상 대조군 | 원 PR End x/y | 보정 End x/y | 한컴 PDF End x/y |
| --- | ---: | ---: | ---: |
| Top | 128 / 256.0 | 48 / 259.7 | 48.774 / 259.734 |
| Center | 128 / 373.1 | 48 / 376.8 | 48.774 / 376.566 |
| Bottom | 128 / 490.1 | 48 / 493.9 | 48.774 / 493.558 |

기존 구현의 5줄 y는 Top에서 127.2 / 149.0 / 170.7 / 192.5 / 214.2px였고,
보정 후 127.2 / 149.7 / 172.2 / 194.7 / 217.2px다. PDF 상단은
127.718 / 150.094 / 172.629 / 195.164 / 217.540px다.
정식 검사에 3개 정렬의 5줄과 End 위치·별도 문단 테두리를 포함했고 모두 통과했다.
같은 독립 PDF 위치 계약을 보정 전/후 WASM tree에 대조하면 기준 밖 글자 위치는 14곳 → 0곳이다.
[보정 전 Top](../assets/pr7200_review/before_valid_top_review_001.png),
[Center](../assets/pr7200_review/before_valid_center_review_001.png),
[Bottom](../assets/pr7200_review/before_valid_bottom_review_001.png)과 아래 최종 증적을 직접 대조했다.

### 최신 Native/fresh WASM 증적

96 DPI의 동일 입력·PDF·페이지로 compare/standalone overlay/review를 다시 생성해 9쪽 모두 직접 판독했다.
Native와 fresh WASM의 페이지 PNG는 **9/9 바이트 동일**하다. 동일 PNG를 두 벌 보관하지 않고
fresh WASM 증적을 대표로 보존한다. 자동 점수는 참고값이며 낮은 점수를 시각 통과로 표현하지 않는다.

| 입력 | pixel / ink match (%) | 보정 후 증적 |
| --- | --- | --- |
| valid-top 1쪽 | 97.90395 / 9.94019 | [compare](../assets/pr7200_review/maintainer_valid_top_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_valid_top_overlay_001.png) · [review](../assets/pr7200_review/maintainer_valid_top_review_001.png) |
| valid-center 1쪽 | 97.91787 / 12.05253 | [compare](../assets/pr7200_review/maintainer_valid_center_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_valid_center_overlay_001.png) · [review](../assets/pr7200_review/maintainer_valid_center_review_001.png) |
| valid-bottom 1쪽 | 97.85378 / 10.88324 | [compare](../assets/pr7200_review/maintainer_valid_bottom_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_valid_bottom_overlay_001.png) · [review](../assets/pr7200_review/maintainer_valid_bottom_review_001.png) |
| empty-top 1쪽 | 97.10339 / 3.81356 | [compare](../assets/pr7200_review/maintainer_empty_top_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_empty_top_overlay_001.png) · [review](../assets/pr7200_review/maintainer_empty_top_review_001.png) |
| empty-center 1쪽 | 97.19062 / 3.97009 | [compare](../assets/pr7200_review/maintainer_empty_center_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_empty_center_overlay_001.png) · [review](../assets/pr7200_review/maintainer_empty_center_review_001.png) |
| empty-bottom 1쪽 | 97.25625 / 4.02623 | [compare](../assets/pr7200_review/maintainer_empty_bottom_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_empty_bottom_overlay_001.png) · [review](../assets/pr7200_review/maintainer_empty_bottom_review_001.png) |
| width-top 1쪽 | 94.42812 / 4.57586 | [compare](../assets/pr7200_review/maintainer_width_top_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_width_top_overlay_001.png) · [review](../assets/pr7200_review/maintainer_width_top_review_001.png) |
| width-center 1쪽 | 94.24401 / 4.20820 | [compare](../assets/pr7200_review/maintainer_width_center_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_width_center_overlay_001.png) · [review](../assets/pr7200_review/maintainer_width_center_review_001.png) |
| width-bottom 1쪽 | 94.15964 / 3.68478 | [compare](../assets/pr7200_review/maintainer_width_bottom_compare_001.png) · [overlay](../assets/pr7200_review/maintainer_width_bottom_overlay_001.png) · [review](../assets/pr7200_review/maintainer_width_bottom_review_001.png) |

추가 실문서 대조는 #2470(1·2쪽)와 #6787(1·2쪽)이다. 보정 Native/fresh WASM의 4쪽은 base와
각각 바이트 동일하다. #2470 2쪽의 기존 사진/행 높이 차이나 #6787의 기존 글꼴 차이를 이번 수정으로
해결했다고 표현하지 않는다.

- #2470: [1쪽 overlay](../assets/pr7200_review/maintainer_issue2470_overlay_001.png), [2쪽 overlay](../assets/pr7200_review/maintainer_issue2470_overlay_002.png)
- #6787: [1쪽 overlay](../assets/pr7200_review/maintainer_issue6787_overlay_001.png), [2쪽 overlay](../assets/pr7200_review/maintainer_issue6787_overlay_002.png)

보정 산출물은 review commit `281795d20` 위 작업 트리의 아래 5개 production 파일 수정으로 빌드했다.
원 PR source `2543382e7`의 빌드라고 표현하지 않는다. 보정 전 WASM은 앞서 보존한 `25e0d8c74`
기반 package다. manifest의 현재 작업공간 HEAD를 이전 package의 source로 혼동하지 않는다.

| 보정 산출물 | SHA256 |
| --- | --- |
| `rhwp` | `383cc512fd9cc8d5726b864f8d5627dbacf668dfe63da09094651951c4299adf` |
| `rhwp.js` | `707049ae519de26779c842eec40f94e016b09bda2688fa721a37d2ae670c2f26` |
| `rhwp_bg.wasm` | `cabe050d9ff6f6eaf603422acec0f203f8c8432143511dc41fe71cef2f27791b` |

### 실행한 검사와 미실행 항목

- `stored_nested_content_flow`: **5/5 PASS**, 기존 3개 + 빈 선행 줄/가로 앵커 + 한컴 PDF 위치/테두리.
- `issue_5787_nested_square_table_horz_offset`: **1/1 PASS**.
- `issue_5711_para_border_negative_spacing`: **2/2 PASS**.
- `issue_6656_line_advance_text_height`: **1/1 PASS**.
- 최종 Native CLI build, fresh WASM release build/wasm-opt, `cargo fmt --all -- --check`: PASS.
- 전체 nextest: 사용자 중단(exit 143), PASS 아님. 이후 전체 회귀 재개 없음.
- 최종 변경의 Clippy 3종 / Native Skia / 통합 head CI: 미실행, push 준비 완료로 표현하지 않는다.

### 남은 보류 사유

1. **원본 합성 HWP/PDF의 시각 불일치**: 비정상 저장 캐시에서 한컴 출력은 글자가 겹친다.
   정상 대조군 통과를 이 원본의 일치로 바꾸어 보고하지 않는다.
2. **TAC 표의 글자 테두리 미재현**: 정상 대조군에도 표 하단 아래 추가 외곽선이 PDF에 남는다.
   [글자 테두리만 끈 통제 입력](../../../tests/fixtures/pr7200_hancom_recomposed/host-char-border-off.hwpx)과
   [같은 엔진 PDF](../../../pdf/pr7200/host-char-border-off-2020.pdf)에서 그 선이 사라지는 것을 확인했다.
   셀/문단 테두리 중복으로 지워도 되는 선이 아니다. 실제 글자처럼 취급 개체의 테두리 영역 계약을
   확인하지 못한 상태이므로 임의의 고정 높이를 더해 맞추지 않았다.
3. 폰트/미세 위치·선 래스터 차이도 남는다. 특히 원본의 저장 메트릭을 한컴이 다시 계산한 차이와
   실제 renderer 차이를 분리한다. 아직 외형 일치·보류 해제·merge 승인으로 판정하지 않는다.

`AGENTS.md`/`CLAUDE.md`에는 최종 소비 위치 역추적, 빈 줄 등 실제 반례의 정식 실행,
렌더링 변경의 Visual Sweep/standalone overlay 직접 판독, 큰 차이가 남으면 전체 회귀보다
원인 수정/재캡처를 먼저 할 것, 대조군 통과를 원본의 통과로 바꾸어 보고하지 않을 것을 보완했다.
