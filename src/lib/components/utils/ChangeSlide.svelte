<script lang="ts">
  import LeftArrow from "~icons/typcn/arrow-left";
  import RightArrow from "~icons/typcn/arrow-right";
	import { IconButton } from '../elements';
	import liveView from '$lib/stores/views/liveView';
	import activeScripture from '$lib/stores/active/scripture';
	import activeTemplate from '$lib/stores/active/template';
	import scriptures from '$lib/stores/lists/scriptures';
	import liveSong from '$lib/stores/live/song';
	import { getNextScripture, getPreviousScripture } from '$lib/utils/scripture';
	import { setLiveScriptureView } from '$lib/utils/mutations/scripture';
	import { setNextLiveSongSlide, setPrevLiveSongSlide } from '$lib/utils/song';

	const nextSlide = async () => {
		if ($liveView === 'Scripture') {
			const nextScripture = await getNextScripture($activeScripture, $scriptures);
			const text = new TextDecoder('utf-8').decode(
				new Uint8Array(nextScripture.verse_text as ArrayBuffer)
			);
			setLiveScriptureView({ scripture: nextScripture, text, templateId: $activeTemplate });
		} else if ($liveView === 'Song') {
			if ($liveSong) setNextLiveSongSlide($liveSong);
		}
	};

	const prevSlide = async () => {
		if ($liveView === 'Scripture') {
			const prevScripture = await getPreviousScripture($activeScripture, $scriptures);
			const text = new TextDecoder('utf-8').decode(
				new Uint8Array(prevScripture.verse_text as ArrayBuffer)
			);
			setLiveScriptureView({ scripture: prevScripture, text, templateId: $activeTemplate });
		} else if ($liveView === "Song") {
      if ($liveSong) setPrevLiveSongSlide($liveSong)
    }
	};
</script>

<div class="flex items-center gap-1">
	<IconButton class="flex flex-col gap-0.5 p-1 items-center" on:click={prevSlide} title="Previous Slide">
    <LeftArrow font-size={15}/>
    <span class="text-xs bg-black/20 rounded-sm px-2">Prev</span>
	</IconButton>
	<IconButton class="flex flex-col gap-0.5 p-1 items-center" on:click={nextSlide} title="Next Slide">
    <RightArrow font-size={15}/>
    <span class="text-xs bg-black/20 rounded-sm px-2">Next</span>
	</IconButton>
</div>
