<script lang="ts">
	import ToolCard from '$lib/components/ToolCard.svelte';
	import { Input } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import ItemScrollBox from '$lib/components/ItemScrollBox.svelte';

	let { data }: { data: PageData } = $props();

	let productsFilter = $state('');
</script>

<div>
	<Input class="mx-auto mb-4 max-w-96" bind:value={productsFilter} placeholder="Search" />
	<ul>
		<ItemScrollBox maxH={200}>
			{#each data.products as product}
				{#if product.product_label.toLowerCase().includes(productsFilter.toLowerCase())}
					<li>
						<ToolCard {product} link="/tools/{product.item_id}" />
					</li>
				{/if}
			{/each}
		</ItemScrollBox>
	</ul>
</div>
