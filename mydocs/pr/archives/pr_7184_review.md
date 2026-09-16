---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7184_review.md
last_verified: 2026-09-16
---

# PR #7184 검토

## 판정

**메인터너 보정 후 변경 범위 승인** — 중첩 표 행 높이와 컷 예산 보완 범위. 검증한 변경 범위에 대한 로컬 검토 판정이며 전체 이슈 해결 판정과 구분한다.
메인터너 보정 `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d`에서 #7118의 실행 회귀·코드 원칙 위반과 #7184의 끝 컷/경계 증거 부족을 해소했다.
최종 로컬 검토는 **검증한 변경 범위 승인**이다. 통합 코드의 원격 CI는 PR 제출 후 확인할 항목이다.
[2회차 결과·제한 사항](../../working/task_m100_6970_open_pr_stage2.md)과 [최종 검증 결과](../assets/pr7118_7187_review_stage2/gate-results.json)를 기준으로 한다.
CI 재실행·remote push·통합 PR 생성·source PR close는 하지 않았다.

## 검토 대상

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7184](https://github.com/edwardkim/rhwp/pull/7184) |
| 제목 | 수정(layout): 중첩 표 행 유닛과 조각 예산을 페인트와 같은 값으로 센다 (#7140) |
| 작성자 / reviewer | planet6897 (기존 기여자) / jangster77 사전 요청 |
| 원 head | `52fbf006a83b5a46de152aa89b6303280eb23da5` |
| source base / 규모 | devel / 8 files, +174/-52, 2 commits |
| 원 PR 상태 | OPEN / draft=False / MERGEABLE / CLEAN |
| source CI | [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/35045223986) 및 전체 rollup 실패/대기 없음; 검토 종료 전 source head 불변 확인 |
| 통합 base | `8d45f242baa1a565357aaa38e9f459595b1e756c` |
| 통합 code head | `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d` (메인터너 보정·검증 증적 포함) |
| branch | `codex/open-pr-review-20260916` |
| 충돌 해결 | 텍스트 충돌 없음. |

원 commit은 `-x`로 누적 적용했고 작성자·메시지·co-author를 보존했다.
source/local SHA와 실행 명령은 [검증 기록](pr_7184_review_impl.md), 공통 제한은 [2회차 보고](../../working/task_m100_6970_open_pr_stage2.md)에 있다.
route: collaborator_external_pr + intake/local_validation/multi_pr_update_branch/visual_fixture_evidence.
#7118의 1,000줄 초과 변경은 별도 코드·실행·시각 검토로 취급했고 admin merge는 하지 않았다.

## 변경·증거 심사

판정 대상은 원본 `52fbf006`에 메인터너 보정 `f2e21164c`를 포함한 통합 코드다.
`cell_units`의 다중 행 중첩 표가 실제 paint의 `resolve_row_heights`를 사용하는 원 변경을 유지했다.
1회차 보류였던 가상 끝 컷과 경계 증거를 아래와 같이 보정했다.

| 기존 보류 사유 | 메인터너 보정 | 최종 증거 |
| --- | --- | --- |
| 예약 helper가 `units.len()-1`을 가상 끝으로 사용 | 실제 선택한 end cut을 전달. 마지막 유닛도 종결 viewport 예약 계산에 포함 | 보정 전 기대 24px 대신 0px로 RED, 보정 후 GREEN |
| 예약 뒤 실제 컷·예산 소비 불명확 | `advance_row_cut_with_mixed_nested_reserve`가 선택 컷을 측정하고 부족하면 예산을 줄여 재선택 | 원래 유닛만 fit하는 예산, 시작/끝 컷, 1×1/다열, legacy/native 재귀 경계 |
| 마지막 유닛·후속 내용 증거 부족 | 기존 multi-cell 테스트에 선택 구간의 유닛 소유와 실제 `row_cut_content_height`, 소비 종료를 연결 | 유닛 2·3·4·5를 중복·누락 없이 소비, 종료 높이/예약 0, 42065 15–17쪽 마지막 내용 유지 |

일반 컷과 재시도 모두 실제 컷으로 예약한다. 그 이후의 저장 `source_frame_tail` 확장은
별도의 저장 프레임 소유 계약이며, 최종 `row_cut_content_height`와 `consumed` 누적은 유지된다.
분할할 수 없는 첫 유닛을 소비하는 기존 진행 규칙도 유지한다. 모든 조각이 임의의 작은 예산에
들어간다고 주장하지 않는다. [호출 경로·합성 기대값의 범위](../../working/task_m100_6970_open_pr_stage2.md#실제-호출-경로),
[RED/GREEN](../assets/pr7118_7187_review_stage2/red-green-results.json), [focused 결과](../assets/pr7118_7187_review_stage2/focused-stage2-results.json)를 대조했다.

최종 focused 66/66, 전체 회귀 9933/9933, Native Skia 3종과 fresh WASM 비교를 완료했다.
거대 HWPX는 48쪽이며 18–21·47–48쪽에서 본문 하단과 최종 내용을 직접 확인했다.
이 6쪽과 #3637의 26–30쪽은 보정 전 Native 출력과 픽셀 동일하다. Native/WASM도 동일한 본문을 보인다.

**해소 범위:** 실제 끝 컷/종결 예약 결함과 관련 경계 증거 부족을 해소했다. 19쪽에서 PDF는
표 전체, rhwp는 마지막 행만 남는 기존 차이와 #3637 29쪽의 기존 표 내용 누락은 남는다.
이를 #7140·#3637 전체 해결로 표시하지 않는다. 합성 24px 검사는 기존 viewport 계약의 전달을
검사하며 그 숫자 자체가 새 한컴 실측값은 아니다. 원본 42065 종결 내용은 별도 직접 비교했다.

통합 회귀의 그림 프레임 실패는 #7118에 귀속됐으며 이번 보정에서 함께 해소했다.

## 공통 조판 원칙

| 항목 | 판정 | 근거 |
| --- | --- | --- |
| 구현 근거·일반성 | 충족 | 선언 행 높이/저장 등식과 실제 PDF, 가상 끝 컷 제거 |
| 측정·배치 일관성 | 충족(검증 범위) | 실제 컷 예약·재선택·후속 높이 누적 경로 연결 |
| 분할·이어받기 | 충족(검증 범위) | 시작/끝/종결 컷, 예산 부족, native/legacy와 후속 내용 |
| 줄 소속·점유 높이 | 충족(검증 범위) | 논리 높이+예약과 같은 cut의 실제 행 높이 검사 |
| 증거 독립성 | 충족 | 기존 viewport 계약과 RED/GREEN, 원본 한컴 PDF·Native/WASM |
| 기준값 변경 | 충족 | 원 PR 47→48의 PDF 근거 유지, 보정에서 추가 완화 없음 |
| 주장·검증 범위 | 충족 | 합성 계약·실물 관측·기존 PDF 차이를 구분 |

## 증적

최종 공통 검증: focused 66/66, 전체 nextest 9933/9933(기존 skip 51), Clippy 3종, Native Skia 3종 통과.
새 WASM host `--no-opt` 진단 빌드와 Native의 6문서 22쪽 Visual Sweep을 완료했다.
Docker 배포 최적화 경로는 미실행이며 표준 원격 CI 통과로 대체 표기하지 않는다.
[최종 실행 결과](../assets/pr7118_7187_review_stage2/gate-results.json) · [시각 실행](../assets/pr7118_7187_review_stage2/visual-results.json) · [입력 원장](../assets/pr7118_7187_review_stage2/fixture-manifest.json).

![직접 확인한 비교/브라우저 증적](../assets/pr7118_7187_review_stage2/visual/wasm/pr7184/compare_019.png)

시각 검토 정본: [Visual Sweep](../../manual/verification/visual_sweep_guide.md).
Native/WASM의 PNG 라벨과 본문을 구분해 직접 열었다. HWP3 저장 비교 이미지는 Visual Sweep의
`make_compares`로 **후보를 한컴이 출력한 PDF(왼쪽)**와 **원본 한컴 PDF(오른쪽)**를 합친 것이다.
그 이미지의 기본 `rhwp` 라벨은 Native renderer 출력을 뜻하지 않는다.
1회차(보정 전)의 [입력/PDF 해시 원장](../assets/pr7118_7187_review/fixture-manifest.json), [시각 지표](../assets/pr7118_7187_review/visual-metrics.json),
[focused 이력](../assets/pr7118_7187_review/focused.log)를 함께 보존했다. 낮은 pixel/ink proxy는 완전 일치로 해석하지 않는다.

직접 사용한 기존 HWP/HWPX는 기존 Git 경로를 재사용했다. 1회차의 한컴 PDF와 누적 저장 HWP는 `788ab292b`에 이미 포함돼 있다.
2회차는 기존 Git 입력·PDF를 재사용하고 새 비교 이미지와 로그를 보존했다. 아직 merge하지 않았으므로 source PR/issue는 닫지 않는다.
