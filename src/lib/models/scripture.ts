export interface ScriptureOptions {
    version: string,
    book: string,
    chapter: number,
    limit?: number
}

export interface ScriptureResult {
  book: string
  canon_order: string
  chapter: string
  end_verse: string
  start_verse: string
  verse_id: string
  verse_text: unknown
}
