---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7238_review.md
last_verified: 2026-09-17
---

# PR #7238 검토

## 최종 판정

**머지 보류** — CLI는 #7230의 별도 helper를 사용하므로 #7238이 주장한 구역 시작·비트 보존 계약이 제품 경로 전체에 적용되지 않는다. Studio Ctrl+Enter·HwpCtrl BreakPage까지 바꾸는 범위에는 offset 0 한컴 동작 근거가 추가로 필요하다.

이 판정은 로컬 cherry-pick 통합 검토이며 GitHub APPROVE 제출·remote push·PR 생성·merge·issue close는 수행하지 않았다.

## Metadata·계보·CI

| 항목 | 확인값 |
| --- | --- |
| PR | [#7238: 수정: 문단 시작의 쪽 나눔은 문단을 가르지 않고 그 문단에 건다 (#7218)](https://github.com/edwardkim/rhwp/pull/7238) |
| 작성자 / reviewer | planet6897 / jangster77 |
| base / state | devel / OPEN, non-draft |
| 규모 | 4 files, +252/-9, 1 commit |
| source head | `103ab177ab3c536a6d74d05ed59c10c67f37c605` |
| 적용 commit / 통합 code head | `3127bcce9` / `3127bcce945b00bf7c767df2fb9da1f67ebb3633` |
| 조회 상태 | MERGEABLE / CLEAN; merge 직전 재조회 필요 |

- [Build & Test](https://github.com/edwardkim/rhwp/actions/runs/35204539449/job/105150854246): **SUCCESS**.
- [Native Skia tests](https://github.com/edwardkim/rhwp/actions/runs/35204539449/job/105147135139): **SKIPPED**.
- [CodeQL](https://github.com/edwardkim/rhwp/runs/105151551208): **NEUTRAL**.

SKIPPED/NEUTRAL은 해당 검사가 실행되어 통과했다는 의미로 세지 않는다. source CI는 통합 head의 CI가 아니다.

## 충돌 조정·통합 심사

source #7238은 #7230과 독립적으로 같은 #7218을 수정했다. 기능 source는 그대로 적용하고 겹친 두 테스트 파일을 조정했다. `insert_page_break_contract.rs`는 #7230의 중간 분할과 봉투 검사를 유지했다. `issue_7218_page_break_at_paragraph_start.rs`는 #7230 CLI 3개를 유지하고 #7238 core 4개를 `core_contract` 모듈로 모두 보존했다. header/footer fixture 생성 helper의 명시적 문단 삽입도 유지했다. [충돌 처리 기록](../pr_7238_review_impl.md).

핵심 코어는 offset 0에서 분할을 피하고 `raw_break_type |= 0x04`, synthesized=false, reflow/vpos/recompose를 수행한다. 코어 4개와 header/footer 4개가 통과했고 반환 paraIdx를 Studio `command/commands/page.ts` 및 `hwpctl/actions/text.ts`가 실제 소비함을 확인했다. 임의로 커서를 +1하는 회귀를 발견한 것은 아니다.

**통합 경로 미충족:** #7230이 CLI를 다른 helper로 배선해서 이 코어 보정이 CLI/MCP에서 우회된다. [#7230의 실제 CLI 재현](pr_7230_review.md#코드독립-실행-심사)에서 첫 문단 pageBreak=0을 확인했다. 원 #7238 본문의 구역 시작 pageBreak=1, 다른 비트·명시적 break 보존 주장은 통합 CLI 전체에 적용되지 않는다.

**필수 증거 부족:** 기존 코어의 의미를 바꾸므로 Studio Ctrl+Enter와 HwpCtrl `BreakPage`에도 적용된다. HWPX의 break-before 저장 의미와 "내용 문단도 pageBreak=1을 가진다"는 코퍼스 통계만으로 대화형 명령의 offset 0 반복·빈 첫 문단 동작을 입증할 수 없다. 기존 한컴 scenario의 문단 중간 offset 사례는 이 경계를 검사하지 않는다. 실제 한컴 명령 실행과 rhwp 제품 경로를 같은 문단/커서에서 비교하지 못했으므로 호환성 회귀가 재현됐다고 쓰지 않고 미검증으로 남긴다.

**해제 조건:** CLI 속성 설정 계약과 사용자 문단 분할 명령의 의미를 구분해 공통 진입점을 결정한다. 문서/구역 첫 문단·일반 문단 시작·빈 문단·반복 입력·문단 중간을 실제 한컴 명령과 대조하고, 대상 문단·반환 커서·빈 쪽·저장 후 재열기를 검증한다. 저장 속성 설정에만 한정한다면 명령 동작을 보존하는 방식도 가능하다. UI 전체 엔진 재작성을 요구하지 않는다.

시각 입력은 #7230과 동일하므로 HWPX/PDF/PNG를 중복 복사하지 않았다. [입력 해시·4쪽 비교·기존 개요 번호 차이](pr_7230_review.md#visual-sweep-직접-검토)를 함께 사용한다. 전체 #7218 해결 및 원 PR close는 아직 진행하지 않는다.

## 공통 조판 원칙 준수

렌더 영향: **있음, Visual Sweep 필수**. 편집/생성 경로도 페이지 가시 출력과 fixture/PDF 주장을 포함하므로 적용한다.

| 항목 | 판정 | 근거 |
| --- | --- | --- |
| 구현 근거와 일반성 | 미검증 | 저장 비트 근거는 있음, 사용자 명령 의미의 독립 oracle 부족 |
| 측정·배치 일관성 | 미검증 | 코어 reflow는 적용되나 CLI 우회 경로 존재 |
| 분할·이어받기 계약 | 미충족 | 두 PR의 offset 0 속성/분할 범위와 저장 계약 불일치 |
| 줄 소속과 점유 높이 | 비해당 | 표 조각 높이·저장 LineSeg 소유 규칙 변경 없음 |
| 사례와 증거의 독립성 | 미검증 | CLI/core 합성 계약과 한컴 Ctrl+Enter 실측을 구분 |
| 기준값 변경 | 충족 | 중간 분할·header/footer 보호 유지, 원 core 테스트 4개 보존 |
| 주장과 검증 범위 | 충족 | 호환성 우려를 실행 검출 회귀로 과장하지 않음 |

[통합 검토 공통 실행·binary/WASM hash·명령·정확한 검증 범위](pr_7212_review.md#통합-검토-공통-실행)를 따른다.

## Visual Sweep·입력 커밋 확인

**충족(공유 입력)** — [#7230의 입력 SHA-256·확인 commit·4쪽 전체 PNG](pr_7230_review.md#visual-sweep-직접-검토)를 사용한다. 두 PR에 동일 HWPX/PDF/PNG를 중복 추가하지 않았다. 별도 UI 명령 oracle는 미검증이며 위 보류 해제 조건과 구분한다.

## Merge 후 contributor PR comment 계획

보류 PR은 해제·최종 CI·실제 merge가 완료된 뒤에만 게시한다. 한국어로 기여에 감사하고 실제 merge SHA·최종 head CI URL·수정 범위·실제 검증 범위·남은 차이를 설명한다. [Visual Sweep 정본](https://github.com/edwardkim/rhwp/blob/devel/mydocs/manual/verification/visual_sweep_guide.md#github-merge-comment)을 직접 연결한다.

- `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr7230_review/outline_after_wasm_review_001.png`
- `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr7230_review/outline_after_wasm_overlay_001.png`
- `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr7230_review/outline_after_wasm_review_002.png`
- `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr7230_review/outline_after_wasm_overlay_002.png`

페이지·후보 수·pixel/ink 지표와 사람의 판정을 함께 적는다. review 패널만으로 standalone overlay를 대체하지 않으며 모든 위 대표 쪽을 댓글 본문에 실제 이미지로 표시하고 `<details>` 밖에 둔다. PR과 관련 issue 댓글 모두 같은 해시 고정 경로를 사용한다. UTF-8 파일+`gh ... --body-file`로 게시한 뒤 API/렌더된 본문에서 한국어·실제 head·이미지 URL과 표시를 재확인한다. 해결하지 않은 issue를 일괄 close하지 않는다.
