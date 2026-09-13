---
kind: investigation
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-13
---

# PR #7103 메인터너 보정·체리픽 통합 계획

[원 PR 검토](pr_7103_review.md)의 F1/F2를 제한된 조판 범위에서 보정하는 후속 경로다.
작업지시자의 요청은 처리 가능 여부 확인이며, 이 문서는 보정 완료나 remote 변경 승인으로 세지 않는다.

## 확인 완료

- source `1c5fd967674e238b3d4e42948c1abe5d4c279f6f`는 `d92086eec6`으로 이미 cherry-pick했다.
  원 저자와 `-x` provenance를 유지했다. 중복 cherry-pick은 하지 않는다.
- 현재 `review/pr7103-20260913`은 최신 `upstream/devel=cd2d9e8a4`를 포함한다.
- 원본 저장소에 `jangster77`의 `push: true`가 있어 upstream 임시 branch의 통합 PR 경로를 쓸 수 있다.
  devel 직접 push는 사용하지 않는다.
- fork는 일반 `push: false`지만 PR의 `maintainer_can_modify: true`이며, 비LFS 빈 commit의
  `GIT_LFS_SKIP_PUSH=1 git push --dry-run`은 통과했다. 초기 LFS lock 오류만으로 쓰기 불가라고
  판단했던 기록을 정정했다. 실제 push는 없었다. 이 사실이 체리픽 통합 경로를 막지는 않는다.

## 보정 범위

1. `Paragraph::control_text_positions`의 **편집 논리 위치**를 원시 UTF-16 위치로 오인하는
   빈 carrier 소속 계산을 바로잡는다. 편집 API의 논리 위치 의미 자체를 전역 변경하지 않는다.
   원시 control 위치/저장 LineSeg 대응을 공통 결과로 마련하고 HWPX axis shift·재조판 줄의 적용
   범위를 구분한다. 이번 HWP의 24/40은 독립 원시 레코드에서 확인한 값이지 샘플 상수가 아니다.
2. 연속 표 배치에서 `seg_idx + 1` 대신 실제 다음 표의 소유 줄과 그 줄의 점유·advance를 사용한다.
   현재 `forward_line_seg_gap_hu`의 무조건 음수 clamp를 원인 해결로 유지하지 않는다.
   표 테두리 높이와 바깥여백 포함 줄 높이, 기준선 및 마지막 표 이후 문단 advance를 함께 검토한다.
3. 기존 소속 함수는 layout 외에 composer의 `owned_rowbreak_tac_height`, pagination의 routing,
   table cell 배치에서도 사용한다. 이 호출자들이 같은 소속 결과를 소비하는지 확인한다.
   `inline_flow::supports`는 현재 Header/Footer가 있는 본 입력을 지원하지 않으므로 조건만 넓혀
   강제로 재조판시키는 대안으로 처리하지 않는다. 저장 줄 소속 교정과 재조판 경로를 구분한다.
4. #7103 integration test의 “안 겹침 + 2px 이하”만으로 정답을 판정하지 않는다. 소유 줄 대응과
   한컴 PDF의 실제 경계·후속 표 위치를 검증한다. glyph 형태 차이와 기하 잔여 차이는 별도로 다룬다.

전체 엔진 재작성이나 원본 fixture의 좌표 변경은 현재 보정의 필수 조건이 아니다. 다만 위 내용은
코드 경로에서 확인한 보정 범위이며, 아직 패치를 구현·실행해 성공을 입증한 상태는 아니다.

## 보정 후 검증과 기록

- 비가시 제어 사이 TAC, 정상 양수 간격, 합법적인 음수 줄간격, 같은 줄/다른 줄 표 및
  저장 LineSeg/재조판 경로의 적용·비적용 계약을 실제 결과로 확인한다.
- #7103/#6078/#6181 focused를 재실행하고, 공유 소속 함수의 영향에 따라 RowBreak·cell·pagination
  회귀를 추가 선택한다. 기존 focused·OVR·source CI는 보정 전 증거로 유지한다.
- 필수 Rust lint 묶음(native/WASM/workspace Clippy 포함), integration suite 및 해당 Native/WASM
  검증을 local_validation 절차대로 보정된 head에서 완료한다.
- 기존 커밋 입력과 기준 PDF를 재사용하여 OVR5 및 p.1 fidelity/Visual Sweep/3-way/OVL를 갱신한다.
  이미 Git에 있는 HWP/HWPX/PDF를 이름 변경해 추가하지 않는다. 허용치·golden으로 차이를 가리지 않는다.
- 보정 code/test를 별도 commit으로 보존하고 원 head/보정 SHA/통합 head/증거를 원 PR review에 기록한다.
  모든 보류 사유 해소 후에만 판정을 `메인터너 보정 후 수용 가능`으로 변경한다.

## 통합 PR 및 후속 처리 경로

보정 검증 후 작업지시자가 승인한 원격 작업 범위에서 진행한다.

1. 원본 저장소 임시 branch(예: `fix/7096-tac-line-ownership-20260913`)를 사용해 devel 대상
   통합 PR을 만든다. owner를 자동 reviewer로 지정하지 않는다.
2. 원 PR의 `closes #7103` 오류를 통합 PR에 복사하지 않는다. #7096의 실제 해결 범위에 맞게
   closing 관계를 정하고 원 contributor PR/source SHA 및 메인터너 보정 내용을 구분한다.
3. code candidate의 정확한 head CI를 확인한다. 검토·오늘할일·증적 최종 기록은 같은 통합 PR의
   trailing commit 경로로 처리하며 별도 통합 번호용 review 문서를 새로 만들지 않는다.
4. 최종 head CI와 mergeability 확인 후 승인 범위에 따라 통합 PR을 merge한다.
   반영된 통합 PR/merge SHA를 원 #7103에 코멘트한 뒤 close하고 #7096 상태를 확인한다.
5. post_merge 절차에 따라 devel 동기화·적용되는 duration refresh·소유 산출물 정리를 완료한다.
   원 contributor fork branch는 삭제하지 않는다.
