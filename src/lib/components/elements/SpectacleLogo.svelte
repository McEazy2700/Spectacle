<script lang="ts">
	import { Heading } from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';

	let klass = '';
	export { klass as class };
</script>

<Heading
	class={twMerge('self-center opacity-50 flex w-full justify-center items-center', klass)}
	tag="h1"
>
	<div class="w-fit relative">
		<img
			class="inline h-[80%] -bottom-4 -left-14 absolute"
			src="/spectacle_eye.svg"
			alt="spectacle_eye"
		/>
		<span class="italic">Spectacle</span>
	</div>
</Heading>
