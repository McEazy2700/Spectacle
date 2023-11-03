<script>
	import { twMerge } from 'tailwind-merge';
	let klass = '';
	export { klass as class };
</script>

<div class={twMerge('w-screen h-screen', klass)}>
	<slot />
</div>
