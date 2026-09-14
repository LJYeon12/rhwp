/**
 * ko 카탈로그.
 *
 * 값 대응 표에서 만들어진 파일이지만, 여기서 값을 고쳐도 된다 — 키는 그대로 두고
 * 값만 바꾸면 화면에 그대로 반영된다. 키를 더하거나 지우는 일은 마크업의
 * data-i18n 표시·소스의 t() 호출과 함께 바꿔야 한다. 자세한 규칙은 ../README.md.
 */

const catalog = {
  "menu.file.label.x46b91c": "(최근 문서 없음)",
  "ui.hfApplyTo.both": "양쪽",
  "ui.hfApplyTo.even": "짝수 쪽",
  "ui.hfApplyTo.odd": "홀수 쪽",
  "ui.hfLiveStatus.editing": "{p1} {p2} 편집 중, 구역 {p3} 첫 페이지",
  "ui.hfLiveStatus.ended": "머리말 꼬리말 편집 종료",
  "ui.sbMessage.editMode": "기본 편집 모드",
  "ui.sbMessage.filePages": "{p1} — {p2}페이지",
  "ui.sbMessage.filePagesTimed": "{p1} — {p2}페이지 ({p3}ms)",
  "ui.sbMessage.formMode": "양식 모드",
  "ui.sbMessage.newDocPages": "새 문서.hwp — {p1}페이지",
  "ui.sbMode.label": "삽입",
  "ui.sbMode.label.overwrite": "수정",
  "ui.sbPage.text": "{p1} / {p2} 쪽",
  "ui.sbSection.text": "구역: {p1} / {p2}",
  "ui.tbHfLabel.editing": "{p1} · {p2} 편집 중",
  "ui.tbHfLabel.footer": "꼬리말",
  "ui.tbHfLabel.header": "머리말",
} as const;

export default catalog;
