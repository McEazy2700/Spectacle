<script lang="ts">
	export let direction: 'vertical' | 'horizontal';
	export let itemId: string;

	function handleResize(e: MouseEvent) {
		const before = document.getElementById(itemId);
		before?.style.setProperty('overflow', 'auto');
		let prevX = e.clientX;
		let prevY = e.clientY;

		function handleMouseMove(e: MouseEvent) {
			if (before) {
				let rect = before.getBoundingClientRect();
				before.style.height = rect.height - (prevY - e.clientY) + 'px';
			}
			prevY = e.clientY;
			prevX = e.clientX;
		}

		function handleMouseUp() {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', handleMouseUp);
		}

		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);
	}
</script>

<div
	role="button"
	tabindex="0"
	on:mousedown={handleResize}
	class={`w-full h-0.5 my-1 bg-black/20 dark:bg-white/20 hover:bg-blue-500 ${
		direction === 'vertical' ? 'cursor-ns-resize' : 'cursor-ew-resize'
	}`}
/>
