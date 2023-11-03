<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { UndoRedo, TextAlign, TextStyle } from '.';

	export let editor: Editor;
</script>

<div class="flex items-center flex-wrap gap-2">
	<UndoRedo bind:editor />
	<TextAlign bind:editor />
	<TextStyle bind:editor />
</div>
