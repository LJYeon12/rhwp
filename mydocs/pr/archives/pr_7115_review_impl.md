---
kind: investigation
status: active
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-14
---

# PR #7115 체리픽·검증·후속 계획

[개별 review](pr_7115_review.md)의 판정은 **머지 보류**다. 통합 번호를 미리 만들지 않았으며
원 저자의 commit·SHA를 `-x`로 보존했다. 이 계획은 local 검토 완료와 아직 하지 않은 원격 조치를 구분한다.

## 이미 적용한 commit

| 원 SHA | 로컬 SHA | 제목 |
| --- | --- | --- |
| `fa94b52a07de08af9d4320ab911caefa19e323dd` | `8a90acbabcfa3a1fe503b5a7a9b9dc8e88b74e65` | 수정: ㆍ(U+318D) 를 반각으로 박던 폴백을 전각으로 되돌린다 (#7080) |

기준 `93ffc3dd59c120bd54df4c2ac6d1ddbe630f8a2d` 위에서 PR 순서 7094 → 7100 → 7104 → 7111 → 7112 → 7113 → 7115 →
7116 → 7117 → 7120 → 7131 → 7132로 누적했다. 이 PR의 마지막 local SHA는 `8a90acbabcfa3a1fe503b5a7a9b9dc8e88b74e65`다.
추가 fixture/PDF 보존은 `6933852a11b7e5998429eeb15708fdaeed7db626`, 최신 upstream 정렬 후 기록 기준은 `28d702d8f84bacb7ecec4f2a6dafd049e09bfcfa`다.

## 보정과 규칙

U+318D의 fallback 전진폭은 독립 글꼴·한컴 출력에 따라 정해야 하며, 올바른 glyph 폭을 적용한 뒤에도 유효 저장 줄 소속과 페이지 경계를 보존해야 한다. 글자폭을 다시 줄여 페이지 차이를 숨기는 보정은 수용하지 않는다.

F1 [P1, 실행 시각 회귀]: 80168 108쪽 오른쪽 셀의 `9. 그 밖에 시ㆍ도조례로 정하는 사항`이 두 줄이 되면서 이후 내용이 33.6px 아래로 밀린다. 표 bbox는 전후 x=75.6,y=107.6,w=640.4,h=910.0으로 동일하다. 한컴/변경 전의 108쪽 끝 `3. 그 밖에 시ㆍ도조례로 정하는 사항`이 candidate 109쪽 맨 위로 이동한다. 유효 저장 줄의 측정·배치/재조판 경계를 점검하고 해당 페이지 소속을 독립 PDF 회귀로 고정해야 한다. 76→77쪽 기존 차이도 커져 함께 검토 대상이다.

#7111의 text_measurement 충돌에서는 최신 supplemental_metrics와 신규 font_metric_trusted를 함께 남겼다.
이외 생산 코드에 메인터너 수정은 추가하지 않았다. #7115 원인 확인용 임시 함수 rollback은 별도
대조 바이너리에만 사용하고 검토 source를 바이트 단위로 복구했다. 정식 보정으로 세지 않는다.

## 단계와 실행 상태

| 단계 | 상태 / 다음 작업 |
| --- | --- |
| source inventory·최신 devel·reviewer | 완료; draft #7098 제외, reviewer jangster77; owner 자동 요청 없음 |
| 16 commit 체리픽·충돌 보정 | 완료; 원 SHA·저자·적용 순서 위에 보존 |
| 원본·한컴 PDF 확보 | 완료; Git 내 동일 SHA 재사용, 신규 자료만 별도 보존 commit |
| focused/full/Native/Clippy/WASM/실물 sweep | 완료; 정확한 결과·제약은 review 참조 |
| 발견 결함 처리 | #7104·#7113·#7115 보류. 원인 보정 또는 범위 분리 후 필요한 검사 재실행 |
| 통합 PR | 아직 생성하지 않음. 준비가 확정되면 upstream 임시 head → devel; reviewer owner 자동 지정 안 함 |
| CI와 trailing 기록 | 새 code candidate CI 통과 후 review·오늘할일을 동일 PR에 포함; source/fixture를 docs-only trailing에 섞지 않음 |
| merge·원 PR/issue 후속 처리 | 최종 SHA·CI·MERGEABLE/CLEAN·승인 확인 후 수행. 원 PR comment에 통합 merge와 반영 SHA, 미해결 issue 범위를 남김 |
| devel·cleanup | 실제 merge 후 devel 동기화. 실행 중 Cargo/Rust가 없는지 확인하고 전용 target만 post_merge 7.7.1에 따라 정리 |

사용자 선택이 필요한 다음 범위는 발견 결함을 보정해 12개를 함께 진행할지, 보류 PR을 제외한 별도
수용 묶음을 만들지다. 지금 reviewed history에서 임의로 source PR을 빼거나 원격에 게시하지 않았다.

## rollback 경계

현재 branch는 `review/planet6897-20260914`다. 작업공간은 공유하므로 reset/clean으로 다른 변경을 버리지 않는다.
범위 분리가 승인되면 당시 최신 `upstream/devel`에서 새 branch를 만들고 이 표의 승인된 원 SHA와 필요한
메인터너 보정만 순서대로 적용한다. 특히 #7111·#7112·#7115는 같은 측정 코드, #7115·#7117은
form-002 golden을 공유하므로 중간 commit만 취소한 상태를 검증 결과로 재사용하지 않는다.

[후속 처리 정본](../../manual/pr_review/post_merge.md)을 따른다.
