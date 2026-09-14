---
kind: snapshot
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7109 저장 세로 위치가 없는 여러 줄 셀 검토

**판정: 머지 보류 — 독립 한컴 시각 증거 미확보.** 로컬 체리픽 통합본에 대한 검토다. 원 PR merge/close 또는 GitHub approve를 완료했다는 의미가 아니다.

## 출처와 통합 범위

| 항목 | 값 |
| --- | --- |
| 원 PR / 작성자 | [#7109](https://github.com/edwardkim/rhwp/pull/7109) / LJYeon12 |
| 원 head | `191fa0046337dc8bde7d4ed396960f625ee370e7` |
| 최신 devel 시작점 | `410d22cdf77e3e9e45159999cab19719c621c025` |
| 통합 code head | `b40c2953cf526b86319f2841c5b5584d7a83d735` |
| 작업 branch | `codex/non-draft-integration-20260914` |
| reviewer | jangster77, 검토 전 요청 등록 확인 |
| source → local | `30c9314e3` → `a161bd646`; `1ff3c0388` → `a4054879d`; `191fa0046` → `09836e4f7` |

사용자 지시로 #7141·#7142를 제외했다. draft #6670·#7118도 제외했다. 현재 대상은 #7099·#7109·#7134·#7136·#7137·#7139의 6개 PR, 원 commit 9개다. 원 작성자와 cherry-pick -x 출처를 보존했다.

## 코드와 증거 판독

`cell_vpos_ladder_is_intact`가 서로 다른 연속 두 줄의 vpos=0을 유효한 저장 사다리에서 제외한다. 세로 정렬과 표 축소 하한이 같은 helper를 사용하므로 실제 배치의 두 줄 높이와 측정 결과를 맞춘다. 같은 text_start 중복, 양수→0 리셋, 페이지·단 첫 줄, 기존 가로 조각 판정을 보존한다. 특정 문서 ID나 임의 보정 상수는 없다.

원본 CI의 신규 테스트는 위/가운데/아래 정렬·두 셀 높이·HWP roundtrip·여유 행 축소·중복/리셋 경계를 다룬다. 이 검토에서 해당 회귀를 다시 실행한 것은 아니다.

공개 입력 `tests/fixtures/multiline_cell_zero_positions/two_lines.hwpx`는 rhwp 합성 문서다. 1000 HU 두 줄 + 위아래 140 HU로 2280 HU가 필요하다는 계약은 입력 치수에서 독립적으로 설명된다. 이것을 한컴 생성 문서라고 부르지 않는다. contributor의 비공개 실제 문서 2~4쪽 개선 수치와 이미지는 검토자가 접근하지 못해 재검증하지 않았다.

한컴 MCP engine 2020 변환을 요청했고 상태를 중간 조회했다. 최종 상태와 브라우저 결과는 아래 후속 절에 기록한다. 서버가 처리 중이라는 사실을 PDF 생성 성공으로 바꾸어 기록하지 않는다.

## CI와 검증 범위

조회한 원 PR head는 위 source SHA와 일치하고, CI에 실패·대기 항목이 없다. skipped/neutral을 실제 검사 통과 수로 합산하지 않는다. [Lint (fmt, clippy, WASM check): SUCCESS](https://github.com/edwardkim/rhwp/actions/runs/34767492226/job/103841709976) · [Frontend package gates: SUCCESS](https://github.com/edwardkim/rhwp/actions/runs/34767492226/job/103841709975) · [Build & Test: SUCCESS](https://github.com/edwardkim/rhwp/actions/runs/34767492226/job/103844440022).

이 통합 head에는 별도 GitHub CI가 아직 없다. 원 PR들의 녹색 상태를 새 통합 head의 CI 성공으로 대신하지 않는다. 사용자 지시에 따라 완료된 CI 전체 회귀를 로컬에서 반복하지 않았다. macOS 전용 `CARGO_TARGET_DIR=target/nondraft-review-20260914`의 native build를 새 head에서 완료했고, fresh WASM web 빌드도 성공했다(host `--no-opt`, 5분 12초). Docker/wasm-opt 표준 빌드 및 성능 검증 성공으로 확대하지 않는다. [WASM 빌드 로그](../assets/non_draft_20260914_wasm-build-six-pr.log.txt). push/최종 PR 전 필수 lint와 통합 CI 게이트는 남아 있다.

## 공통 조판 원칙 준수

| 항목 | 판정·범위 |
| --- | --- |
| 독립 근거·원인 계층 | 위 코드/입력 계약에 근거. contributor 주장과 직접 관찰을 구분 |
| 문서별 예외·좌표 clamp | 이번 수용 코드에 문서 ID 분기나 픽셀 맞춤 상수 추가 없음 |
| 측정·배치 공통 결과 | #7109는 공통 사다리 helper. #7099는 devel 유지. 나머지는 레이아웃 산식 변경 비해당 |
| 줄 소속·높이 | 변경 범위와 반례는 본문 참조. 모든 중첩 표·모든 문서 보장으로 확대하지 않음 |
| golden·허용치 | 기존 baseline을 완화하지 않음. #7099 원 golden 변경은 수용하지 않음 |
| 실제 시각 증거 | 실행한 Visual Sweep/Studio만 기록. 미공개 문서/미실행 호스트는 미검증 |
| 성능·전체 fidelity | 이번 검토에서 측정·보증하지 않음 |

입력 SHA-256·Git blob 일치·source CI·원 commit 매핑은 [공통 증적](../assets/non_draft_20260914_evidence.json)에 있다. 이미 Git에 있는 HWP/HWPX/PDF를 이름만 바꿔 중복 추가하지 않았다.

## 최종 통합 이후 comment 계획

통합 PR이 승인·CI 완료·merge된 뒤 원 PR에 구현 출처, 실제 수용 범위, 보정 사유, 검증 결과와 미검증 범위를 설명한다. Visual Sweep가 적용된 PR에는 [사용법](../../manual/verification/visual_sweep_guide.md)과 확정 merge SHA의 증적 링크를 포함한다. 원 PR close는 구현 반영을 확인한 뒤 진행하고 contributor fork branch는 보존한다. 현재 원격 comment/close/통합 PR 생성은 하지 않았다.

## 최종 실측과 보류 사유

Visual Sweep의 WASM exporter와 공통 webfont rasterizer로 동일 합성 입력의 전후를 직접 캡처했다. [이전](../assets/pr7109_two_lines_before.png)에서는 두 번째 줄 아래가 셀 clip에 잘리고, [통합](../assets/pr7109_two_lines_wasm.png)에서는 두 줄이 보인다. 96 DPI tree의 셀은 y=69.3, h=30.4로 동일하다. 두 줄 y가 77.9/91.2 → 71.2/84.5px로 바뀌어 두 번째 줄 하단이 104.5 → 97.8px, 셀 아래 경계 99.7px 안으로 들어왔다. 값은 exporter 소수점 1자리 반올림이다. 이는 합성 입력의 결함 해소 근거다.

한컴 MCP job `a5a3a97b-6f43-404b-b105-4a1dbff3b3f5`는 engine 2020, timeout_seconds 900으로 실행했다. status에서 converting 진행을 여러 차례 확인했고, 최종 `failed` / `Hancom 2020 direct conversion exceeded 900 seconds.` / output_bytes=0을 확인했다. [최종 상태](../assets/pr7109_hancom_conversion.json). PDF는 생성되지 않아 빈 파일이나 대체 PDF를 커밋하지 않았다. 입력 생성기 문제인지 변환 worker 문제인지는 이 timeout만으로 판정할 수 없다.

**보류 조건:** 재현 가능한 유효 한컴 문서/독립 PDF로 같은 여러 줄 정렬·축소 조건을 확인해야 한다. 현재 공개 합성 계약과 기여자의 비공개 문서 주장만으로 한컴 호환 시각 검증을 완료 처리할 수 없다. 코드의 신규 회귀를 발견했다는 판정과 필수 증거 부족을 구분한다. 원 PR은 통합 branch에 보존되어 있으나 이 조건 해소 전 merge하지 않는다.
