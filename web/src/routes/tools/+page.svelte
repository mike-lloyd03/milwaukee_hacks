<script lang="ts">
	import ToolCard from '$lib/components/ToolCard.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<div>
	<ul>
		{#each data.products as product}
			<li>
				<ToolCard {product} link="/tools/{product.item_id}" />
			</li>
		{/each}
	</ul>
</div>
