<script lang="ts">
	import ToolCard from '$lib/components/ToolCard.svelte';
	import { Button, Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import SearchInput from '$lib/components/SearchInput.svelte';
	import { Fzf } from 'fzf';
	import { simplifyName } from '$lib/utils';

	let { data }: { data: PageData } = $props();

	let productsFilter = $state('');
	const pageSize = 25;
	let page = $state(0);

	const productsFzf = new Fzf(data.products, {
		selector: (product) => simplifyName(product.product_label)
	});
	const results = $derived(productsFzf.find(productsFilter));

	function nextPage() {
		if (page + 1 > results.length / pageSize) {
			return;
		}
		page++;
	}

	function prevPage() {
		if (page - 1 == -1) {
			return;
		}
		page--;
	}
	$effect(() => {
		if (productsFilter) {
			page = 0;
		}
	});
</script>

<div class="space-y-2">
	<Heading tag="h2" class="mb-4">Tools</Heading>
	<P class="text-center">All tools with active promotions ({results.length})</P>

	<div class="flex justify-between">
		<SearchInput bind:value={productsFilter} />
		<div class="flex items-center justify-center gap-4">
			<Button onclick={prevPage} color="light">Prev</Button>
			<p>{page + 1}/{Math.ceil(results.length / pageSize)}</p>
			<Button onclick={nextPage} color="light">Next</Button>
		</div>
	</div>

	<ul>
		{#each results.slice(page * pageSize, (page + 1) * pageSize) as result (result.item.item_id)}
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
