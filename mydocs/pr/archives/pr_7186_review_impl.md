---
kind: snapshot
status: active
canonical: mydocs/pr/archives/pr_7186_review_impl.md
last_verified: 2026-09-16
---

# PR #7186 적용·검증 기록

[검토 판정](pr_7186_review.md) / [공통 실행·미실행 기록](../../working/task_m100_6970_open_pr_stage1.md).

| 원 commit | 누적 cherry-pick commit |
| --- | --- |
| `f6234f82c39da20989db55d750ee70745a77cfca` | `a1ae1465ee9529331e48202de76cf719f5cf1851` |
| `9d65b36843aece32a1c9b456cf3833f3d2daa43b` | `744fd4d0ca7966742d263597dba4646708c28877` |

- 원 `refs/pull/7186/head`를 fetch하고 PR API의 head SHA와 일치함을 확인했다.
- 최신 devel `8d45f242baa1a565357aaa38e9f459595b1e756c` 위의 누적 적용이다. 텍스트 충돌 없음.
- code 검증: `2a2089bf71e7a42286e0643a3fb517b7047c7320`. 별도 보정은 #7118의 동일 바이트 fixture 참조 경로와 중복 복사본 제거뿐이다.
- macOS arm64, Rust 1.93.1, `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
  처음 선택된 Xcode의 라이선스 오류는 테스트 실패로 세지 않았다. 실제 CLT 빌드 exit 0.
- 독립 target: `/Users/tsjang/rhwp/target/pr7118-7187-20260916`; shared target 삭제/이동 없음.
- `cargo build --locked --profile release-test --target-dir target/pr7118-7187-20260916`: CLT에서 성공.
- focused는 [suite/필터 명령 배열](../assets/pr7118_7187_review/focused-command.json)대로 실행했다.
  **54 run / 53 pass / 1 fail**. 실패와 제거 대조군은 #7118 검토에 기록했다.
- source 각 PR의 green CI를 누적 code head의 green CI로 간주하지 않는다.
- fmt check, base 고정 unit tier 검사 통과. fixture 경로 변경으로 생긴 generated suite drift는
  prepare 재실행 후 manifest base 검사 통과; 파생 파일은 commit하지 않는다.
- fresh dev WASM: `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_TARGET_DIR=target/pr7118-7187-20260916 scripts/wasm-pack-locked.sh --target web --out-dir pkg --dev` 성공.
- renderer 비교 명령: `venv/bin/python scripts/visual_sweep.py --file-target <key> <원본> <PDF>
  --rhwp-bin <누적 CLI> [--wasm-pkg pkg] --pages <페이지> --dpi 96 --out <임시 경로>`.
  입력·페이지·hash·실제 CLI/WASM은 [manifest/metrics](../assets/pr7118_7187_review/visual-metrics.json)와 같은 폴더의
  `*-manifest.json`에 보존한다. 임시 root는 `/private/tmp/rhwp-review-open-20260916`이다.
- HWP3 저장: `rhwp convert tests/fixtures/issue_4680/german-legislative-system.hwp
  tests/fixtures/issue_4680/german-legislative-system-open-pr-review.hwp --json`.
  최초 임시 출력과 최종 Git 경로 파일의 SHA-256은 동일하다.
- 전체 release-test / Native Skia 3종 / 새 head의 Clippy 3종은 **미실행**이다.
  선행 focused에서 blocker가 확인되어 최종 PR 제출 게이트를 완료했다고 주장하지 않는다.
  해당 회귀 해결 후 최종 head에서 실행해야 한다.
- #7186: 일반/CI-unit tsc, npm unit 1740 pass/2 skipped, production build,
  command-palette E2E, 영어/한국어 Chrome 실제 실행 통과. 다른 PR의 Rust 검증 대체가 아니다.
