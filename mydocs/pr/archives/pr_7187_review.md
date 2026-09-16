---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7187_review.md
last_verified: 2026-09-16
---

# PR #7187 검토

## 판정

**승인** — HWP3 인라인 탭의 폭·점끌기 저장 범위. 검증한 변경 범위에 대한 로컬 검토 판정이며 전체 이슈 해결 판정과 구분한다.
메인터너 보정 `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d`에서 #7118의 실행 회귀·코드 원칙 위반과 #7184의 끝 컷/경계 증거 부족을 해소했다.
최종 로컬 검토는 **검증한 변경 범위 승인**이다. 통합 코드의 원격 CI는 PR 제출 후 확인할 항목이다.
[2회차 결과·제한 사항](../../working/task_m100_6970_open_pr_stage2.md)과 [최종 검증 결과](../assets/pr7118_7187_review_stage2/gate-results.json)를 기준으로 한다.
CI 재실행·remote push·통합 PR 생성·source PR close는 하지 않았다.

## 검토 대상

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7187](https://github.com/edwardkim/rhwp/pull/7187) |
| 제목 | 수정(parser/hwp3): 인라인 탭이 담은 폭과 점끌기를 HWP5 로 옮긴다 (#7170) |
| 작성자 / reviewer | planet6897 (기존 기여자) / jangster77 사전 요청 |
| 원 head | `75a1a8b9716890225b60e01be001f2d37ba00fb2` |
| source base / 규모 | devel / 3 files, +344/-0, 1 commits |
| 원 PR 상태 | OPEN / draft=False / MERGEABLE / CLEAN |
| source CI | [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/35054782115) 및 전체 rollup 실패/대기 없음; 검토 종료 전 source head 불변 확인 |
| 통합 base | `8d45f242baa1a565357aaa38e9f459595b1e756c` |
| 통합 code head | `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d` (메인터너 보정·검증 증적 포함) |
| branch | `codex/open-pr-review-20260916` |
| 충돌 해결 | 텍스트 충돌 없음. |

원 commit은 `-x`로 누적 적용했고 작성자·메시지·co-author를 보존했다.
source/local SHA와 실행 명령은 [검증 기록](pr_7187_review_impl.md), 공통 제한은 [2회차 보고](../../working/task_m100_6970_open_pr_stage2.md)에 있다.
route: collaborator_external_pr + intake/local_validation/multi_pr_update_branch/visual_fixture_evidence.
#7118의 1,000줄 초과 변경은 별도 코드·실행·시각 검토로 취급했고 admin merge는 하지 않았다.

## 변경·증거 심사

HWP3 인라인 탭의 hunit 폭 ×4와 점끌기 여부를 HWP5의 7-word 확장으로 옮긴다.
같은 문단에 폭/채움이 모두 0인 탭이 있으면 전체 확장을 비워 기존 #1892 순번 계약을 유지한다.
이 fallback은 혼합 문단의 탭 보존을 완성하는 수정이 아니며, 해당 한계를 숨기지 않는다.

focused 3개와 #1892 관련 계약이 통과했다. `hwp3-sample10` 안에서 점끌기 on/off가 함께 존재하는
반례도 검사한다. 실제 264쪽 입력을 저장한 HWP5와 독립 한컴 HWP5에서 탭 확장 **112/112가
폭·fill을 포함해 모든 7 words 동일**하다. 이전 Git 저장본의 112개는 모두 폭 0이었다.
원본/누적 저장본의 한컴 PDF 목차 4–8쪽 점 U+00B7 수는 각각 9,189개이며,
4쪽을 직접 비교하여 줄별 점끌기와 항목 끝 위치를 확인했다.

Native renderer 자체의 탭 채움 읽기 경로는 이 PR의 범위가 아니다. 구조 보존 및 한컴 저장 결과
범위로 수용하며 혼합 zero-width 탭까지 전체 수정했다고 주장하지 않는다.

## 공통 조판 원칙

| 항목 | 판정 | 근거 |
| --- | --- | --- |
| 구현 근거·일반성 | 충족 | 독립 한컴 HWP5 레코드와 위 적용/비적용 조건 |
| 측정·배치 일관성 | 비해당 | 구조/저장 필드 변환이며 컷 높이 변경 없음 |
| 분할·이어받기 | 비해당 | 컷 알고리즘 변경 없음 |
| 줄 소속·점유 높이 | 비해당 | 줄/객체 재조판 규칙 변경 없음 |
| 증거 독립성 | 충족 | 기존 원본·한컴 HWP5·새 저장본 PDF 직접 대조 |
| 기준값 변경 | 비해당 | baseline 완화 없음 |
| 주장·검증 범위 | 충족 | 위 저장 필드 범위 및 미해결 축 구분 |

## 증적

최종 공통 검증: focused 66/66, 전체 nextest 9933/9933(기존 skip 51), Clippy 3종, Native Skia 3종 통과.
새 WASM host `--no-opt` 진단 빌드와 Native의 6문서 22쪽 Visual Sweep을 완료했다.
Docker 배포 최적화 경로는 미실행이며 표준 원격 CI 통과로 대체 표기하지 않는다.
[최종 실행 결과](../assets/pr7118_7187_review_stage2/gate-results.json) · [시각 실행](../assets/pr7118_7187_review_stage2/visual-results.json) · [입력 원장](../assets/pr7118_7187_review_stage2/fixture-manifest.json).

![직접 확인한 비교/브라우저 증적](../assets/pr7118_7187_review/hwp3_saved_hancom_p004.png)

시각 검토 정본: [Visual Sweep](../../manual/verification/visual_sweep_guide.md).
Native/WASM의 PNG 라벨과 본문을 구분해 직접 열었다. HWP3 저장 비교 이미지는 Visual Sweep의
`make_compares`로 **후보를 한컴이 출력한 PDF(왼쪽)**와 **원본 한컴 PDF(오른쪽)**를 합친 것이다.
그 이미지의 기본 `rhwp` 라벨은 Native renderer 출력을 뜻하지 않는다.
1회차(보정 전)의 [입력/PDF 해시 원장](../assets/pr7118_7187_review/fixture-manifest.json), [시각 지표](../assets/pr7118_7187_review/visual-metrics.json),
[focused 이력](../assets/pr7118_7187_review/focused.log)를 함께 보존했다. 낮은 pixel/ink proxy는 완전 일치로 해석하지 않는다.

직접 사용한 기존 HWP/HWPX는 기존 Git 경로를 재사용했다. 1회차의 한컴 PDF와 누적 저장 HWP는 `788ab292b`에 이미 포함돼 있다.
2회차는 기존 Git 입력·PDF를 재사용하고 새 비교 이미지와 로그를 보존했다. 아직 merge하지 않았으므로 source PR/issue는 닫지 않는다.
