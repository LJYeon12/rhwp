---
kind: snapshot
status: historical
canonical: mydocs/manual/pr_review/visual_fixture_evidence.md
last_verified: 2026-09-13
---

# PR #7103 검증 입력과 시각 증적

최종 판정과 보류 해제 조건은 [개별 review](../../archives/pr_7103_review.md)를 따른다.

## 입력 동일성

아래 실행 입력을 증적 commit `37245bd59`의 Git blob과 SHA-256으로 대조했다. 전부 일치한다.
원 HWP는 winchoose가 PR #7103 source `1c5fd9676`에 추가한 파일을 재사용했다.
한컴 PDF만 새로 보존했고 기존 tracked PDF 중 크기·SHA가 동일한 파일은 없었다.
나머지 7개 HWP/HWPX는 기존 focused/OVR 입력으로 원래 경로를 사용했다.

| 경로 | bytes | SHA-256 |
| --- | ---: | --- |
| `tests/fixtures/issue_7103/ari-tutoring-application.hwp` | 67584 | `09596c115082f8fb873312152553741b15fe2ccfcdeecd9357195d62c94f2a43` |
| `pdf/ari-tutoring-application-2020.pdf` | 93633 | `37970ce6dffaff2d3bc119549753e591215404ca7e8f19857676b11dbebbc5a8` |
| `samples/hwp3-table-caption.hwp` | 26433 | `e8ac29656b1ca41f96b1dd9f6ef88be294c61165e2daf40f75873814291795e1` |
| `samples/issue6181/156562368_inline_tac_table_line_advance.hwpx` | 4562989 | `e5f92644f671eb8cfb0cfea9ebe9837c390b46b29329ec7858ed61d1ce423f96` |
| `samples/KTX.hwp` | 163840 | `b6c1492152f53e8dd7d4bbbb4faca88866bb8458e9018c70c936cd469ea6fab3` |
| `samples/exam_math.hwp` | 770048 | `e40e3d675373c8efb3a844fc71f209600d3b0db987a04b3808b8e74a6b1671fe` |
| `samples/21_언어_기출_편집가능본.hwp` | 435200 | `905454045ca2e236839a7cab59750678116d08af3db31dbf846819af355b8d15` |
| `samples/aift.hwp` | 5724672 | `a3e94e613a7d3dad0ee11e2df8f9572a5b7c2d704602960c2075b5fd22df995c` |
| `samples/biz_plan.hwp` | 33792 | `8b786d6824622afae2220b203beeef6e5592157e1896fea055ebc602817113c1` |

## 한컴 기준

- 입력 `lastSavedWith.product=hancom-office-2022`, 저장 버전 `12.0.0.535`에 따라 engine 2020 선택.
- client `hwp-convert-mcp-2024-client-20260824-011002`, 실제 한컴 `12.0.0.4605`.
- job `54190c9b-5b88-4447-aa35-e6cd0274f8ea`; start → status(succeeded, terminal=true) → download 완료.
- 2026-09-13 13:46:07Z 시작, 13:46:32.911Z 완료; elapsed 25767ms, worker23620ms.
- direct DLL host, input preprocessing 없음, printMethod0, one-up PDF; 1페이지,93633bytes.
- server/download SHA가 위 PDF 해시와 일치. 인증 환경 파일·URL·토큰은 증적에 포함하지 않는다.
- 한컴 서버와 macOS Chrome의 실제 글꼴 공급이 다르므로 동일 glyph shape를 주장하지 않는다.

## 원인 추적

원시 HWP의 `BodyText/Section0`를 deflate 해제해 첫 `PARA_TEXT` 레코드를 읽었다.
UTF-16 길이는49이며 extended control 시작은 0(secd),8(cold),16(head),24(tbl),32(foot),40(tbl)다.
`rhwp::parser::parse_document`로 읽은 첫 문단은 빈 text/char_offsets 및
`control_text_positions=[0,0,0,0,1,1]`이다. 모델 기반 probe에서
`control_line_seg_index` 빈 carrier 분기의 `p*8` 투영을 그대로 대조하면 두 표 모두 LineSeg0이다.
LineSeg의 text_start는0/24/32/40이므로 표의 원시 앵커와 맞지 않는다.
probe는 공개 parser 모델과 기존 분기 계산의 진단이며 새로운 제품 회귀 테스트로 세지 않는다.

