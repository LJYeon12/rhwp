---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review_workflow.md
last_verified: 2026-09-13
---

# PR #7103 — 연속 TAC 표 겹침 검토

**최종 판정: 머지 보류.** 제목과 개인정보 표의 겹침은 개선된다. 그러나 표의 실제 저장 줄을 잘못
고르는 원인을 유지한 채 음수 간격을 0으로 막으므로 현재 head를 승인하지 않는다.
이 문서는 로컬 검토 결과이며 GitHub review/comment·push·merge를 실행한 기록이 아니다.

## 1. 대상과 권한

| 항목 | 확인값 |
| --- | --- |
| 원 PR / 실제 issue | [#7103](https://github.com/edwardkim/rhwp/pull/7103) / [#7096](https://github.com/edwardkim/rhwp/issues/7096) |
| 작성자 / reviewer | winchoose, 첫 rhwp PR / jangster77 사전 지정 |
| 상태 | OPEN, non-draft, MERGEABLE, CLEAN; 검토 종료 전 재조회 |
| source head | `1c5fd967674e238b3d4e42948c1abe5d4c279f6f` |
| fork / branch | `winchoose/rhwp` / `codex/investigate-hwp-form-overlap` |
| 기준 devel | `cd2d9e8a430e2664181326c684b7491aa451cd31` |
| 로컬 branch | `review/pr7103-20260913`, 주 작업공간 `/Users/tsjang/rhwp` |
| cherry-pick 검증 head | `d92086eec6fa0f053e3fc50f7845aedc0202dd27`; 원 SHA를 `-x`로 기록, 충돌 없음 |
| 증적 commit | `37245bd59` — 코드 변경 없이 한컴 PDF·3-way·OVL 보존 |
| 변경 범위 | layout.rs, integration test, HWP fixture; 3 files, 104 insertions / 2 deletions |
| 절차 | collaborator_external_pr / intake_and_review, first_time_contributor, local_validation, visual_fixture_evidence |

**fork 직접 push 확인:** 현재 계정 `jangster77`의 fork repository API 권한은 `pull: true`,
`push: false`, `maintain: false`, `admin: false`다. PR의 `maintainer_can_modify`는 `true`다.
원 head 그대로의 no-op dry-run은 up-to-date였지만, source head의 자식인 로컬 빈 commit을 만들어
실제 fast-forward 변경을 가정한 `git push --dry-run --porcelain`은 다음 Git LFS 검사에서 실패했다.

```text
Authentication error: Authentication required: You must have push access to verify locks
```

따라서 no-op 결과를 쓰기 권한 확인으로 세지 않는다. 이후 저장소 절차
[9.3.0 LFS 대상 사전 판독](../../manual/pr_review/collaborator_external_pr.md#930-lfs-대상-사전-판독)을
다시 확인했다. 권한 점검용 빈 commit의 source 대비 변경 파일은 없고 `git lfs status`의 새 object
목록도 비어 있었다. 이 경우 절차가 정한 `GIT_LFS_SKIP_PUSH=1`을 적용한 fast-forward dry-run은
**exit 0, `1c5fd9676..b14e522b2`로 통과**했다. 실제 ref update는 수행하지 않았다.

**이전의 “fork push 경로 사용 불가” 판단은 정정한다.** fork 전체의 `push: false`와 해당 PR의
수정 허용 경로를 구분해야 하며, 첫 실패는 변경과 무관한 LFS lock 확인이었다. 이번 결과는
비LFS 변경의 dry-run 통과이지 실제 push 또는 향후 LFS object 쓰기 성공 증명은 아니다.

작업지시자가 요청한 체리픽 통합 경로도 사용 가능하다. 원본 저장소 `edwardkim/rhwp`에 대한
`jangster77`의 `push: true`를 API로 확인했으며, 다시 fetch한 최신 `upstream/devel`은 위 기준 SHA와
같고 현재 검토 branch의 조상이다. 원격 contributor head도 위 source SHA 그대로다.
보정은 현재 로컬 통합 branch의 별도 commit으로 진행하고, 검증 후 원본 저장소 임시 branch에서
통합 PR을 만드는 경로를 사용한다. 현재 원격 push/PR 생성/merge/close를 완료한 것은 아니다.

## 2. 보류 사유

### F1 — 잘못 고른 LineSeg의 간격을 clamp한다

**분류: 코드 검토상 규칙 위반, 실물 입력의 줄 소속 오계산 확인.**
변경 위치는 [layout.rs](../../../src/renderer/layout.rs)의 `forward_line_seg_gap_hu` 및
`layout_table_control`의 연속 TAC 처리다. `next.vertical_pos - (current.vertical_pos +
current.line_height)`를 `max(0)`으로 바꾸지만, `current`와 `next`가 해당 두 표의 줄인지 확인하지 않는다.

실제 fixture의 첫 문단은 텍스트·char_offsets가 비어 있고 다음 순서를 가진다. 공개 parser로 모델을
읽은 결과와 압축 해제한 HWP `PARA_TEXT`의 UTF-16 원시 위치를 별도로 대조했다.

| control index | 종류 | 원시 UTF-16 시작 | control_text_positions | 현재 `p × 8` 투영 | 선택 LineSeg |
| ---: | --- | ---: | ---: | ---: | ---: |
| 0 | SectionDef | 0 | 0 | 0 | 0 |
| 1 | ColumnDef | 8 | 0 | 0 | 0 |
| 2 | Header | 16 | 0 | 0 | 0 |
| 3 | 제목 TAC 표 | 24 | 0 | 0 | 0 |
| 4 | Footer | 32 | 1 | 8 | 0 |
| 5 | 본문 TAC 표 | 40 | 1 | 8 | 0 |

`control_text_positions()`는 빈 문단에서 SectionDef/ColumnDef/Header/Footer를 자리 증가에
포함하지 않는다. 반면 `control_line_seg_index()`의 빈 carrier 분기는 이 값을 8배 하여 원시
스트림 위치로 간주한다. 따라서 원시 위치 24·40의 두 표가 모두 LineSeg 0으로 선택된다.
이 함수의 기존 결함을 이번 PR이 새로 도입했다고 주장하는 것은 아니다. **이번 변경은 이 오선택에서
나온 간격을 clamp하는 방식으로 겹침을 고친다**는 것이 보류 이유다.

| LineSeg | text_start | vertical_pos | line_height | line_spacing |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 0 | 7527 | -92 |
| 1 | 24 | 208 | 7527 | -1052 |
| 2 | 32 | 6683 | 44740 | -300 |
| 3 | 40 | 7383 | 44740 | -300 |

현재 호출이 계산하는 차이는 `208 - (0 + 7527) = -7319 HU = -97.5867px`다.
이는 첫 표와 다음 표의 소유 줄 간격이 아니다. 또한 `seg_idx + 1`은 다른 제어의 줄일 수 있으므로,
소유 줄을 바로잡더라도 다음 표의 소유 줄 조회와 기준선·바깥여백·점유 높이를 함께 검토해야 한다.
저장 줄 차이를 그대로 표 테두리 간격으로 간주하거나 모든 음수를 유효/무효로 일반화하지 않는다.

**해제 조건:** 현재 빈 carrier의 원시 control 위치와 LineSeg 소속을 공통 결과로 일치시키고,
실제 다음 TAC 표가 속한 줄의 advance/점유 영역을 소비하도록 수정한다. 정상 양수 간격, 합법적인
음수 줄간격, 비가시 제어가 끼는 경우의 적용·비적용 근거를 남긴다. 지금의 범위 내 수정으로
검증하며, 전체 엔진 재작성을 요구하지 않는다.

### F2 — 새 테스트의 정답이 한컴 배치를 검증하지 않는다

**분류: 필수 증거 부족 및 실행으로 확인한 잔여 차이.**
[새 회귀 테스트](../../../tests/cases/issue_7103_tac_table_rewind.rs)는 다음 표가 제목 표 아래이고
간격이 2px 이하인지만 검사한다. 같은 입력의 한컴 출력 위치 또는 실제 control의 줄 소속은 검사하지
않아 F1이 그대로인 현재 코드에서도 통과한다.

수정 전후 동일한 CLI/render tree로 첫 표는 y=60.5, h=92.8px이고, 다음 표의 y는
55.7→153.3px로 변한다(표시값 0.1px 반올림). 큰 겹침 제거는 실제 개선이다. 그러나 다음 경계는
한컴 PDF보다 약 4pt 위에 남는다. 비교는 glyph bbox가 아닌 표의 수평 경계/행 상단이다.

| p.1 경계 | 한컴 PDF y (pt) | 후보 render tree y (px @96dpi) | 후보 환산 y (pt) | 후보 − 한컴 |
| --- | ---: | ---: | ---: | ---: |
| 개인정보 내부 표 상단 | 151.994 | 197.2 | 147.900 | -4.094pt |
| 튜터 인적사항 색상 행 상단 | 286.487 | 376.8 | 282.600 | -3.887pt |

이는 새 회귀를 검출했다는 뜻이 아니라 **원래 보고된 배치 문제의 잔여 차이**다. 전체 글꼴·획 굵기
차이는 이번 간격 변경의 회귀로 분류하지 않는다. 한컴 PDF의 하단 로고와 본문의 글꼴이 browser
webfont와 다르므로 픽셀 수치만으로 정답 일치 여부를 판정하지 않는다.

**해제 조건:** F1 수정 뒤 실제 저장 줄 소속, 표 간 advance, 한컴 기준의 위 경계 및 후속 표의
위치를 확인하는 회귀를 추가하고 1페이지 전체를 재대조한다. 차이가 별도 기존 원인이라면 범위와
독립 근거를 분리해 #7096의 해결 주장을 한정한다. 임의 허용치 확대나 golden 재생성으로 통과시키지 않는다.

### F3 — 닫을 issue와 최종 테스트 기록을 정정해야 한다

**분류: PR metadata 오류.** PR 본문은 `closes #7103`으로 자기 PR을 가리킨다. 실제 결함은
#7096이다. 최종 해결 범위를 확인한 후 관련 issue/closing 문구를 정정해야 한다. 또한 본문의
`cargo test --lib forward_line_seg_gap_hu`는 최종 head에 존재하는 integration test와 다르다.
이전 테스트 실행 기록과 최종 head에서 실제 실행한 결과를 구분하여 갱신해야 한다.

## 3. 공통 조판 원칙과 입력 커밋

| 항목 | 근거 | 판정 |
| --- | --- | --- |
| 구현 근거와 일반성 | 잘못 투영한 control 위치에서 얻은 음수 gap을 clamp; F1 | 미충족 |
| 측정·배치 일관성 | 배치의 gap clamp만 바뀌며 올바른 소유 줄/점유 높이의 공통 결과를 입증하지 않음 | 미검증 |
| 줄 소속과 점유 높이 | 원시 위치 24·40과 투영 0·8의 불일치 실측; F1 | 미충족 |
| 사례와 증거의 독립성 | 원 HWP와 한컴 PDF, 전후 시각 비교는 확보. 정상 음수 간격·관련 경계 계약은 새 테스트로 보호하지 않음 | 미검증 |
| 기준값 변경 | 기존 baseline/golden/래칫 변경 없음. 새 2px 상한은 한컴 좌표의 독립 검증을 대신할 수 없음 | 미검증 |
| 주장과 검증 범위 | 본문의 closing 번호·최종 테스트 설명 불일치; 새 회귀와 잔여 차이를 구분해 기록 | 미충족 |

**검증 입력 커밋 확인: 로컬 검토 branch에서 충족.** 원 HWP는 source 및 cherry-pick commit에
포함되어 있고 기준 PDF는 `37245bd59`에 추가했다. 기존 동일 PDF가 없는지 바이트 해시로 확인했다.
기존 HWP/HWPX는 원래 경로를 재사용했고 복사·이름 변경한 중복 파일을 추가하지 않았다.
[입력 manifest와 재현 절차](../assets/issue_7103/README.md)에 전체 경로·출처·SHA-256을 기록했다.
원격 contributor head에는 새 한컴 PDF·검토 문서가 아직 없으며 로컬 기록과 구분한다.

## 4. 실행 검증

| 검사 | 결과 / 범위 |
| --- | --- |
| source Full CI | [34757985771](https://github.com/edwardkim/rhwp/actions/runs/34757985771) 성공; lint, Native Skia, archive A/B/C/D 실제 실행 |
| 기타 source CI | CodeQL 34757985826, Render Diff 34757985678, Adapter 34757985911, Proptest 34757985790 성공 |
| 로컬 focused | #7103 / #6078 / #6181 각각 1 PASS, 실패 0; release-test, 전용 target |
| 음성 대조 | 동일 최신 devel의 layout.rs로 만든 CLI에서 표 y=55.7로 겹침 재현; 후보는153.3. 대조 뒤 source 원복·focused 재실행 |
| OVR5 | 5문서 142페이지, 표 개체 48개; devel→후보 기하 회귀 0건, 기본 2px tolerance |
| fidelity 원장 | p.1 요청/완료1, 누락0; body/각주·표/footer·frame·표셀 겹침 후보0 |
| Visual Sweep | 전후 각1페이지. flagged 페이지 1→0, pixel match 78.78501→87.38109%, ink match 13.62244→36.66524% |
| 실제 확인 경로 | fresh native CLI의 SVG/render tree + Chrome 152.0.7977.83, Studio 공통 webfont raster, 한컴 PDF |
| 미검증 | 최종 통합 head의 전체 suite 재실행·새 WASM 모듈의 Studio 실행, 정상 음수 간격 경계 계약 |

CI는 **원격 source head**의 완료 증거를 재사용했다. 로컬 cherry-pick head 자체의 GitHub CI로
표현하지 않는다. broad lint/Native Skia/full suite를 같은 코드에 다시 실행하지 않았으며, 수용하려면
보정된 최종 head에 적용되는 검증 게이트를 다시 충족해야 한다.

시각 후보 0은 정답 판정이 아니다. 한컴 PDF 텍스트층에서만 c/d/e/f/g 각10개(총50)가 추출되어
fidelity text 원장에 reference-only로 잡혔다. raster에 보이는 해당 영문 본문 소실로 확정하지 않았고,
문자 완전 일치도 주장하지 않는다. font/encoding 차이와 기하 잔여 차이를 별개로 남긴다.

[3-way 전체 페이지](../assets/pr7103_tac_review_p001_3way.png)와
[OVL 정합 패널](../assets/pr7103_tac_review_p001_ovl.png)을 직접 확인했다.
큰 겹침 제거, 후속 표와 서명·하단 로고 유지, 기준과 남은 수평 경계 차이를 확인했다.
OVL은 R=한컴 gray, G=B=후보 gray다. 글자 프린지는 font 환경 차이를 포함한다.

## 5. Merge 후 contributor PR comment 계획

현재는 보류이며 게시하지 않는다. 보정·재검토 후 merge되는 경우에만 다음을 최종 결과로 갱신한다.

- [Visual Sweep 정본](../../manual/verification/visual_sweep_guide.md), 실제 입력·head·페이지 및
  후보 수/지표/직접 시각 판정을 링크한다. 현재 1페이지 flagged 1→0은 원 head 검토 결과다.
- 대표 이미지는 `mydocs/pr/assets/pr7103_tac_review_p001_3way.png`와
  `mydocs/pr/assets/pr7103_tac_review_p001_ovl.png`를 사용한다. 보정 결과가 바뀌면 이미지도 갱신한다.
- 실제 merge SHA 확정 후
  `https://raw.githubusercontent.com/edwardkim/rhwp/<merge-commit-sha>/mydocs/pr/assets/pr7103_tac_review_p001_3way.png`
  형식으로 고정하고 `gh pr comment ... --body-file`로 게시한 뒤 API 재조회한다.
- 승인된 통합 PR 경로를 쓰면 merge된 통합 PR과 반영 SHA를 원 PR에 명시하고 후속 close를 처리한다.
  #7096은 확인한 해결 범위에 따라 처리한다. 지금 close/merge 완료로 기록하지 않는다.


## 6. 메인터너 보정·체리픽 처리 가능 여부

**처리 경로는 가능하다. 보정 구현과 최종 수용 판정은 아직 미완료다.** 원 contributor commit은 이미
최신 devel 위에 저자·source SHA를 보존해 적용했다. 같은 branch에서 원 commit을 다시 cherry-pick하지
않는다. [보정 범위와 검증 계획](pr_7103_review_impl.md)에 공통 줄 소속과 줄 advance의 수정 지점,
검증 및 통합 후 원 PR 처리 순서를 기록했다.

기존 focused 3개·OVR5·source CI 통과는 **보정 전 후보**의 증거다. 보정 commit에 그대로 승계하여
`메인터너 보정 후 수용 가능`으로 올리지 않는다. 원 head의 F1/F2를 수정한 정확한 보정 SHA와
통합 검증 결과가 확보될 때만 최종 판정을 갱신한다.
