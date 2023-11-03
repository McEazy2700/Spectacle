interface FileSrcOptions {
  thumbnail?: boolean,
  time?: number
}

export function convertFileSrc(src: string, opts?: FileSrcOptions): string {
	if (!src.startsWith('http')) {
		return `http://127.0.0.1:2700/media/fetch?path=${src}${opts?.thumbnail ? "&thumbnail=true" : ""} ${opts?.time ? "&time=" + opts.time : ""}`;
	} else {
		return src;
	}
}
