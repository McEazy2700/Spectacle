<script lang="ts">
  import AlignLeft from "~icons/prime/align-left";
  import AlignRight from "~icons/prime/align-right"
  import AlignCenter from "~icons/prime/align-center"
  import AlignJustify from "~icons/prime/align-justify"
	import type { Editor } from '@tiptap/core';
	import { ToolbarButton } from "../../buttons";
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
    <AlignLeft font-size={15}/>
	</ToolbarButton>
	<ToolbarButton
		title="Center"
		shortCut="Ctrl+Shift+e"
		active={editor.isActive({ textAlign: 'center' })}
		on:click={() => editor.chain().focus().setTextAlign('center').run()}
		class="rounded-none border-r-0"
	>
    <AlignCenter font-size={15}/>
	</ToolbarButton>
	<ToolbarButton
		title="Right"
		shortCut="Ctrl+Shift+r"
		active={editor.isActive({ textAlign: 'right' })}
		on:click={() => editor.chain().focus().setTextAlign('right').run()}
		class="rounded-none border-r-0"
	>
    <AlignRight font-size={15}/>
	</ToolbarButton>
	<ToolbarButton
		title="Justify"
		shortCut="Ctrl+Shift+j"
		active={editor.isActive({ textAlign: 'justify' })}
		on:click={() => editor.chain().focus().setTextAlign('justify')}
		class="rounded-l-none"
	>
    <AlignJustify font-size={15}/>
	</ToolbarButton>
</div>
