/**
 * en 카탈로그.
 *
 * 값 대응 표에서 만들어진 파일이지만, 여기서 값을 고쳐도 된다 — 키는 그대로 두고
 * 값만 바꾸면 화면에 그대로 반영된다. 키를 더하거나 지우는 일은 마크업의
 * data-i18n 표시·소스의 t() 호출과 함께 바꿔야 한다. 자세한 규칙은 ../README.md.
 */

const catalog = {
  "menu.file.label.x46b91c": "(No recent documents)",
  "ui.hfApplyTo.both": "Both pages",
  "ui.hfApplyTo.even": "Even pages",
  "ui.hfApplyTo.odd": "Odd pages",
  "ui.hfLiveStatus.editing": "Editing {p1} ({p2}), section {p3}, first page",
  "ui.hfLiveStatus.ended": "Finished editing header/footer",
  "ui.sbMessage.editMode": "Standard editing mode",
  "ui.sbMessage.filePages": "{p1} — Pages: {p2}",
  "ui.sbMessage.filePagesTimed": "{p1} — Pages: {p2} ({p3}ms)",
  "ui.sbMessage.formMode": "Form Mode",
  "ui.sbMessage.newDocPages": "새 문서.hwp — Pages: {p1}",
  "ui.sbMode.label": "Insert",
  "ui.sbMode.label.overwrite": "Overwrite",
  "ui.sbPage.text": "Page {p1} / {p2}",
  "ui.sbSection.text": "Section: {p1} / {p2}",
  "ui.tbHfLabel.editing": "Editing {p1} · {p2}",
  "ui.tbHfLabel.footer": "Footer",
  "ui.tbHfLabel.header": "Header",
} as const;

export default catalog;
