<script lang="ts">
	import Icon from '@iconify/svelte';
	import ToolbarButton from './ToolbarButton.svelte';
	import type { Editor } from '@tiptap/core';
	export let editor: Editor;
</script>

<div class="flex gap-0">
	<ToolbarButton
		title="Left"
		shortCut="Ctrl+Shift+l"
		active={editor.isActive({ textAlign: 'left' })}
		on:click={() => editor.chain().focus().setTextAlign('left').run()}
		class="rounded-r-none border-r-0"
	>
		<Icon class="text-lg" icon="prime:align-left" />
	</ToolbarButton>
	<ToolbarButton
		title="Center"
		shortCut="Ctrl+Shift+e"
		active={editor.isActive({ textAlign: 'center' })}
		on:click={() => editor.chain().focus().setTextAlign('center').run()}
		class="rounded-none border-r-0"
	>
		<Icon class="text-lg" icon="prime:align-center" />
	</ToolbarButton>
	<ToolbarButton
		title="Right"
		shortCut="Ctrl+Shift+r"
		active={editor.isActive({ textAlign: 'right' })}
		on:click={() => editor.chain().focus().setTextAlign('right').run()}
		class="rounded-none border-r-0"
	>
		<Icon class="text-lg" icon="prime:align-right" />
	</ToolbarButton>
	<ToolbarButton
		title="Justify"
		shortCut="Ctrl+Shift+j"
		active={editor.isActive({ textAlign: 'justify' })}
		on:click={() => editor.chain().focus().setTextAlign('justify')}
		class="rounded-l-none"
	>
		<Icon class="text-lg" icon="prime:align-justify" />
	</ToolbarButton>
</div>
