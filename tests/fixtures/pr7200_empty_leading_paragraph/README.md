# PR #7200 빈 선행 문단 경계

`samples/stored-nested-content-flow/cell-{top,center,bottom}-collapsed.hwpx`에서
바깥 셀의 첫 문단 `Start` 텍스트만 비운 합성 입력이다. 문단·저장 LineSeg·표·스타일은
그대로 유지했다. 원본 3개를 다시 이름만 바꿔 복제한 것이 아니라 빈 문단 경계를 추가한 입력이다.
한컴에서 실제 저장한 문서 또는 한컴 PDF와 일치하는 입력으로 주장하지 않는다.

ZIP의 `Contents/section0.xml`을 XML로 읽어 첫 `tbl/tr/tc/subList/p` 아래 첫 `t`의
text를 빈 문자열로 바꾼 후 재포장했다. XML namespace 접두어와 ZIP 압축은 재직렬화됐다.
나머지 ZIP member는 원본 바이트와 같다.

독립 기대값은 PR의 기존 Top/Center/Bottom 기하 계약이다. Top의 실제 중첩 표 하단과
부모 셀 하단 사이 여유 높이만큼 Bottom이, 절반만큼 Center가 이동해야 한다.
이는 한컴 외형 일치 주장이 아니라 동일 배치 엔진의 정렬 불변식이다.

```bash
python3 mydocs/pr/assets/pr7200_review/check_empty_leading_paragraph.py \
  --rhwp-bin /path/to/rhwp --trees /tmp/pr7200-empty-check
```

WASM tree는 `scripts/export-wasm-for-sweep.mjs`로 각 입력을
`<trees>/empty-first-<align>`에 출력한 뒤 위 명령에서 `--rhwp-bin`을 생략해 검사한다.
통과 시 exit 0, 정렬 회귀 시 exit 1이다.
