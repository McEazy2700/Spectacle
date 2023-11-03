// Scripture | Song | Sermon | Video | Audio
// Template

import type { FontWeight } from '$lib/types';

interface TemplateOptions {
	fontSize?: number;
	fontWeight?: FontWeight;
	font?: string;
	backgroundURL?: string;
	backgroundType?: 'Video' | 'Image';
}

export class Template {
	fontSize: number;
	fontWeight: FontWeight;
	font: string;
	backgroundURL?: string;
	backgroundType: 'Video' | 'Image';
	constructor(opts: TemplateOptions = {}) {
		this.fontSize = opts.fontSize ?? 12;
		this.fontWeight = opts.fontWeight ?? 'Normal';
		this.font = opts.font ?? 'Arial';
		this.backgroundType = opts.backgroundType ?? 'Image';
		this.backgroundURL = opts.backgroundURL;
	}
}

export class SermonView {
	content: string;
	texts: string[];
	template: Template;

	constructor(content: string, template: Template, texts?: string[]) {
		this.content = content;
		this.texts = texts ?? [];
		this.template = template;
	}
}

export class ScriptureView {
	book: string;
	chapter: number;
	verses: number[];
	version: string;
	template: Template;

	constructor(
		book: string,
		chapter: number,
		verses: number[],
		template: Template,
		version?: string
	) {
		this.book = book;
		this.chapter = chapter;
		this.verses = verses;
		this.version = version ?? 'KJV';
		this.template = template;
	}
}

type ViewData = ScriptureView | SermonView;
type ViewName = 'Sermon' | 'Scription';

/**
 * Contains attributes required for specifying what is presented
 * and methouds to perform actions on a presentation
 * @param data {ViewData} - this contains information used to render a presentation
 */
export class View {
	name: ViewName;
	data: ViewData;
	constructor(data: ViewData, name: ViewName) {
		this.name = name;
		this.data = data;
	}
}
