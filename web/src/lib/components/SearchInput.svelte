<script lang="ts">
	import { Input } from 'flowbite-svelte';

	interface Props {
		value: string;
	}

	let { value = $bindable() }: Props = $props();
</script>

<Input class="mx-auto mb-4 max-w-96" bind:value placeholder="Search" />
