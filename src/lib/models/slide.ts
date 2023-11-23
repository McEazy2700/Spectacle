import type { TemptlateModel as TemplateModel, MediaTemplateModel } from "./template"

export interface MediaSlide {
  id: number
  type: "Video" | "Image"
  src: string
  template: MediaTemplateModel
}

export interface ScriptureSlide {
  id: number
  type: "Scripture"
  text: string
  passage: string
  template: TemplateModel
}

export interface SermonSlide {
  id: number
  type: "Sermon"
  text: string
  alert?: {
    show: boolean
    title: string
    content: string
  }
  template: TemplateModel
}

export interface SongSlide {
  id: number
  type: "Song"
  text: string
  verse: number
  template: TemplateModel
}

export interface TextSlide {
  id: number
  type: "Text"
  text: string
  template: TemplateModel
}

export type SlideType = MediaSlide | SermonSlide | ScriptureSlide | SongSlide | TextSlide

export interface View {
  id: number
  name: string
  next?: number
  prev?: number
  slides: SlideType[]
}

export type ScheduleType = View[]
