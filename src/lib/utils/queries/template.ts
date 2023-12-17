import type { TemplateView } from "$lib/models/slide";
import type { TemplateModel } from "$lib/models/template";
import alerts from "$lib/stores/alerts";
import { invoke } from "@tauri-apps/api";

export const getTemplates = async (
  templateId: number,
  callback: (template: TemplateModel | undefined) => void,
) => {
  try {
    const template = await invoke("get_template", {
      id: templateId,
    });
    callback(template as TemplateModel | undefined);
  } catch (err) {
    alerts.add({ message: `Error: ${JSON.stringify(err)}`, kind: "error" });
  }
};

export const getDefaultTemplate = async (
  view: TemplateView,
  callback: (templateId: number) => void,
) => {
  try {
    const res = await invoke("get_default_template", { view });
    const templateId = (res as { template_id: number }).template_id;
    callback(templateId);
  } catch (err) {
    alerts.add({
      message: `Error: ${(err as { message: string }).message}`,
      kind: "error",
    });
  }
};
