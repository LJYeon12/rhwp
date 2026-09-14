---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7119 — Canvas 보충 진행폭 self-review 준비 기록

현재 판정: **머지 보류**. 원격 CI와 정식 self-review는 아직 완료하지 않았다.
이는 새 결함 검출이 아니라 제출 직후의 절차 상태다. 기존 로컬 검증과 메인테이너 시각 통과는 유지한다.

## 대상과 승인 경계

- [PR #7119](https://github.com/edwardkim/rhwp/pull/7119), [issue #7084](https://github.com/edwardkim/rhwp/issues/7084).
- 작성자/assignee `edwardkim`, milestone `v1.0.0`, labels `bug`, `rhwp-studio`, `rendering`, `font`.
- `task_m100_7084` → `devel`, Open(non-Draft), reviewer 지정 없음.
- 최초 게시 head `78f372a5b020ab3d7f23c79c1a7ba409f7c75d49`.
- 검증 source/test/fixture `d3a189d5d964e4e6eb3b376062df846744c4c58d`.
- 재조회한 base `11860a9f4186438a3cb9c355e3cd8d2257162d3a`; 최초 merge-tree exit 0.
- 기본 경로 `collaborator_self_merge`; modifier `intake_and_review`, `local_validation`,
  `visual_fixture_evidence`, `rework_and_exceptions`, `review_only_fast_pass`를 읽고 적용했다.
- 이번 승인은 작업 branch push·Open PR 생성·게시 준비 기록이다. GitHub approve/comment,
  merge·issue close는 수행하지 않는다. [후속 검토 순서](pr_7119_review_impl.md)를 따른다.

## 완료된 검증

[Stage 7](../../working/task_m100_7084_stage7.md) 6절과
[결과보고서](../../report/task_m100_7084_report.md)의 동일 후보 검증을 재사용한다.
9,814 nextest PASS/51 skipped/0 FAIL, 별도 lib 4,055 PASS/13 ignored,
집중 84 PASS, Native Skia 4,112 PASS/13 ignored 및 그림 2·PDF 4 PASS,
세 Clippy·workspace build·fmt·doctest·manifest·unit-tier가 완료됐다.
이 수치는 중복 포함되므로 합산하지 않는다.
Docker WASM·TypeScript·production build, Studio 1,692 PASS/2 skipped,
실제 Chrome HWP/HWPX·서식·portable 복원·결합열 경계·오류 복구 12조건,
renderer/Undo 계약, Render Diff 3/3·직접 PDF gate 3/3이 완료됐다.
원격 CI 성공을 이 로컬 결과로 대신하지 않는다.

제출 시 검증 후보 뒤 변경은 결과 문서 2개뿐이었다. diff 공백 검사와 변경 Markdown 링크 검사 통과.
원본 HWP/HWPX 두 개와 기준 PDF 두 개 모두 실제 검증 bytes가 위 commit의 Git blob과 일치했다.
새 HWP/HWPX sample 추가·변경은 없으므로 신규 sample 보안 검사 입력 대상은 없다.
PDF는 기존 MCP 산출물을 재사용했으며 새로 변환하지 않았다. 상세 출처·SHA-256은 Stage 7 5절을 따른다.

## 조판 원칙 — 정식 self-review 확인 목록

아래는 제출 시 확인 범위이며, 구현 이름만으로 정식 검토 완료를 선언하지 않는다.

| 항목 | 현재 근거와 남은 확인 | 판정 |
| --- | --- | --- |
| 구현 근거·일반성 | 세션 보충 폭과 적용 제외 계약이 존재한다. 정식 검토에서 전체 호출 경로와 임의 조건 유무를 확인한다. | 미검증 |
| 측정·배치 일관성 | `text_measurement.rs`의 snapshot 조회와 `web_canvas.rs` descriptor 소비가 연결된다. 전체 수명·실패 경로 검토가 남았다. | 미검증 |
| 줄 구성·점유 높이 | 객체 줄 소속/표 높이 규칙 변경은 범위 밖. 텍스트 진행폭 전달이 줄바꿈·caret와 일치하는지 정식 검토한다. | 미검증 |
| 독립 사례·시각 근거 | 기존 실제 HWP/HWPX·한컴 PDF·메인테이너 시각 통과, 실제 Chrome 서식 및 결합열 검사. 합성 계약을 한컴 출력으로 간주하지 않는다. | 충족 |
| baseline 변경 | 쪽수 원장에 독립 PDF를 근거로 두 입력의 2/2 행만 추가. 기존 기대값·허용치 변경 없음. | 충족 |
| 주장과 검증 범위 | 동일 검증 SHA·명령·실행 결과와 미지원 경로를 Stage 7에 구분했다. CI와 정식 코드 검토는 미완료로 유지한다. | 충족 |

## 직접 시각 증적

[표준 비교 가이드](../../manual/verification/visual_sweep_guide.md#github-merge-comment)의
compare/overlay/review 패널을 사용했다. 실행 방법은 Stage 7 7절을 따른다.
HWP/HWPX 각각 2쪽 한 장씩, 총 2페이지를 비교했다. 구조 자동 후보 검사는 실행하지 않았으며
후보 0건이라고 주장하지 않는다. 각각 pixel match 90.78317%, 내용 픽셀 보조값 6.68869%다.
메인테이너는 두 형식의 이모지 렌더링을 통과로 판정했다. 통합 화면은 이전 승인 유지 화면과
byte-identical이며 활성 paint 배율 1/1, 비활성 대조 약 0.364245/1이다.
이 수치는 전체 조판 fidelity 또는 이모지 정확도를 뜻하지 않는다. PDF와의 색상/글꼴·표 간격 차이는 남는다.

- 임시 패널: `output/7084/stage7/visual/{hwp,hwpx}/{compare,overlay,review}/`의 `*_002.png`.
- 영구 대표: [HWP 2쪽 비교](../assets/pr_7119_7084_hwp_review_002.png).
- 대표 PNG를 직접 열어 본문·도구 라벨·한글·지표·footer가 판독 가능함을 확인했다.
  두 형식의 Canvas 본문이 같으므로 대표 하나만 보존한다. 중간 로그·JSON·PNG는 커밋하지 않는다.
- PDF SHA-1: HWP `b143b6154f036b37903801168ef56b6eba0c21f5`, HWPX `420f1785b2e70e7a2ce91a87b9e3f3160c60f8eb`.
  두 PDF 모두 1.6, 2쪽, 595×841pt, Creator/Producer `Hancom PDF 1.3.0.550`.
  MCP engine/profile 2020, 실제 응답 Hancom 12.0.0.4605를 구분한다.

## Merge 후 contributor PR comment 계획

현재 게시하지 않는다. 정식 self-review·최신 CI·병합 승인 후 실제 merge와 asset의 devel 포함을 확인한 뒤,
별도 승인 범위에서 UTF-8 LF 본문 파일을 `gh pr comment --body-file`로 게시하고 API로 재조회한다.
문서 비교 정본 링크, 위 2페이지/자동 후보 미실행/지표, 사람의 이모지 통과와 잔여 차이를 함께 쓴다.
대표 이미지 URL은 실제 merge SHA로 다음 형식을 채운다.

`https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr_7119_7084_hwp_review_002.png`

이 PR 생성 또는 로컬 검증만으로 #7084를 close하지 않는다.
