<script lang="ts">
	import PromoTitle from '$lib/components/PromoTitle.svelte';
	import type { ProductDB, PromotionDB } from '$lib/dbTypes';
	import { formatCurrency } from '$lib/utils';
	import { Button, Card, Heading, Radio } from 'flowbite-svelte';

	interface Props {
		promo: PromotionDB;
		products: ProductDB[];
	}
	let { promo, products }: Props = $props();

	const srcProductIds = promo.eligibility_criteria.filter((ec) => ec.itemGroup.startsWith('SRC'))[0]
		.itemIds;
	const tgtProductIds = promo.eligibility_criteria.filter((ec) => ec.itemGroup.startsWith('TGT'))[0]
		.itemIds;

	let srcProducts = products.filter((p) => srcProductIds.includes(p.item_id));
	let tgtProducts = products.filter((p) => tgtProductIds.includes(p.item_id));
</script>

{#snippet tool(product: ProductDB)}
	<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
		<label class="flex items-center py-2 text-sm font-medium">
			<Radio name="selectedProduct" class="w-full">
				<div class="flex w-full items-center justify-between">
					<div class="flex items-center">
						<img
							src={product.image_primary_url.replace('<SIZE>', '65')}
							alt="tool"
							class="rounded-md"
						/>
						<a
							href={`https://www.homedepot.com${product.canonical_url}`}
							class="mx-2 hover:underline"
							target="_blank">{product.product_label}</a
						>
					</div>
					{formatCurrency(product.pricing_value)}
				</div>
			</Radio>
		</label>
	</div>
{/snippet}

<div class="space-y-4">
	<PromoTitle {promo} />
	<Card size="xl" class="mx-auto">
		<div class="flex flex-col gap-8">
			<div>
				<Heading tag="h5">Select One of the Items Below</Heading>
				<div>
					{#each srcProducts as product (product.product_label)}
						{@render tool(product)}
					{/each}
				</div>
			</div>

			<div>
				<Heading tag="h5">Or Select One of These Items</Heading>
				<div>
					{#each tgtProducts as product (product.product_label)}
						{@render tool(product)}
					{/each}
				</div>
			</div>
		</div>

		<div class="mx-auto flex max-w-md gap-2">
			<Button color="red">Calculate</Button>
		</div>
	</Card>
</div>

<pre class="text-left">
    {JSON.stringify(promo, null, 4)}
</pre>
