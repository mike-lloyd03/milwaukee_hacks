<script lang="ts">
	import { Input } from 'flowbite-svelte';

	interface Props {
		value: string;
		centered?: boolean;
	}

	let { value = $bindable(), centered }: Props = $props();

	const classes = centered ? 'mx-auto w-full' : '';
</script>

<Input class="max-w-96 {classes}" bind:value placeholder="Search" />
