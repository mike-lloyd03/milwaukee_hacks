<script lang="ts">
	import PromoTitle from '$lib/components/PromoTitle.svelte';
	import type { Product, Promotion } from '$lib/types';
	import { Button, Card, Heading, Radio } from 'flowbite-svelte';
	import { bogo, type Cart } from '$lib/pkg/algorithm';
	import ResultsCard from '$lib/components/ResultsCard.svelte';
	import ItemScrollBox from '$lib/components/ItemScrollBox.svelte';
	import SearchInput from '$lib/components/SearchInput.svelte';
	import ExcludedProducts from '$lib/components/ExcludedProducts.svelte';
	import PromoItem from '$lib/components/PromoItem.svelte';
	import { simplifyName } from '$lib/utils';
	import { Fzf, type FzfResultItem } from 'fzf';

	interface Props {
		promo: Promotion;
		products: Product[];
		selected_product_ids?: string[];
	}
	let { promo, products, selected_product_ids }: Props = $props();

	let requiredProduct: string = $state('');
	let carts: Cart[] = $state([]);
	let srcProductsFilter = $state('');
	let tgtProductsFilter = $state('');
	let selectProductsMode = $state(true);
	let excludedProducts: string[] = $state([]);

	if (selected_product_ids) {
		requiredProduct =
			products.find((p) => p.item_id == selected_product_ids[0])?.product_label ?? '';
	}

	const srcProductIds = promo.eligibility_criteria.filter((ec) =>
		ec.item_group.startsWith('SRC')
	)[0].item_ids;
	const tgtProductIds = promo.eligibility_criteria.filter((ec) =>
		ec.item_group.startsWith('TGT')
	)[0].item_ids;

	let srcProducts = products.filter((p) => srcProductIds.includes(p.item_id));
	let tgtProducts = products.filter((p) => tgtProductIds.includes(p.item_id));

	const srcProductsFzf = new Fzf(srcProducts, {
		selector: (product) => simplifyName(product.product_label)
	});
	const srcResults = $derived(srcProductsFzf.find(srcProductsFilter));

	const tgtProductsFzf = new Fzf(tgtProducts, {
		selector: (product) => simplifyName(product.product_label)
	});
	const tgtResults = $derived(tgtProductsFzf.find(tgtProductsFilter));

	function calculate() {
		let srcProductsAlgo = srcProducts.map((p) => ({
			name: p.product_label,
			price: p.pricing.value
		}));
		let tgtProductsAlgo = tgtProducts.map((p) => ({
			name: p.product_label,
			price: p.pricing.value
		}));
		carts = [bogo(srcProductsAlgo, tgtProductsAlgo, requiredProduct)];
	}

	function rewardAmount() {
		if (carts.length > 0) {
			return carts[0].items[1].price;
		}
		return 0;
	}
</script>

{#snippet tool(result: FzfResultItem<Product>)}
	<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
		<label class="flex items-center py-2 text-sm font-medium">
			{#if selectProductsMode}
				<Radio
					name="selectedProduct"
					class="w-full"
					value={result.item.product_label}
					bind:group={requiredProduct}
					disabled={excludedProducts.includes(result.item.product_label)}
					checked={requiredProduct == result.item.item_id}
				>
					<PromoItem product={result.item} hlIndices={result.positions} />
				</Radio>
			{:else}
				<input
					class="peer me-2 h-4 w-4 appearance-none rounded border-gray-300 bg-gray-100 text-red-600 focus:ring-2 focus:ring-red-500 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800 dark:focus:ring-red-600"
					type="checkbox"
					name="excludedProduct"
					value={result.item.product_label}
					bind:group={excludedProducts}
					disabled={requiredProduct == result.item.product_label}
				/>
				<PromoItem product={result.item} hlIndices={result.positions} />
			{/if}
		</label>
	</div>
{/snippet}

<div class="space-y-4">
	<PromoTitle {promo} />

	<Card size="xl" class="mx-auto">
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-8">
				<div class="space-y-3">
					<Heading tag="h5">Select One of the Items Below</Heading>
					<SearchInput bind:value={srcProductsFilter} centered />
					<ItemScrollBox>
						{#each srcResults as result (result.item.item_id)}
							{#if result.item.product_label
								.toLowerCase()
								.includes(srcProductsFilter.toLowerCase())}
								{@render tool(result)}
							{/if}
						{/each}
					</ItemScrollBox>
				</div>

				<div class="space-y-3">
					<Heading tag="h5">Or Select One of These Items</Heading>
					<SearchInput bind:value={tgtProductsFilter} centered />
					<ItemScrollBox>
						{#each tgtResults as result (result.item.item_id)}
							{#if result.item.product_label
								.toLowerCase()
								.includes(tgtProductsFilter.toLowerCase())}
								{@render tool(result)}
							{/if}
						{/each}
					</ItemScrollBox>
				</div>

				<ExcludedProducts bind:selectProductsMode />
			</div>

			<div class="mx-auto flex max-w-md gap-2">
				<Button color="red" onclick={calculate}>Calculate</Button>
			</div>
		</div>
	</Card>

	<ResultsCard {carts} requiredProducts={[requiredProduct]} {rewardAmount} />
</div>
