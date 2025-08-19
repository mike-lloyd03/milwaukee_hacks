<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		maxH?: number;
		children: Snippet;
	}

	let { maxH = 96, children }: Props = $props();
</script>

<div
	class="flex flex-col overflow-y-auto rounded-md border border-gray-300 p-2 dark:border-gray-500"
	style="max-height: calc({maxH} * 0.25rem);"
>
	{@render children()}
</div>
