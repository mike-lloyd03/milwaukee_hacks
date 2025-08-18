<script lang="ts">
	import { indexSetToCharGroup } from '$lib/utils';

	interface Props {
		str: string;
		indices?: Set<number>;
	}

	let { str, indices }: Props = $props();

	let charMap = $derived(indexSetToCharGroup(str, indices));
</script>

<span>
	{#each charMap as m, i (i)}
		{#if m.bold}
			<span class="? true : false text-red-600 dark:text-red-500">{m.chars}</span>
		{:else}
			<span>{m.chars}</span>
		{/if}
	{/each}
</span>
