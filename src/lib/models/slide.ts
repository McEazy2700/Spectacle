export type TemplateView = "Song" | "Text" | "Media" | "Sermon" | "Scripture";

export interface MediaSlide {
  Media: {
    id: string
    type: "Video" | "Image"
    src: string
    template_id: number
  }
}

export interface ScriptureSlide {
  Scripture: {
    id: string
    type: "Scripture"
    text: string
    passage: string
    template_id: number
  }
}

export interface SermonSlide {
  Sermon: {
    id: string
    type: "Sermon"
    text: string
    template_id: number
  }
}

export interface SongSlide {
  Song: {
    id: string
    type: "Song"
    text: string
    verse: number
    template_id: number
  }
}

export interface TextSlide {
  Text: {
    id: string
    type: "Text"
    text: string
    template_id: number
  }
}

export type SlideType = MediaSlide | SermonSlide | ScriptureSlide | SongSlide | TextSlide

export interface View {
  id: string
  name: string
  type: TemplateView
  slide: SlideType
}

export type ScheduleType = View[]