## 코드·산출 재현

- 후보: `d92086eec6fa0f053e3fc50f7845aedc0202dd27`, 원 PR을 최신 devel에 cherry-pick.
- 대조군: `cd2d9e8a430e2664181326c684b7491aa451cd31`의 layout.rs로 동일 target에서 CLI build.
  이 파일이 contributor의 유일한 production 변경이다. 대조 빌드 후 파일을 후보 내용으로 복원하고
  #6078/#6181/#7103 focused를 후보 source에서 순차 재실행했다. 다른 production 차이는 없다.
- scratch: `/private/tmp/rhwp-7103-review-20260913`; baseline-rhwp와 candidate-rhwp를 따로 보존했다.
- 전용 target: `/Users/tsjang/rhwp/target/pr7103-review-20260913`; 공유 target 삭제 없음.

```sh
node scripts/rust-test-suite-manifest.mjs --prepare
node scripts/run-rust-test.mjs issue_7103_tac_table_rewind -- \
  --cargo-profile release-test --target-dir target/pr7103-review-20260913
# 같은 방식으로 issue_6078_inline_table_seg_lookup,
# issue_6181_inline_tac_table_line_advance를 각각 실행: 모두 1 PASS.

RHWP_BIN=/private/tmp/rhwp-7103-review-20260913/candidate-rhwp \
  venv/bin/python tools/fidelity_compare/fidelity_compare.py 0 0 \
  --source tests/fixtures/issue_7103/ari-tutoring-application.hwp \
  --reference-pdf pdf/ari-tutoring-application-2020.pdf \
  --label pr7103 --reference-grade hancom-2020-12.0.0.4605 \
  --text-only --export-all-svg --layout-ledger \
  --out-dir /private/tmp/rhwp-7103-review-20260913/fidelity

VISUAL_SWEEP_CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
  venv/bin/python scripts/visual_sweep.py \
  --rhwp-bin /private/tmp/rhwp-7103-review-20260913/candidate-rhwp \
  --key pr7103 --hwp tests/fixtures/issue_7103/ari-tutoring-application.hwp \
  --pdf pdf/ari-tutoring-application-2020.pdf --pages 1 \
  --out /private/tmp/rhwp-7103-review-20260913/sweep
# baseline-rhwp / sweep-baseline으로 별도 반복했다.
```

OVR는 `tools/object_visual_regression.py`를 import한 로컬 launcher에서 `RHWP`를 각 fresh binary로,
`git_head()`를 해당 코드 SHA로 명시했다. 기존 `target/release/rhwp`를 사용하지 않았다.
OVR5 각 문서에 `--no-hwp --save-baseline`을 대조군으로 실행하고, 후보에서 그 `baseline.json`을
`--baseline`으로 전달했다. 27/20/15/74/6페이지 및 표9/9/3/27/0개가 각각 일치하며 회귀0건이다.
이 5문서의 새 한컴 변환·raster 비교를 한 것은 아니다.

## 대표 이미지

- [3-way](../pr7103_tac_review_p001_3way.png): 왼쪽 한컴 PDF, 가운데 devel, 오른쪽 PR 후보.
  공식 sweep의 PDF/Studio webfont PNG를 페이지 전체 비율 그대로 같은 크기로 놓았다.
- [OVL](../pr7103_tac_review_p001_ovl.png): 한컴 gray를 R, 후보 gray를 G/B로 합성했다.
  검정=일치 잉크, 빨강=후보 잉크, 청록=한컴 잉크. 별도 내용 정렬이나 좌표 이동을 하지 않았다.
- 최종 패널의 p.1 제목·개인정보·튜터/튜티 표·서명·로고를 직접 확인했다.
  겹침 개선과 잔여 경계 차이를 review에 구분했다. font 차이를 이번 gap 변경의 회귀로 세지 않는다.
- 중간 SVG/PNG/render-tree JSON/로그는 scratch에 보존하고 commit하지 않는다.
  비교에 사용한 HWP/HWPX와 독립 한컴 PDF는 위 표의 저장소 경로에 모두 있다.
