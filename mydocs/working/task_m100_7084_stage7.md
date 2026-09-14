# #7084 Stage 7 — 최종 결과 승인·제출 전 통합 점검

- Issue: #7084
- 승인: 2026-09-14 「승인합니다」 — 최종 보고서 승인 및 최신 devel 확인·PR 준비 진입.
- 점검 head: `82948e6f389345da835915ed6d72d512a1843663`.
- 제품/테스트: `230f801136a4fc73725b847fe56c49b2c11c88bd`.
- 원격 push·PR 생성·댓글·병합·close 승인은 포함하지 않는다.

## 1. 선택한 절차

- base route: `collaborator_self_merge` — 내부 타스크의 self PR 준비, 아직 PR 번호 없음.
- modifiers: `intake_and_review`, `local_validation`, `visual_fixture_evidence`,
  `rework_and_exceptions`(누적 변경 1,000줄 초과).
- loaded documents: `pr_review_workflow.md`, `pr_review/README.md`, 위 기본·보조 문서.
- PR 번호를 예측해 review 문서를 만들지 않는다. 번호 기반 self-review와 오늘할일 갱신은
  별도 승인된 push·Open PR 생성 뒤 같은 PR의 후속 기록으로 한다.

## 2. 최신 devel 확인 결과

작업 트리가 깨끗한 상태에서 `git fetch upstream devel`을 수행했다.

- 기존 공통 base: `70bf40af2a2818e72bd58b4fa66e2d4c06de2b51`.
- 최신 원격 devel: `11860a9f4186438a3cb9c355e3cd8d2257162d3a`.
- `git rev-list --left-right --count HEAD...upstream/devel`: **25 / 42**.
  42는 원격에만 있는 merge·문서 커밋을 포함한 수이며 PR 42개라는 뜻이 아니다.
- 주요 유입: #6984 하이퍼링크 편집·PDF 보존, #7110 TAC 줄 소속/저장 위치,
  #7108 PDF 합성 굵게.
- `.github/` workflow 변경 없음. `scripts/` 차이는
  `scripts/tests/font_rule_projection_gen.test.mjs`이며 폰트 규칙 보정의 검사 변경이다.
- 로컬 devel과 작업 브랜치의 포인터는 변경하지 않았다. fetch를 로컬 devel 동기화 완료라고 쓰지 않는다.

`git merge-tree --write-tree upstream/devel HEAD`는 **종료 코드 1**이다.
결과 tree `94bcbdf760eb173c68c0a46d65faf77954ef96dc`에는 충돌 표식이 있으며 검증 후보가 아니다.
실제 checkout/index에 merge를 시작하지 않았고, 사용자 작업·WASM·공유 target을 변경하지 않았다.

| 충돌 파일 | 확인한 내용 | 제안하는 최소 해결 |
| --- | --- | --- |
| `src/wasm_api.rs` | 같은 삽입 위치의 `mod hyperlink;`와 `mod canvas_metrics;` | 두 모듈 선언 모두 유지 |
| `mydocs/orders/20260913.md` | #6936/#7108 기록과 #7084 기록의 같은 위치 삽입 | 양쪽 section을 독립적으로 유지 |
| `mydocs/orders/20260914.md` | 양쪽 신규 파일의 #7096/#7110 기록과 #7084 기록 | 날짜 제목 하나, 양쪽 section 내용 보존 |

`composer.rs`, `typeset.rs`, `layout.rs`, 코어 rendering과 Studio bridge 등은 텍스트 자동 병합됐다.
하지만 **자동 병합은 동작 검증이 아니다.** 특히 하이퍼링크의 SVG/PDF layer tree 경로와
#7084 portable 측정 문맥 복원, TAC의 측정/배치와 보충 메트릭 전달을 통합본에서 확인해야 한다.
현재 실행으로 새 회귀를 검출한 것은 아니며, 통합 검증 필요 지점이다.

## 3. 권고하는 다음 절편 — 해결 방침 승인 요청

1. 현재 `task_m100_7084`와 기존 검증 worktree를 유지한다. 새 고아 브랜치나 재작성 이력을 만들지 않는다.
   실행 직전 원격이 더 바뀌었는지 확인하고, 새 충돌·영향 범위가 있으면 다시 분리해 보고한다.
2. 최신 devel을 작업 브랜치에 일반 merge하고 위 3곳만 양쪽 내용을 보존해 해결한다.
   의미상 다른 제품 충돌이 발견되면 추정 구현으로 덮지 않는다.
3. 기존 `rhwp-rust-review-7084-c`의 clean 상태/이번 소유 파생물만 확인해 통합 후보로 갱신하고,
   고정 `/home/edward/mygithub/rhwp/target/pr-review`를 사용한다. target 이동·삭제는 하지 않는다.
4. 집중 메트릭/세션·기존 shaping 보호 → Rust 전체 회귀·Native Skia 3종·세 Clippy와 workspace
   build → Docker WASM·TypeScript/npm·실제 Chrome → 기본 Render Diff/직접 PDF 순서로 검증한다.
   공유 Cargo는 순차 실행한다. 실패하면 원인을 분류하고 이전 head의 통과로 대체하지 않는다.
