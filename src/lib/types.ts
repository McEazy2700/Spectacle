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
