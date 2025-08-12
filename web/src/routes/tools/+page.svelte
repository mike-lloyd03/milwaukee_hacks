<script lang="ts">
	import ToolCard from '$lib/components/ToolCard.svelte';
	import { Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import SearchInput from '$lib/components/SearchInput.svelte';

	let { data }: { data: PageData } = $props();

	let productsFilter = $state('');
</script>

<div class="space-y-2">
	<Heading tag="h2" class="mb-4">Tools</Heading>
	<P class="text-center">All tools with active promotions</P>

	<SearchInput bind:value={productsFilter} />

	<ul>
		{#each data.products as product (product.item_id)}
			{#if product.product_label.toLowerCase().includes(productsFilter.toLowerCase())}
				<li>
					<ToolCard {product} link="/tools/{product.item_id}" />
				</li>
			{/if}
		{/each}
	</ul>
</div>