5. 새 통합본에서 원본 이모지, VS16/ZWJ 경계, 자동 복구와 portable 출력 복원을 재확인한다.
   하이퍼링크/TAC 관련 기존 집중 검사도 병합 변경에 맞춰 선택한다. 기존 시각 판정은 과거 후보의
   유효한 증거로 보존하되 통합본 판정과 구분한다.
6. 검증 입력 보존을 마감한다. 원본 두 파일은 이미 `samples/issue3587/`에 추적되어 있다.
   한컴 기준 PDF는 기존 추적 파일과 내용 중복 여부를 확인한 후 누락분만 `pdf/` 정식 경로에
   보존하고 관련 manifest/쪽수 원장을 검사한다. PDF 재변환·신규 HWP/HWPX 추가는 필요하지 않다.
   대표 최종 이미지만 안정 경로에 보존하며 output 로그·JSON·폰트 bytes는 제출하지 않는다.
7. 실제 검증한 입력과 후보 commit의 hash를 대조하고 PR 본문 초안·제출 파일 목록을 확정한다.
   이후 remote push·Open PR 생성 승인을 요청한다. PR 채번 전 review 파일명은 만들지 않는다.

이 계획은 필수 통합 검증을 승인된 제품 범위 안에서 수행하기 위한 것이다. 폰트 기능 확대나
별도 이슈 생성을 추가하지 않는다. 새 소스가 통합되므로 기존 Stage 5/6 전체 결과를 통합본에
그대로 재사용할 수 없다. 이번 점검에서는 긴 테스트·빌드를 시작하지 않았다.

## 4. 현재 상태

- 최종 보고서 승인 기록을 반영했다.
- fetch·정확한 SHA 비교·충돌 시뮬레이션과 변경 영향 점검을 완료했다.
- **PR 준비 완료는 아니다.** 실제 충돌 해결 방침 승인과 통합본 검증이 남았다.
- 제품 수정·실제 merge·원격 write는 하지 않았다. 문서 점검 결과만 로컬 커밋으로 보존한다.

## 5. 승인 후 실행 — 통합 및 기준 PDF 보존

해결 방침 승인 「네」 후 다시 fetch했다. devel은 `11860a9f4`로 동일했다.
일반 merge `27b28af13`에서 위 3곳을 양쪽 보존으로 해결했다. 기존 review worktree를 같은
commit으로 갱신했고, fmt 및 #7084 session 집중 11건이 통과했다.
이후 전체 게이트는 기준 PDF 보존·쪽수 원장 추가를 포함한 후보로 실행한다.

기존 `output/7084/oracle/` PDF 2개는 추적 중인 `pdf/`·`samples/` PDF와 크기/내용 hash를
대조했으며 같은 bytes의 파일이 없었다. 재변환 없이 다음 경로에 복사한다.

| 원본 (기존 Git 파일) | PDF 정식 경로 | PDF SHA-256 |
| --- | --- | --- |
| `samples/issue3587/c-form-labnote-001-stage11-filled.hwp` | `pdf/issue3587/c-form-labnote-001-stage11-filled-hwp-2020.pdf` | `ba1d5fbe6af800453143d8a7a16733339a25e3943764cb3d7f1e090fa820f83a` |
| `samples/issue3587/c-form-labnote-001-stage11-filled.hwpx` | `pdf/issue3587/c-form-labnote-001-stage11-filled-hwpx-2020.pdf` | `0d4dadfd9523991c722bbbdeb83641c26426820beb80f139409a2c5488125b86` |

PDF는 각 24,211/24,215 bytes, 2쪽, 595×841pt, Creator/Producer `Hancom PDF 1.3.0.550`이다.
출처는 Stage 1의 MCP engine/profile 2020 변환이며 실제 서버 Hancom 버전은 **12.0.0.4605**였다.
파일명 2020은 서비스 engine 선택을 뜻하며 실제 제품 버전을 11.x로 바꾸어 해석하지 않는다.
원본 저장 제품은 HWP null, HWPX `hancom-office-2020`이며 기존 변환 선택을 유지한다.

`regenerate.py`의 `rhwp_info`와 `pick_oracles`를 이번 두 입력에 한정해 호출했다.
통합본 native CLI는 각각 2쪽·모아찍기 false였고, 형식별 위 PDF를 정확히 선택했다.
독립 PDF의 2쪽을 기대값으로 기존 쪽수 원장에 **2/2 신규 행 두 개만** 추가한다.
기존 행이나 허용치는 변경하지 않고 관련 원장 검사는 전체 integration에 포함해 재실행한다.
두 원본은 새 sample이 아니며, 바이너리 입력은 변경하지 않았다.

## 용어

- merge simulation: 작업 트리를 바꾸지 않고 두 commit의 병합 결과·충돌을 계산하는 검사.
- head / base: 제출 후보의 끝 commit / 비교·병합 대상 기준 commit.
- portable: 브라우저 전용 측정에 의존하지 않는 출력 문맥.
- TAC (Treat As Character): 객체를 글자처럼 글줄 흐름에 참여시키는 속성.
- CI (Continuous Integration): 지속적 통합 검사. 로컬 통과와 원격 최신 head 통과는 별도다.
