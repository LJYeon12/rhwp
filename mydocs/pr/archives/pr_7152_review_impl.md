---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7152_review.md
last_verified: 2026-09-15
---

# PR #7152 — 메인터너 보정 및 기록 단계

## 승인 범위

사용자는 메인터너 보정, 경계 검증, 수정 전후 스크린샷·원인·보정 내용을 리뷰 기록에 남기고 코멘트 초안을 보여 주는 단계까지 승인했다. 원격 push, GitHub comment/review 제출, Approve, merge, 이슈 close는 이번 실행 범위에 없다.

[collaborator 직접 보정 경로](../../manual/pr_review/collaborator_external_pr.md#931-contributor-pr-head-직접-보정)를 사용했다. 기존 preview worktree `/private/tmp/rhwp-review-7152`에서 원 PR head로 시작한 `codex/pr7152-review-20260915` 한 branch에 code와 기록을 순서대로 추가한다. 사용자 변경이 있는 기본 checkout의 branch·파일은 유지한다.

## 커밋과 단계

| 단계 | SHA / 산출물 | 결과 |
| --- | --- | --- |
| contributor 원 변경 | `9787d549237b4c678605c1ade7ba116ee2515baf` — feat(i18n): 메뉴·툴바 영어 표시 (2/4) | 보존. 원 작성자 rubidus-api, rewrite 없음 |
| source head 확인 | API, `git ls-remote`, 로컬 시작 SHA | 동일 확인; maintainerCanModify=true |
| 보정 범위 | 영어 부모 폭, 선택 상자 격자 수용, 영어 828/982px 전환, controller query | 완료. Format Painter의 기존 세로 정렬은 제외 |
| code·회귀 검사 | `e917cffb9f70dfc73451a32fb4c1bf3ef9cffc8c` — fix(studio): 영어 서식 도구 모음의 필드 폭과 반응형 경계 보정 | contributor commit 바로 위에 별도 로컬 commit |
| 로컬 검증 | tsc, npm test, production build, responsive 전체 E2E, 원 PR 음성 대조 | 완료. [review 결과](pr_7152_review.md#4-완료-검증과-실행-증거) |
| 리뷰 기록·초안 | review, 이 impl, comment draft, assets | 별도 docs commit으로 보존. 이 문서를 포함하므로 자신의 commit SHA를 본문에 미리 쓰지 않음 |
| push·새 head CI·최종 기록 | 아직 미실행 | 사용자 검토 후 다음 범위를 결정. 오늘할일은 최종 원격 검토 묶음 작성 시 필요한 기록만 반영 |
| GitHub review/comment·merge·후속 이슈 | 아직 미실행 | 각각 실제 승인·실행 결과가 있어야 완료로 기록 |

## 실행·증거 주의점

- 초기 browser 캡처에서 viewport 전환 직후 이미지 크기가 이전 프레임에 남는 현상을 확인했다. 실제 `innerWidth` 측정과 최종 JPEG 해상도를 대조하고 잘못된 중간 캡처는 최종 증거에서 제외했다.
- 새 E2E를 원본 CSS/controller에 적용한 음성 대조 후, 보정 파일을 복원하고 전체 responsive suite를 통과했다. 원본 재현을 위해 source를 잠시 복원한 상태는 최종 commit에 남지 않았다.
- 코드 검증 후 CSS 설명 주석만 정리했다. 테스트·실행 코드·CSS 선언 변경은 없었다.
- 저장소의 PNG 관례 대신 browser 도구가 직접 반환한 JPEG를 그대로 보존했다. UI 증거이며 한컴 PDF 비교/OVL 대상은 아니다. 원본 래스터 변경 없이 HTML에서 도구 모음 영역을 보여 주고, 그 비교 페이지를 다시 캡처했다.
- 최초 preview용 `rhwp-studio/public/pr7152-boundary.html`과 `public/pr7152-evidence/`는 로컬 표시용 untracked 파일이다. 소스·docs commit에 넣지 않는다. 커밋 대상 비교 HTML과 원본은 `mydocs/pr/assets/`에 있다.

## 다음 원격 단계와 정리

사용자가 push를 요청하면 원 PR head와 source branch가 아직 원 SHA인지 재확인한다. contributor가 새 commit을 추가했다면 이전 검증을 그대로 적용하지 않고 canonical의 미공개 보정 replay 절차를 따른다. LFS 대상·dry-run 확인 후 허용된 source branch에만 반영하고, 새 code head의 CI를 기다린다. 원 head의 녹색 CI는 보정 검증을 대체하지 않는다. 최신 devel과의 최종 통합 호환도 이때 확인한다.

코멘트 이미지 링크는 게시 시점의 실제 원격 asset SHA에 고정한다. [코멘트 초안](pr_7152_comment_draft.md)은 현재 로컬 결과를 설명하며 게시 완료를 주장하지 않는다.

원격 실행 전 되돌려야 한다면 미공개 메인터너 변경만 대상으로 범위를 정한다. contributor 이력, 사용자 기본 checkout 변경, 공유 Cargo cache는 되돌리거나 삭제하지 않는다. 현재는 사용자가 검토할 수 있도록 7715 preview 서버·worktree·비교 페이지를 유지한다. 사용자 검토 종료 후 이번 작업의 임시 자료만 정리한다.

## 코멘트 초안 표현 보완

사용자 요청에 따라 비교 페이지 전체를 캡처한 이미지를 코멘트에서 제거했다. 제목·원인·보정·경계 표·검증 결과는 Markdown으로 작성했고, 원본 도구 모음의 핵심 영역만 담은 JPEG 5장을 각각 배치했다. 정렬 패널은 열린 상태를 명시했으며, 이미지 안에 설명용 접기 버튼이나 링크를 넣지 않았다. 원본 전체 JPEG와 상세 비교 페이지는 기존 리뷰 증거로 보존했다. 영역 좌표·출처·해시는 `pr7152_evidence.json`에 추가했다. 제품 코드·검증 결과·원격 상태는 변경하지 않았다.
