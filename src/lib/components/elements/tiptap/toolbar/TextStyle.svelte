<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { Input } from 'flowbite-svelte';
	import ToolbarButton from './ToolbarButton.svelte';
	import Icon from '@iconify/svelte';

	export let editor: Editor;
	let fontSize: number = parseInt(editor.getAttributes('textStyle').fontSize ?? 16);
	let color = editor.getAttributes('textStyle').color;
	let colorInput: HTMLInputElement;

	$: editor.chain().focus().setFontSize(`${fontSize}pt`).run();

	function handleColorChange(e: Event) {
		editor
			.chain()
			.focus()
			.setColor((e.target as HTMLInputElement).value)
			.run();
	}
</script>

<div class="flex h-full">
	<div class="flex">
		<Input
			on:keydown={(e) => console.log(e.key)}
			bind:value={fontSize}
			size="sm"
			class="p-[4px] w-[40px] h-full rounded rounded-r-none border-r-0"
			placeholder="16"
			type="number"
		/>
		<input
			bind:this={colorInput}
			bind:value={color}
			on:input={handleColorChange}
			class="p-[4px] min-h-full w-14 dark:bg-white/10 border border-black/10 dark:border-white/10 rounded rounded-l-none"
			type="color"
		/>
	</div>

	<ToolbarButton
		class="rounded-r-none ml-1 border-r-0"
		title="Bold"
		shortCut="Ctrl+b"
		active={editor.isActive('bold')}
		on:click={() => editor.chain().focus().toggleBold().run()}
	>
		<Icon class="text-lg" icon="clarity:bold-line" />
	</ToolbarButton>
	<ToolbarButton
		class="rounded-l-none rounded-r-none"
		title="Italic"
		shortCut="Ctrl+i"
		active={editor.isActive('italic')}
		on:click={() => editor.chain().focus().toggleItalic().run()}
	>
		<Icon class="text-lg" icon="clarity:italic-line" />
	</ToolbarButton>
	<ToolbarButton
		class="border-l-0 rounded-l-none"
		title="Underline"
		shortCut="Ctrl+u"
		active={editor.isActive('underline')}
		on:click={() => editor.chain().focus().toggleUnderline().run()}
	>
		<Icon class="text-lg" icon="clarity:underline-line" />
	</ToolbarButton>
</div>
