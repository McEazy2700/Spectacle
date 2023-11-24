<script lang="ts">
	import { AlertBox, OpenPannels, Sidebar, Toolbar } from '$lib/components';
</script>

<div class="h-screen max-h-screen overflow-hidden overflow-y-auto">
	<Toolbar />
	<div class="p-1 h-full">
		<OpenPannels />
	</div>
  <Sidebar />
  <AlertBox />
</div>
