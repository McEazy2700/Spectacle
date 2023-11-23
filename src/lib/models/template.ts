export type FontWeight = "light" | "normal" | "semibold" | "bold" | "extrabold"
export type Alignment = "start" | "center" | "end"

export type Background = {
  type: "video" | "image",
  url: string
}

export interface TemptlateModel {
  font?: string
  fontSize: number
  fontWeight: FontWeight
  verticalAlignment: Alignment
  horizontalAlignment: Alignment
  background: Background
}

export interface MediaTemplateModel {
  objectFit: "cover" | "contain" | "fill"
}
