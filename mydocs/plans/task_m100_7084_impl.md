# #7084 구현계획 — 누락 진행폭의 선택·측정·그리기 일치

- Issue: [#7084](https://github.com/edwardkim/rhwp/issues/7084)
- 작성일: 2026-09-13
- 상태: **2026-09-13 승인 후 A 절편 구현·집중 검증 완료. 다음은 B Studio 연결.**
- 수행계획: [승인된 R1](task_m100_7084.md)
- 원인 근거: [Stage 1](../working/task_m100_7084_stage1.md), 보존 commit `7e8b74cdd`.
- 제품 조사 기준: `ad6174255e6aabfaa48131f3f5f2f6287a93e48a`.
- fetch로 확인한 최신 devel: `70bf40af2a2818e72bd58b4fa66e2d4c06de2b51`.
  추가 #7106은 창 제목 변경이다. 문서 로드 연결부를 함께 보존하며 구현 전 최신 base를 통합한다.

## 1. 해결할 규칙

**지정 폰트의 문자 진행폭을 알 수 없어 일반 추정 폭으로 처리하는 경우, 사용 가능한 실제
그리기 소스의 측정 결과가 있으면 그 결과로 공간을 배정하고 같은 선택으로 그린다.**

현재 HWP/HWPX 모두 U+1F600에 500 HWPUNIT(10pt에서 약 6.67px)을 배정한다.
Studio의 실제 진행폭은 18.3027px이고 가로 배율은 0.364245, 세로는 1이다.
동일 입력의 한컴 PDF는 Segoe UI Emoji subset의 약 1.373em 진행폭을 사용하며 가로 압축이 없다.
이 수치는 원인 증거이지 새 제품 상수가 아니다.

수정은 `😀`·샘플 이름·셀 주소·특정 폰트 이름에 대한 조건문이 아니다. 또한 모든 보조 평면
문자를 전각으로 만들거나, 모든 DB miss를 이모지로 분류하지 않는다.
장평·자간·공백·탭·PUA 합성 등의 독립적인 한컴 규칙은 유지한다.

## 2. 확정한 구현 경계

### 2.1 정적 DB 수정부터 하지 않는다

이번 수정의 기본 경로는 **문서/폰트 세대에 종속된 보충 측정 snapshot**이다.
historical generated DB와 기존 overlay를 재작성하지 않는다. DB의 U+1F600 한 항목만 추가해도
지정 face가 함초롬바탕일 때 실제 대체 글리프를 선택하는 연결은 해결되지 않기 때문이다.

기존 DB와 명시적 조판 규칙을 먼저 적용한다. 그 뒤 **일반 추정 `heuristicHalfwidth`로
떨어지는 문자 묶음** 중 유효한 보충 측정이 확보된 범위만 새 결과를 사용한다.
`heuristicHalfwidthForm`, 공백, 좁은 구두점, 자모 조합, 옛한글/PUA 전개 등 의미가 있는
별도 규칙을 일반 추정과 혼동하지 않는다. exact source의 glyph가 있어도 DB에 폭이 없는
경우와 지정 source에 glyph가 없어 대체된 경우를 trace에서 구별한다.

이 규칙은 이모지 외 일반 미등록 glyph에도 적용될 수 있다. 변경 대상 목록을 계측하여
비이모지 반례와 함께 검증한다. 기존 정상 DB hit의 폭은 바꾸지 않는다.

### 2.2 선택 결과는 두 증거 수준으로 구분한다

| 상태 | 선택·측정의 근거 | 그리기 계약과 한계 |
| --- | --- | --- |
| `verifiedSource` | 검증한 font bytes hash·face index·glyph/cluster, 실제 hmtx 또는 기존 shaping 결과 | 동일 source로 그린다. source 불일치 시 적용 거부 |
| `backendMeasured` | 동일 backend·폰트 감지 세대·최종 CSS descriptor·cluster·크기에서 측정한 자연 진행폭 | 같은 descriptor와 세대에서만 사용. Canvas2D 암묵적 face는 `ambiguous`이며 exact로 표시하지 않음 |
| `unavailable` / `unsupported` | bytes/측정 실패, 모호한 capability, 지원 밖 cluster, 자원 상한 | 기존 결과와 진단 사유 유지. 수정 성공으로 집계하지 않음 |

이 이름은 계획상의 계약 이름이며 기존 타입 명명에 맞게 구현할 수 있다. 의미는 고정한다.
`backendMeasured`는 **브라우저가 선택한 실제 face 이름을 알아냈다는 계약이 아니다.**
동일한 CSS 폰트 선택 환경에서 측정과 paint가 같은 글자를 다룬다는 제한된 실행 계약이다.
다른 OS/backend에서 그대로 재사용하거나 native용 exact 데이터로 수출하지 않는다.

Canvas2D의 정상 기본 경로가 Local Font Access 추가 허가에 의존하지 않도록 `backendMeasured`를
지원한다. 이미 허가된 bytes가 있으면 `verifiedSource`를 우선한다. 자동 권한 부여·폰트 설치·
새 폰트 배포채널·OS 전체 글꼴 순회는 하지 않는다.

### 2.3 문서 모델 대신 렌더링 세션이 소유한다

원본 `Document`의 face·charShape·언어 slot·장평·자간을 변경하지 않는다.
`ExactFontSourceRegistry`의 slot 전체를 emoji font로 덮어쓰지도 않는다.

보충 snapshot은 문서/폰트 세대와 backend를 소유한 렌더링 문맥에 보관한다.
`ResolvedStyleSet` → `TextStyle`로 전달하는 읽기 전용 참조 또는 동등한 문맥 연결을 사용해,
기존 `char_width_decision`의 전체 폭·문자 위치·trace 소비자가 같은 결과를 조회하게 한다.
프로세스 전역 mutable 글자폭 표는 만들지 않는다. snapshot의 bytes·hash·generation을
HWP/HWPX 저장 데이터나 기본 스타일 직렬화에 섞지 않는다.

snapshot 항목에는 다음을 함께 둔다.

- 문서/폰트/backend 세대, 문서 스타일 식별자, 문자 묶음과 scalar/UTF-16 범위
- 선택 상태와 소스 식별자 또는 실제 paint CSS descriptor
- 실제 크기·굵기·기울임·방향 등 측정 설정, 자연 advance와 단위
- glyph 지원/조형 성공 여부, 실패 사유, 필요한 font resource 참조

이 구조의 새 필드는 영향 없는 기존 run의 serialized 출력에서 생략한다.
동일 스타일 안의 한글 run과 이모지 run을 구별하며, 논리 텍스트/편집 주소는 유지한다.

## 3. 자료 흐름과 배치 계약

`문서의 누락 메트릭 요청 수집 → 폰트 준비 후 세대별 보충 측정 → snapshot 원자적 등록`

`같은 snapshot → 전체 폭/줄 나눔/char positions → 같은 선택 정보와 positions로 paint`

1. 문서 로드·폰트 준비 연결부에서 필요한 요청만 수집한다. `loadWebFonts`와 fonts ready 이후
   최종 paint descriptor를 이용한다. presence probe용 원시 descriptor와 실제 그리기 descriptor를
   혼동하지 않는다. 측정 helper와 `WebCanvasRenderer`의 font 구성 규칙을 공유한다.
2. 등록은 일괄 검증 후 한 번에 반영한다. 문서가 바뀐 동안 도착한 비동기 결과는 버린다.
   기존 초기 layout이 먼저 계산됐다면 폰트 준비 뒤 대상 문맥을 무효화하고 한 번 재조판한다.
3. 편집으로 처음 등장한 문자 묶음·스타일은 미측정 요청으로 수집한다. paint 도중 문자마다
   비동기 폰트 탐색을 하지 않는다. 같은 세대의 반복 문자/기존 style은 cache로 재사용한다.
4. 자연 advance에 문서 장평·자간·첨자 배율을 기존 계약대로 **각각 한 번만** 적용한다.
   ink bounds를 advance로 사용하지 않으며 음수 자간의 기존 보호 규칙을 보존한다.
5. 해당 cluster의 paint는 snapshot과 일치하는 source/descriptor를 사용한다. 원래 문서 스타일은
   유지하되 그리기 선택을 명시한다. 의도된 장평은 적용하고, 잘못된 0.5em에 맞추는 추가 fit은
   제거한다. snapshot과 다른 폰트가 그려지면 fit으로 숨기지 않고 stale/unsupported로 재준비한다.
6. 유효 LineSeg의 문단·줄 소속을 무조건 지우지 않는다. 재조판이 필요한 편집 경로는 같은
   advance로 줄 나눔한다. 저장 줄의 폭/정렬 보정이 자연 glyph 비율을 다시 압축하는지도 검사한다.
7. backend/폰트 감지 세대/문서/style 변경 시 cache와 layout을 함께 무효화한다.
   실패 시 이전 세대의 폭과 새 세대의 glyph를 섞지 않는다.

### 3.1 cluster와 backend 범위

기존 `build_cluster_len`은 주로 한글 자모를 묶으며 완전한 emoji segmentation이 아니다.
새 보충 요청의 범위는 기존 의존성 `unicode-segmentation`을 사용해 문자 묶음 경계를 보존한다.
단독 emoji는 이번 수용 대상이다. VS16·ZWJ는 지원 경로에서 전체 묶음을 측정·그릴 수 있을 때만
적용하고, 그렇지 않으면 묶음 전체를 unsupported로 남긴다. selector/joiner별 폭을 더하거나
일부 scalar만 새 폭으로 바꾸는 혼합은 금지한다. 새 조형 엔진은 도입하지 않는다.

Canvas2D의 측정값을 CanvasKit/native/SVG에 그대로 전달하지 않는다.
bytes가 있는 경로는 기존 source identity·resource·positioned/shaping publication 계약을
재사용하되 기존 shaping 활성화 조건을 전역으로 풀지 않는다. 각 backend가 동일 source를
그릴 수 있는지 capability 확인 후 적용한다. 불가능한 경우 진단과 기존 출력을 유지하고
해당 환경을 수정 완료라고 보고하지 않는다. 기본 수용은 실제 결함이 재현된 Studio Canvas2D이며,
공통 Rust 변경으로 다른 backend가 회귀하지 않는 검증은 필수다.

## 4. 변경 예정 위치와 금지선

| 소유 영역 | 예정 작업 |
| --- | --- |
| `src/renderer/`의 새 소규모 보충 메트릭 모듈 | snapshot/요청/검증/cache key·소스 증거 수준 정의 |
| `layout/text_measurement.rs`, `style_resolver.rs`, `layout.rs`, `mod.rs` | 동일 보충 결과를 측정·스타일·run 전달에 연결; 기존 hit/특수 규칙 보존 |
| `document_core` 및 `wasm_api` | 읽기 전용 요청 조회와 렌더링 세션 snapshot 등록; 문서 IR/undo/저장 무변경 |
| `web_canvas.rs`, 필요 시 layer/render tree 및 SVG·Skia 경계 | 공통 positions와 선택 식별 전달, backend별 재사용 차단·capability 검사 |
| Studio `core/wasm-bridge.ts`, 폰트 준비 연결부 및 새 측정 helper | fonts ready/문서 세대와 일괄 측정 연결; #7106 이름 알림 보존 |
| `font_decision.rs` 및 Studio trace | 원래 DB miss와 새 advance 출처·미확정 face·거부 사유 표시 |
| `tests/cases/`, Studio tests/E2E | 아래 수정 전후 계약과 실물 관측 |

이 목록은 연결 소유 영역이며 모든 파일의 광범위 리팩토링 지시가 아니다.
`font_metrics_generated.rs`, 기존 overlay, generated font-rule projection은 직접 고치지 않는다.
유한 family mapping 추가가 필요해지면 v2 append-only change set 절차와 evidence delta를
별도 계획 보완으로 제시한다. 이번 기본안은 새 family mapping을 추가하지 않는다.

## 5. 구현 절편 — 세 번의 검증 가능한 인계

| 순서 | 구현/검증 | 통과 기준 |
| --- | --- | --- |
| A. 공통 결정 자료 | 최신 devel 통합; snapshot·세대·단위·허용 상태·읽기 전용 측정 연결; 집중 회귀 | fake provider/통제 source의 독립 메트릭과 positions 일치, 잘못된 등록 시 무변경, 기존 DB hit 보존 |
| B. Studio 연결 | 폰트 준비→snapshot→재조판→동일 paint 연결; 보존 두 파일과 주변 텍스트 확인 | 실제 기본 Canvas2D에서 압축 해소, 뒤 문자/줄 나눔·문서 장평 유지; SVG 등 미지원 경로 명시 |
| C. 통합 검증 | Docker WASM·Studio 재판정, 승인된 전체 회귀/lint/Native Skia/Render Diff·비용 | 동일 최종 source SHA의 결과와 작업지시자 시각 판정, 잔여 제한 공개 |

A의 계약이 연결 불가능하거나 B에 새 폰트 배포/전체 shaping 전환이 필요하면 코드를 늘리기 전에
근거와 수정계획을 제시한다. 절편 A만으로 이슈 완료를 선언하지 않는다.

## 6. 검증과 비용

- **RED→GREEN**: 0.5em 강제 폭과 실제 paint 축소를 기존 두 입력에서 검출한다. 수정 후
  독립 한컴 PDF의 자연 비율·주변 배치를 비교한다. 18.3027px를 모든 환경의 golden으로 쓰지 않는다.
- **일관성**: estimate 전체 폭, 문자 위치 마지막 경계, 줄 나눔, paint의 다음 시작점이 같은
  보충 결과를 사용한다. 뒤에 한글/영문이 있는 줄 및 가용 폭 경계를 검사한다.
- **적용 경계**: 지정 source의 DB 미등록 glyph / 실제 fallback glyph / 기존 DB hit / source 부재,
  일반 기호와 비이모지 보조 평면 문자, 공백·PUA·탭의 명시 규칙을 구분한다.
- **서식**: 장평 100%/비100%, 자간 0/양수/음수, 굵게·기울임·첨자를 검사한다.
  자연 그림 비율과 문서가 의도한 가로 변경을 구별한다.
- **수명**: 반복 load, 새 문서 전환 중 늦은 응답, 폰트 교체, backend 전환, 편집에서 새 문자 추가,
  동일 값 재등록, 실패 등록·undo/redo·저장/재열기의 문서 무변경을 검사한다.
- **자원**: 등록 경계에서 NaN/Infinity/음수 advance·잘못된 범위·세대·소스 식별을 거부한다.
  0 advance 자체는 합법적인 결합문자도 있으므로 일괄 오류로 취급하지 않는다.
  기존 32MiB/font·4,096 codepoint/glyph 등 관련 상한을 넘기지 않는다. 새 요청/cache는
  초기 상한 4,096 고유 항목·4MiB UTF-8 key 총량으로 제한하고 초과 사유를 남긴다.
  초과를 성공으로 보고하거나 반복 전체 스캔으로 우회하지 않는다.
- **비용**: 같은 입력과 폰트 상태에서 cold/warm 준비·layout·paint 시간, 요청/측정/cache hit 수,
  WASM 크기·cache 메모리를 비교한다. paint/hot path의 font I/O·전수 열거는 0이어야 한다.
  반복 요청의 추가 측정은 0, 세대 전환은 필요한 항목만 재측정한다. 성능 증가분은 수치로 제출한다.
- **정식 게이트**: [local_validation 4.3](../manual/pr_review/local_validation.md#43-변경-범위별-기본-검증)의
  Rust 세 Clippy·workspace build·fmt, release-test 전체, Native Skia 3종, Docker WASM,
  Studio TypeScript/npm test/실브라우저, Render Diff를 변경 범위에 따라 수행한다.
  전체 긴 검증은 집중 결과 보고 후 승인 게이트를 따른다. 새 Rust 회귀는 `tests/cases/`에 두고
  generated suite/manifest는 review 검증에서만 준비하며 PR에 넣지 않는다.

기준값 완화나 무근거 golden 갱신으로 실패를 숨기지 않는다. 원본 두 파일·한컴 PDF는 재사용하고,
보존용 PDF 및 최소 재현 증적은 제출 단계에서 프로젝트 경로/manifest 규칙에 따라 보존한다.
MCP 접속정보와 로컬 상용 폰트 bytes는 Git에 넣지 않는다.

## 7. 승인 요청

**위 공통 보충 측정 계약과 A→B→C 절편 구현을 승인 요청한다.**
특히 Canvas2D 기본 동작은 실제 face 이름을 확정할 수 없을 때도 동일 backend/descriptor/세대의
실측을 사용할 수 있도록 하되, exact source 계약과 구별하는 설계다. 모든 backend·모든 폰트 부재
환경의 동일 출력이나 전체 emoji 조형 지원을 이번 완료 범위로 확대하지 않는다.

위 승인 요청은 계획 작성 시점의 기록이다. 승인 후 A 절편을 구현했으며,
제품/테스트 `cccc33a36`에서 집중 15건·포맷·native Clippy를 통과했다.
구현 범위·명령·첫 검사 오류 정정·미검증 경계는 [Stage 2](../working/task_m100_7084_stage2.md)에 기록했다.
B의 실제 Studio 측정/paint 연결과 C의 통합 게이트는 아직 수행하지 않았다.
원격 push·PR·GitHub 댓글은 수행하지 않았다.

## 용어

- snapshot: 한 문서/폰트 세대에서 고정한 측정 자료. 저장 문서가 아니라 렌더링용 임시 자료다.
- descriptor: Canvas가 해석하는 최종 font 설정 문자열 및 관련 그리기 설정.
- generation(세대): 문서·폰트·backend 변경 전후의 자료를 섞지 않기 위한 식별 값.
- advance: 다음 문자 시작까지의 진행폭. ink bounds(실제 그림 경계)와 구별한다.
- exact / ambiguous: 실제 폰트 자원까지 검증한 상태 / 그리기 후보는 알지만 실제 face는 미확정인 상태.
- scalar / UTF-16: Unicode 문자 코드 포인트 / 저장·API에서 쓰는 16비트 코드 단위 체계.
- SFNT / hmtx: 폰트 테이블 컨테이너 / 글리프의 가로 진행폭과 여백을 담는 테이블.
