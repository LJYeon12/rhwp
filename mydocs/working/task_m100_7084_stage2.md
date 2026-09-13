# #7084 Stage 2 — A 절편 공통 보충 메트릭

- 일자: 2026-09-13
- 계획: [승인된 구현계획](../plans/task_m100_7084_impl.md)
- 상태: **A 구현 후 집중 검증 준비. Studio 활성화·시각 개선 완료 아님.**
- 통합 기준: `upstream/devel`의 `70bf40af2a2818e72bd58b4fa66e2d4c06de2b51`.
  계획 보존 `1f33a7059` 후 `c71cea458`에서 충돌 없이 통합했다. #7106 창 제목 변경은 보존했다.

## 구현한 규칙

지정 폰트 DB에서 폭을 찾지 못하여 일반 `heuristicHalfwidth`로 떨어진 독립 scalar에만,
현재 렌더링 세션의 보충 자연 advance를 사용한다. 장평·자간은 이후 기존 계산에서 한 번 적용한다.
DB hit·공백/탭·반각 형태·전각·좁은 구두점·합성 PUA의 의미 있는 폭 규칙을 덮어쓰지 않는다.

- `supplemental_metrics.rs`: 문서/폰트/backend 세대, 정확한 크기·family·bold/italic 키,
  backend 실측과 검증한 font bytes/cmap/hmtx 근거를 구분한다.
- 일괄 등록: 잘못된 세대, 중복 키, 비유한/음수 진행폭, 자원 상한 초과를 거부하며 이전 자료는 유지한다.
  동일 내용 재등록은 기존 snapshot을 유지한다. 다른 자료 등록·세대 변경·세션 폐기는 이전 참조도 무효화한다.
- 상한: 4,096항목, UTF-8 key 총량 4MiB, font 32MiB/개·64MiB/세션.
  동일 hash의 bytes는 snapshot 안에서 공유한다. 레이아웃 루프에서 font I/O나 parsing을 하지 않는다.
- `ResolvedStyleSet`에서 `TextStyle`로 읽기 참조를 전달한다. 직렬화에서는 제외하며 원본 Document를 변경하지 않는다.
- 전체 폭, 문자 위치, 비반올림 폭과 trace가 같은 `char_width_decision`을 소비한다.
  기존 snapshot 없는 경로에는 추가 grapheme 분할/할당이 없다.
- 여러 scalar로 이루어진 VS16/ZWJ/결합 문자 묶음은 전체 기존 경로를 유지한다. 부분 폭 교체를 하지 않는다.

## 검증 구분

집중 테스트는 통제 provider의 독립 입력값을 이용한 **계약 검사**다.
자연 폭을 임의의 정방형이나 Stage 1의 특정 Windows 측정값으로 고정하지 않는다.
실제 브라우저/한컴 일치 검증은 B·C 절편에서 수행한다.

- 초기 `cargo check --locked --lib --target-dir target/pr-review`: 성공(34.19초).
- 최종 source SHA의 집중 테스트·포맷 결과: 아래에 후속 기록한다.

## 아직 남은 연결과 보호 경계

현재 제품 세션에서는 보충 자료를 등록하지 않으므로 기본 출력은 아직 변경하지 않는다.
다음 B에서 fonts ready → 요청 수집/측정 → snapshot 등록/캐시 무효화 → 재조판 → 동일 paint를 연결한다.
검증한 font bytes의 존재만으로 backend가 그 font를 로드했다고 보지 않는다.
Canvas2D 실측값은 다른 backend에 등록할 수 없다. exact source의 paint capability 확인은 B에 남아 있다.
세대가 바뀌면 이전 참조는 적용되지 않지만, 이미 계산한 layout/paint 캐시의 폐기는 세션 소유자가 해야 한다.

font parsing/hash는 준비 단계의 API에서 수행한다. 반복 요청 중복 제거와 provider cache,
문서 스타일 주소·UTF-16 범위·paint descriptor 일치 검증, 세로쓰기 등 capability 제한은 B에서 연결한다.
이번 A의 단일 scalar 크기 키만으로 전체 문서의 cluster shaping 지원을 주장하지 않는다.
WASM/Studio 빌드 교체·전체 회귀·원격 push·PR·GitHub 댓글은 수행하지 않았다.
