export type FontWeight = 'Light' | 'Normal' | 'SemiBold' | 'Bold' | 'ExtraBold';
export type TemplateType = {
  id?: number;
  name?: string;
  font?: string;
  font_size?: number;
  background_url?: string;
  font_weight?: FontWeight;
  background_type?: 'Image' | 'Video';
};

export type AlignmentType = "normal" | "start" | "center" | "end" | "safe" | "unset" | "unsafe" | "stretch" | "flex-start" | "flex-end" | "baseline" | "initial" | "inherit";
export type FontStyleType = "italic" | "oblique" | "normal" | "initial" | "inherit" | "unset"
