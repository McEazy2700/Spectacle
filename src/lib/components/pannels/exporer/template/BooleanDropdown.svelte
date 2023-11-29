<script lang="ts">

	import { Button, Dropdown, DropdownItem } from "flowbite-svelte";
	import { fade } from "svelte/transition";
  export let value: boolean = false
  export let open: boolean = false

  const options = [true, false]

  function clickHandlerFactory(option: boolean) {
    return function() {
      value = option
      open = false
    }
  }
</script>

<Button color="alternative" class="rounded w-fit" size="xs">{value}</Button>
<Dropdown bind:open transition={fade}>
  {#each options as option}
		<DropdownItem on:click={clickHandlerFactory(option)}>{option}</DropdownItem>
  {/each}
</Dropdown>
