<script lang="ts">
	import { bmsm, Cart } from '$lib/pkg/algorithm';
	import type { Product } from '$lib/types';
	import { Card, Heading, Button, Modal } from 'flowbite-svelte';
	import { onMount, type Snippet } from 'svelte';
	import SearchInput from './SearchInput.svelte';
	import ItemScrollBox from './ItemScrollBox.svelte';
	import ExcludedProducts from './ExcludedProducts.svelte';
	import PromoItem from './PromoItem.svelte';

	interface Props {
		products: Product[];
		requiredProducts: string[];
		carts: Cart[];
		minCartSize: number;
		maxCartSize: number;
		minCartTotal?: number;
		options?: Snippet;
		selected_product_ids?: string[];
	}

	let {
		products,
		requiredProducts = $bindable(),
		carts = $bindable(),
		minCartSize,
		maxCartSize = $bindable(),
		minCartTotal = $bindable(),
		options,
		selected_product_ids
	}: Props = $props();

	let productsFilter = $state('');
	let cartSize = $state(3);
	let selectedProducts: string[] = $state([]);
	let excludedProducts: string[] = $state([]);
	let optionsOpen = $state(false);
	let selectProductsMode = $state(true);

	onMount(() => {
		if (selected_product_ids) {
			selectedProducts = products
				.filter((p) => selected_product_ids.includes(p.item_id))
				.map((p) => p.product_label);
		}
	});

	function rowDisabled(productLabel: string): boolean {
		return (
			(!selectedProducts.includes(productLabel) && selectedProducts.length >= cartSize) ||
			excludedProducts.includes(productLabel)
		);
	}

	function getCarts(event: MouseEvent) {
		event.preventDefault();
		requiredProducts = selectedProducts;
		const productData = products
			.filter((p) => !excludedProducts.includes(p.product_label))
			.map((p: Product) => {
				return { name: p.product_label, price: p.pricing.value };
			});

		const allCarts = bmsm(
			productData,
			minCartSize,
			maxCartSize,
			minCartTotal ?? 0,
			requiredProducts
		);
		carts = allCarts.slice(0, 10);
	}
</script>

<Card size="xl" class="mx-auto">
	<Heading tag="h5">Select Products</Heading>
	<div class="my-4 flex flex-col gap-4">
		<div>
			<SearchInput bind:value={productsFilter} />
			<ItemScrollBox>
				{#each products as product (product.product_label)}
					{#if product.product_label.toLowerCase().includes(productsFilter.toLowerCase())}
						<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
							<label
								class="flex items-center py-2 text-sm font-medium {rowDisabled(
									product.product_label
								)
									? 'text-gray-500 dark:text-gray-600'
									: ''} "
							>
								{#if selectProductsMode}
									<input
										class="text-primary-600 focus:ring-primary-500 dark:focus:ring-primary-600 me-2 h-4 w-4 rounded border-gray-300 bg-gray-100 focus:ring-2 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800"
										type="checkbox"
										name="requiredProduct"
										value={product.product_label}
										bind:group={selectedProducts}
										disabled={rowDisabled(product.product_label) ||
											excludedProducts.includes(product.product_label)}
										checked={selectedProducts.includes(product.item_id)}
									/>
								{:else}
									<input
										class="peer me-2 h-4 w-4 appearance-none rounded border-gray-300 bg-gray-100 text-red-600 focus:ring-2 focus:ring-red-500 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800 dark:focus:ring-red-600"
										type="checkbox"
										name="excludedProduct"
										value={product.product_label}
										bind:group={excludedProducts}
										disabled={selectedProducts.includes(product.product_label)}
									/>
								{/if}

								<PromoItem {product} />
							</label>
						</div>
					{/if}
				{/each}
			</ItemScrollBox>

			<ExcludedProducts bind:selectProductsMode />
		</div>

		<div class="mx-auto flex max-w-md gap-2">
			{#if options}
				<Button color="alternative" onclick={() => (optionsOpen = true)}>Options</Button>
			{/if}
			<Button color="red" onclick={getCarts}>Calculate</Button>
		</div>
	</div>
</Card>

{#if options}
	<Modal title="Options" bind:open={optionsOpen} autoclose>
		{@render options()}
		<Button color="alternative">Okay</Button>
	</Modal>
{/if}
