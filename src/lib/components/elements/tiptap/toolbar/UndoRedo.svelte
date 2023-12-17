<script lang="ts">
  import Undo from "~icons/iconamoon/do-undo-fill";
  import Redo from "~icons/iconamoon/do-redo-fill";
	import type { Editor } from '@tiptap/core';
	import { ToolbarButton } from "../../buttons";

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
    <Undo font-size={15}/>
	</ToolbarButton>
	<ToolbarButton
		class="rounded-l-none"
		title="Redo"
		shortCut="Ctrl+y"
		disabled={!editor.can().redo()}
		on:click={() => editor.chain().focus().redo().run()}
	>
    <Redo font-size={15}/>
	</ToolbarButton>
</div>
