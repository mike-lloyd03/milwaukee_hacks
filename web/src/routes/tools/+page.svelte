<script lang="ts">
	import ToolCard from '$lib/components/ToolCard.svelte';
	import { Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import SearchInput from '$lib/components/SearchInput.svelte';
	import { Fzf } from 'fzf';
	import { simplifyName } from '$lib/utils';

	let { data }: { data: PageData } = $props();

	let productsFilter = $state('');

	const productsFzf = new Fzf(data.products, {
		selector: (item) => simplifyName(item.product_label)
	});
	const results = $derived(productsFzf.find(productsFilter));
</script>

<div class="space-y-2">
	<Heading tag="h2" class="mb-4">Tools</Heading>
	<P class="text-center">All tools with active promotions</P>

	<SearchInput bind:value={productsFilter} />

	<ul>
		{#each results as result (result.item.item_id)}
			<li>
				<ToolCard
					product={result.item}
					link="/tools/{result.item.item_id}"
					hlIndices={result.positions}
				/>
			</li>
		{/each}
	</ul>
</div>
