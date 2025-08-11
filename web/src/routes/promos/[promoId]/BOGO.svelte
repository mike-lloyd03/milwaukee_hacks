<script lang="ts">
	import PromoTitle from '$lib/components/PromoTitle.svelte';
	import type { ProductDB, PromotionDB } from '$lib/dbTypes';
	import { formatCurrency } from '$lib/utils';
	import { Button, Card, Heading, Input, Radio } from 'flowbite-svelte';
	import { bogo, type Cart } from '$lib/pkg/algorithm';
	import ResultsCard from '$lib/components/ResultsCard.svelte';
	import ItemScrollBox from '$lib/components/ItemScrollBox.svelte';
	import SearchInput from '$lib/components/SearchInput.svelte';
	import ExcludedProducts from '$lib/components/ExcludedProducts.svelte';
	import PromoItem from '$lib/components/PromoItem.svelte';

	interface Props {
		promo: PromotionDB;
		products: ProductDB[];
	}
	let { promo, products }: Props = $props();

	let requiredProduct: string = $state('');
	let carts: Cart[] = $state([]);
	let srcProductsFilter = $state('');
	let tgtProductsFilter = $state('');
	let selectProductsMode = $state(true);
	let excludedProducts: string[] = $state([]);

	const srcProductIds = promo.eligibility_criteria.filter((ec) => ec.itemGroup.startsWith('SRC'))[0]
		.itemIds;
	const tgtProductIds = promo.eligibility_criteria.filter((ec) => ec.itemGroup.startsWith('TGT'))[0]
		.itemIds;

	let srcProducts = products.filter((p) => srcProductIds.includes(p.item_id));
	let tgtProducts = products.filter((p) => tgtProductIds.includes(p.item_id));

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

{#snippet tool(product: ProductDB)}
	<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
		<label class="flex items-center py-2 text-sm font-medium">
			{#if selectProductsMode}
				<Radio
					name="selectedProduct"
					class="w-full"
					value={product.product_label}
					bind:group={requiredProduct}
					disabled={excludedProducts.includes(product.product_label)}
				>
					<PromoItem {product} />
				</Radio>
			{:else}
				<input
					class="peer me-2 h-4 w-4 appearance-none rounded border-gray-300 bg-gray-100 text-red-600 focus:ring-2 focus:ring-red-500 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800 dark:focus:ring-red-600"
					type="checkbox"
					name="excludedProduct"
					value={product.product_label}
					bind:group={excludedProducts}
					disabled={requiredProduct == product.product_label}
				/>
				<PromoItem {product} />
			{/if}
		</label>
	</div>
{/snippet}

<div class="space-y-4">
	<PromoTitle {promo} />

	<Card size="xl" class="mx-auto">
		<div class="flex flex-col gap-4">
			<div class="flex flex-col gap-8">
				<div>
					<Heading tag="h5">Select One of the Items Below</Heading>
					<SearchInput bind:value={srcProductsFilter} />
					<ItemScrollBox>
						{#each srcProducts as product}
							{#if product.product_label.toLowerCase().includes(srcProductsFilter.toLowerCase())}
								{@render tool(product)}
							{/if}
						{/each}
					</ItemScrollBox>
				</div>

				<div>
					<Heading tag="h5">Or Select One of These Items</Heading>
					<SearchInput bind:value={tgtProductsFilter} />
					<ItemScrollBox>
						{#each tgtProducts as product}
							{#if product.product_label.toLowerCase().includes(tgtProductsFilter.toLowerCase())}
								{@render tool(product)}
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
