<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { Editor } from '@tiptap/core';
	import ToolbarButton from './ToolbarButton.svelte';

	export let editor: Editor;
</script>

<div class="flex">
	<ToolbarButton
		class="rounded-r-none border-r-0"
		title="Undo"
		shortCut="Ctrl+z"
		disabled={!editor.can().undo()}
		on:click={() => editor.chain().focus().undo().run()}
	>
		<Icon class="text-lg" icon="iconamoon:do-undo-fill" />
	</ToolbarButton>
	<ToolbarButton
		class="rounded-l-none"
		title="Redo"
		shortCut="Ctrl+y"
		disabled={!editor.can().redo()}
		on:click={() => editor.chain().focus().redo().run()}
	>
		<Icon class="text-lg" icon="iconamoon:do-redo-fill" />
	</ToolbarButton>
</div>
