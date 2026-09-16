---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7186_review.md
last_verified: 2026-09-16
---

# PR #7186 검토

## 판정

**승인** — 영문 명령 레지스트리와 대화상자 탭 범위. 개별 PR의 검토 의견이며, 이 batch의 통합 head 전체 수용 또는 원격 merge 승인이 아니다.
통합 head는 #7118의 실행 회귀와 #7184의 경계 증거 부족으로 **머지 보류**다.
CI 재실행·remote push·통합 PR 생성·source PR close는 하지 않았다.

## 검토 대상

| 항목 | 확인값 |
| --- | --- |
| 원 PR | [#7186](https://github.com/edwardkim/rhwp/pull/7186) |
| 제목 | feat(i18n): 명령 레지스트리·대화상자 탭 영어 표시 — UI 문자열 로케일 분리 4/4 (#5852) |
| 작성자 / reviewer | rubidus-api (기존 기여자) / jangster77 사전 요청 |
| 원 head | `9d65b36843aece32a1c9b456cf3833f3d2daa43b` |
| source base / 규모 | devel / 19 files, +496/-232, 2 commits |
| 원 PR 상태 | OPEN / draft=False / MERGEABLE / CLEAN |
| source CI | [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/35048795385) 및 전체 rollup 실패/대기 없음; 검토 종료 전 source head 불변 확인 |
| 통합 base | `8d45f242baa1a565357aaa38e9f459595b1e756c` |
| 통합 code head | `2a2089bf71e7a42286e0643a3fb517b7047c7320` + #7118 fixture 참조 경로 보정(실행 바이트 동일) |
| branch | `codex/open-pr-review-20260916` |
| 충돌 해결 | 텍스트 충돌 없음. |

원 commit은 `-x`로 누적 적용했고 작성자·메시지·co-author를 보존했다.
source/local SHA와 실행 명령은 [검증 기록](pr_7186_review_impl.md), 공통 제한은 [회차 보고](../../working/task_m100_6970_open_pr_stage1.md)에 있다.
route: collaborator_external_pr + intake/local_validation/multi_pr_update_branch/visual_fixture_evidence.
#7118의 1,000줄 초과 변경은 별도 코드·실행·시각 검토로 취급했고 admin merge는 하지 않았다.

## 변경·증거 심사

명령의 execute/canExecute/undo 동작과 내부 tab ID는 그대로 두고 표시 라벨을 번역 키로 연결한다.
현재 locale은 모듈 import 전에 결정되며 언어 변경은 다음 실행부터 적용하는 기존 계약이므로,
명령 상수의 초기 번역과 열린 대화상자의 표시 언어가 충돌하지 않는다.

TypeScript 일반/CI unit 검사, unit 1,740 pass/2 skipped, production build,
실제 Chrome command-palette E2E가 통과했다. 추가 브라우저 실측에서 영어와 한국어 각각 팔레트
191개를 열었다. 영어 라벨의 한글 0개이며 문단 탭은 Basic/Extended/Tabs/Borders/Background,
한국어는 기본/확장/탭 설정/테두리/배경이다. 실제 스크린샷을 열어 확인했다.

미리보기 견본 문장·일부 상태/오류 문구, 접근키 Set(D), 영문 단위 `pt` 폭 차이는 source 본문이
명시한 후속 범위다. “전체 영어화 완료”로 해석하지 않으며 #5852 종료는 하지 않는다.
브라우저 진단 초기 시도는 잘못된 로컬 base 경로/초기 포커스로 timeout 났고,
실제 앱 경로와 문서 준비·캔버스 포커스를 적용한 재실행에서 두 locale 확인을 마쳤다.

## 공통 조판 원칙

| 항목 | 판정 | 근거 |
| --- | --- | --- |
| 구현 근거·일반성 | 비해당 | 표시 라벨만 변경하며 조판/분할/baseline을 변경하지 않음 |
| 측정·배치 일관성 | 비해당 | 표시 라벨만 변경하며 조판/분할/baseline을 변경하지 않음 |
| 분할·이어받기 | 비해당 | 표시 라벨만 변경하며 조판/분할/baseline을 변경하지 않음 |
| 줄 소속·점유 높이 | 비해당 | 표시 라벨만 변경하며 조판/분할/baseline을 변경하지 않음 |
| 기준값 변경 | 비해당 | 표시 라벨만 변경하며 조판/분할/baseline을 변경하지 않음 |
| 증거 독립성 | 충족 | 실제 두 locale 브라우저·unit/E2E |
| 주장·검증 범위 | 충족 | 팔레트와 탭에 한정, 전체 영어화/issue 종료 아님 |

## 증적

![직접 확인한 비교/브라우저 증적](../assets/pr7118_7187_review/i18n-en-tabs.png)

시각 검토 정본: [Visual Sweep](../../manual/verification/visual_sweep_guide.md).
Native/WASM의 PNG 라벨과 본문을 구분해 직접 열었다. HWP3 저장 비교 이미지는 Visual Sweep의
`make_compares`로 **후보를 한컴이 출력한 PDF(왼쪽)**와 **원본 한컴 PDF(오른쪽)**를 합친 것이다.
그 이미지의 기본 `rhwp` 라벨은 Native renderer 출력을 뜻하지 않는다.
[입력/PDF 해시 원장](../assets/pr7118_7187_review/fixture-manifest.json), [시각 지표](../assets/pr7118_7187_review/visual-metrics.json),
[focused 결과](../assets/pr7118_7187_review/focused.log)를 함께 보존했다. 낮은 pixel/ink proxy는 완전 일치로 해석하지 않는다.

직접 사용한 기존 HWP/HWPX는 기존 Git 경로를 재사용했다. 새 한컴 PDF와 누적 저장 HWP는 이 검토
회차 commit에 포함한다. 아직 merge하지 않았으므로 source PR/issue는 닫지 않는다.
