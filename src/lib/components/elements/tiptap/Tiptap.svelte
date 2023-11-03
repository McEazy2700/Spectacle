<script lang="ts">
	import StarterKit from '@tiptap/starter-kit';
	import TextAlign from '@tiptap/extension-text-align';
	import TextStyle from '@tiptap/extension-text-style';
	import Color from '@tiptap/extension-color';
	import FontSize from 'tiptap-extension-font-size';
	import Underline from '@tiptap/extension-underline';
	import { Editor } from '@tiptap/core';
	import { TiptapToolbar } from '.';
	import { onDestroy, onMount } from 'svelte';

	export let content = '';

	let element: HTMLDivElement | undefined = undefined;
	let editor: Editor | undefined = undefined;

	onMount(() => {
		editor = new Editor({
			element,
			extensions: [
				StarterKit,
				TextStyle,
				FontSize,
				Color,
				Underline,
				TextAlign.configure({
					types: ['heading', 'paragraph'],
					alignments: ['left', 'center', 'right', 'justify']
				})
			],
			content,
			onTransaction: () => {
				editor = editor;
			}
		});
	});
	$: content = editor?.getHTML() ?? content;

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

<div>
	{#if editor}
		<TiptapToolbar bind:editor />
	{/if}
	<div
		class={`
    relative focus-within:shadow-xl transition-all text-black mt-2
    dark:text-white border rounded-md border-black/10 dark:border-white/10
  `}
	>
		{#if content === '<p></p>'}
			<p class="absolute top-2 left-2.5 opacity-30">Type Something</p>
		{/if}
		<div class="p-2" bind:this={element} />
	</div>
</div>
