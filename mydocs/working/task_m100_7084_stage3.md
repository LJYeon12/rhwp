# #7084 Stage 3 — B 절편 측정 준비부 및 남은 적용 경계

- 일자: 2026-09-13
- 계획: [구현계획](../plans/task_m100_7084_impl.md)
- 상태: **B 진행 중. 측정 준비부만 구현·검증. Studio 재조판/paint 활성화 미완료.**
- 선행: [A 결과](task_m100_7084_stage2.md), 인계 `b9b271238`.
- 이번 제품/테스트 SHA: `eef7b3293f803ca3f8b702419c381f512e808b44`.

## 1. 이번에 구현한 범위

`rhwp-studio/src/core/supplemental-text-metrics.ts`는 Canvas2D의 일괄 측정 준비부다.
아직 WasmBridge/RendererSession에서 호출하지 않으며, Rust snapshot 등록도 하지 않는다.
기존 Studio 화면이 개선되었다는 뜻이 아니다.

- 문서·폰트 세대에 귀속된 cache를 소유한다. 폰트 준비 대기 중 세대 변경/폐기 시 늦은 결과를 버린다.
- `fontsReady` 완료 후 요청 descriptor를 **일반 Canvas setter**에 넣는다.
  presence probe용 raw setter로 Studio 대체 폰트 처리를 우회하지 않는다.
  설정 후 `context.font`를 읽어 실제로 해석된 descriptor를 함께 반환한다.
- 결과는 `canvas2d` / `backendMeasured`로 표시한다. 실제 fallback face를 식별한 `verifiedSource`라고 하지 않는다.
- Canvas가 잘못된 CSS font 입력을 무시하고 이전 font를 유지하는 경우를 두 초기 설정값으로 검출한다.
- NaN/Infinity/음수 advance, 중복 키, 여러 scalar cluster, 제어문자/단독 surrogate를 거부한다.
  정상 0 advance는 허용한다.
- 등록 후보 전체가 성공해야 cache를 교체한다. 중간 실패는 앞서 측정한 항목도 부분 게시하지 않는다.
- 동일 세대·키·문자·descriptor 재요청은 추가 Canvas 생성/측정 없이 cache를 쓴다.
  반환 자료 수정이 cache를 바꾸지 않도록 복사한다.
- 4,096항목·UTF-8 key 4MiB 상한을 요청 및 누적 cache에 적용한다. 큰 문자열은 encoding 전에
  UTF-16 길이로도 하한 검사를 하며, 해결된 descriptor도 cache 예산에 포함한다.

## 2. 코드 연결에서 확인한 경계

### 2.1 backend별 조판 문맥

`DocumentCore.styles`는 Canvas2D 전용이 아니다. `build_page_layer_tree_with_profile`의 결과는
Canvas2D, SVG, CanvasKit 및 PDF 경로에서 사용된다. `RenderProfile::Screen/Print`도 backend와 다르다.
여기에 Canvas 측정 snapshot을 항상 등록하면, 다른 backend가 같은 폰트 선택을 보장하지 못하면서도
브라우저 진행폭으로 만든 pagination/char positions를 사용하게 될 수 있다.

A의 `MetricContext.backend`와 등록 거부 검사는 **자료의 세대와 출처**를 보호한다.
그 자체로 이미 계산된 문서 코어의 공통 pagination/page-tree cache가 backend별로 분리되지는 않는다.
따라서 공통 스타일에 연결했다는 사실만으로 B의 backend 보호 요건을 통과시킬 수 없다.

기존 Studio `view/renderer-session.ts`에는 `documentRevision`, `resourceGeneration`,
`beginDocument`, `invalidateDocument`, `resolve`, `dispose`가 있다. 이 소유권을 활용하되,
실제 backend 변경/내보내기에서 **측정 문맥과 조판 cache가 함께 선택·무효화**되는 연결이 필요하다.
원본 Document 복사·저장 변경이나 단순 paint 단계의 scale 제거로 대신하지 않는다.

### 2.2 자연 advance와 축소 장평의 단위

`web_canvas.rs::draw_text_positioned`는 첨자를 적용한 뒤
`condensed_ratio_draw_params`로 실제 font size와 가로 배율을 구한다.
축소 장평에서는 기존 규칙에 따라 각각 √ratio를 사용한다.
반면 A의 `SupplementalMetric`은 첨자 적용 크기에서의 **장평 적용 전 자연 advance**를 기대한다.

이번 준비부의 `naturalAdvancePx`는 요청받은 **실제 CSS descriptor 크기에서의 측정값**이다.
이를 A에 그대로 넣으면 축소 장평을 중복 적용할 수 있다. 다음 연결에서 공유 font 설정 결과를 통해
측정 크기와 기준 크기를 명시적으로 환산하고, 측정 키와 실제 paint descriptor의 정합을 검사해야 한다.
장평 100%의 emoji 한 개만 통과하는 조건을 전체 서식 통과로 간주하지 않는다.

## 3. 실행 검증

위 source SHA와 같은 내용에서 실행했다.

```bash
cd /home/edward/mygithub/rhwp/rhwp-studio
node --test tests/supplemental-text-metrics.test.ts
npx tsc --noEmit
```

- 집중 테스트 **10 passed / 0 failed**, 128.22ms.
- 전체 프로젝트 TypeScript 타입 검사 성공.
- `git diff --check` 성공.

테스트는 통제 Canvas mock으로 독립적인 advance와 setter 치환/거부 동작을 제공한다.
실제 OS 폰트·브라우저·한컴 출력 일치의 증거가 아니다. 신규 WASM 빌드, browser 시각 판정,
전체 npm/Rust 회귀는 수행하지 않았다. 이 준비부를 아직 호출하지 않으므로 기존 렌더링 동작은 유지한다.

## 4. 다음 순서 — 같은 B 절편의 잔여 작업

1. 공유 font 설정 결과에 실제 descriptor·그리기 크기·자연 advance 기준 단위를 묶는다.
2. RendererSession 수명과 Rust 측정 문맥을 연결하고 backend별 cache 적용/복원을 검증한다.
3. 문서에서 실제 누락 요청을 수집하여 준비부 → A snapshot → 재조판 → 같은 paint로 연결한다.
   편집으로 추가된 glyph·서식, 폰트 교체, SVG/CanvasKit 내보내기도 포함한다.
4. Docker WASM으로 두 원본 HWP/HWPX를 다시 열어 압축·뒤 문자·장평을 확인하고 작업지시자 판정을 요청한다.

B의 수용 기준을 충족하지 않았으므로 C 완료나 PR 준비로 넘어가지 않는다.
이번 turn에서는 별도 이슈·브랜치를 만들지 않았고 원격 push·PR·GitHub 댓글도 하지 않았다.
