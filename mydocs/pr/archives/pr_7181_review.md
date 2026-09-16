---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7181_review.md
last_verified: 2026-09-16
---

# PR #7181 검토

## 판정

**승인** — HWP3 각주 구분선 위·아래 여백의 HWP5 슬롯 연결 범위. 검증한 변경 범위에 대한 로컬 검토 판정이며 전체 이슈 해결 판정과 구분한다.
메인터너 보정 `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d`에서 #7118의 실행 회귀·코드 원칙 위반과 #7184의 끝 컷/경계 증거 부족을 해소했다.
최종 로컬 검토는 **검증한 변경 범위 승인**이다. 통합 코드의 원격 CI는 PR 제출 후 확인할 항목이다.
[2회차 결과·제한 사항](../../working/task_m100_6970_open_pr_stage2.md)과 [최종 검증 결과](../assets/pr7118_7187_review_stage2/gate-results.json)를 기준으로 한다.
CI 재실행·remote push·통합 PR 생성·source PR close는 하지 않았다.

## 검토 대상

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7181](https://github.com/edwardkim/rhwp/pull/7181) |
| 제목 | 수정(parser/hwp3): 각주 구분선 여백을 HWP5 슬롯에 배선한다 (#7174) |
| 작성자 / reviewer | planet6897 (기존 기여자) / jangster77 사전 요청 |
| 원 head | `d4070e7ae3710c1eb047029c7751ad9d71562e7c` |
| source base / 규모 | devel / 6 files, +121/-0, 1 commits |
| 원 PR 상태 | OPEN / draft=False / MERGEABLE / CLEAN |
| source CI | [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/35029519148) 및 전체 rollup 실패/대기 없음; 검토 종료 전 source head 불변 확인 |
| 통합 base | `8d45f242baa1a565357aaa38e9f459595b1e756c` |
| 통합 code head | `f2e21164c029320d1e9b42a8c8ad1751c69fcb1d` (메인터너 보정·검증 증적 포함) |
| branch | `codex/open-pr-review-20260916` |
| 충돌 해결 | 텍스트 충돌 없음. |

원 commit은 `-x`로 누적 적용했고 작성자·메시지·co-author를 보존했다.
source/local SHA와 실행 명령은 [검증 기록](pr_7181_review_impl.md), 공통 제한은 [2회차 보고](../../working/task_m100_6970_open_pr_stage2.md)에 있다.
route: collaborator_external_pr + intake/local_validation/multi_pr_update_branch/visual_fixture_evidence.
#7118의 1,000줄 초과 변경은 별도 코드·실행·시각 검토로 취급했고 admin merge는 하지 않았다.

## 변경·증거 심사

`section_def.footnote_shape.separator_margin_bottom`과 `note_spacing`에 HWP3 hunit ×4를 넣는다.
HWPX 이름과 달리 HWP5 저장 offset 16/18이 구분선 위/아래라는 모델·serializer 계약을 대조했다.
0 입력은 기존 기본값을 보존하며 번호 장식은 #7185가 담당한다.

누적 HWP5의 실제 `FOOTNOTE_SHAPE[0]`은 위 852, 아래 568, 주석 사이 424다.
기존 Git 한컴 변환본의 동일 세 필드와 정확히 일치한다. focused 1개는 저장소 HWP3 3표본을 검사해
통과했다. 합성 식만 재계산한 결과가 아니라 저장 레코드를 직접 읽었다.
한컴 PDF 9쪽의 본문 끝·구분선·첫 각주 행을 직접 대조했다.
구분선 길이(기준 -1 대 후보 14160)와 사용하지 않는 미주 shape 차이는 남으므로 #7174 전체를 닫지 않는다.

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

![직접 확인한 비교/브라우저 증적](../assets/pr7118_7187_review/hwp3_saved_hancom_p009.png)

시각 검토 정본: [Visual Sweep](../../manual/verification/visual_sweep_guide.md).
Native/WASM의 PNG 라벨과 본문을 구분해 직접 열었다. HWP3 저장 비교 이미지는 Visual Sweep의
`make_compares`로 **후보를 한컴이 출력한 PDF(왼쪽)**와 **원본 한컴 PDF(오른쪽)**를 합친 것이다.
그 이미지의 기본 `rhwp` 라벨은 Native renderer 출력을 뜻하지 않는다.
1회차(보정 전)의 [입력/PDF 해시 원장](../assets/pr7118_7187_review/fixture-manifest.json), [시각 지표](../assets/pr7118_7187_review/visual-metrics.json),
[focused 이력](../assets/pr7118_7187_review/focused.log)를 함께 보존했다. 낮은 pixel/ink proxy는 완전 일치로 해석하지 않는다.

직접 사용한 기존 HWP/HWPX는 기존 Git 경로를 재사용했다. 1회차의 한컴 PDF와 누적 저장 HWP는 `788ab292b`에 이미 포함돼 있다.
2회차는 기존 Git 입력·PDF를 재사용하고 새 비교 이미지와 로그를 보존했다. 아직 merge하지 않았으므로 source PR/issue는 닫지 않는다.
