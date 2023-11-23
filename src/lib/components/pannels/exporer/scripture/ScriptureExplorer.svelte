<script>
	import { Label } from 'flowbite-svelte';
	import { BibleVersion } from '.';
</script>

<div class="flex items-center gap-4">
	<div class="flex gap-3 items-center">
		<div>
			<Label class="font-bold">Book</Label>
		</div>
		<div>
			<Label class="font-bold">Chapter</Label>
		</div>
		<div>
			<Label class="font-bold">Verse</Label>
		</div>
	</div>
  <BibleVersion />
</div>
