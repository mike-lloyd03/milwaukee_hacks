<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { Label, NumberInput, Tooltip } from 'flowbite-svelte';
	import { simplifyName } from '$lib/utils';

	import OptionsCard from '$lib/components/OptionsCard.svelte';
	import ResultsCard from '$lib/components/ResultsCard.svelte';
	import PromoTitle from '$lib/components/PromoTitle.svelte';
	import { onMount } from 'svelte';
	import type { ProductDB, PromotionDB } from '$lib/dbTypes';

	interface Props {
		promo: PromotionDB;
		products: ProductDB[];
		selected_product_ids?: string[];
	}

	let { promo, products, selected_product_ids }: Props = $props();

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let minCartSize: number = $state(0);
	let maxCartSize: number = $state(0);
	let minCartTotal: number = $state(0);

	let productsSimpleName = products.map((p) => {
		p.product_label = simplifyName(p.product_label);
		return p;
	});

	onMount(() => {
		let maxCart = 0;
		let minTotal = 0;

		if (promo.reward_tiers) {
			const first_tier = promo.reward_tiers[0];

			if (first_tier.min_purchase_quantity && first_tier.max_purchase_quantity) {
				// Item quantity based promo
				// TODO: There is a bug here where the min and max purchase quantity can be higher than we want our BMSM algorithm running for
				promo.reward_tiers.forEach((tier) => {
					if (tier.min_purchase_quantity) {
						if (tier.min_purchase_quantity > maxCart) {
							maxCart = tier.min_purchase_quantity;
						}
					}
				});

				minCartSize = maxCart;
				maxCartSize = maxCart;
				minCartTotal = minTotal;
			} else if (first_tier.min_purchase_amount && first_tier.max_purchase_amount) {
				// Cart total based promo
				promo.reward_tiers.forEach((tier) => {
					if (tier.min_purchase_amount) {
						if (tier.min_purchase_amount > minTotal) {
							minTotal = tier.min_purchase_amount;
						}
					}
				});

				minCartSize = 2;
				maxCartSize = 5;
				minCartTotal = minTotal;
			}
		}
	});

	function rewardAmount(cart: Cart): number {
		const cartSize = cart.items.length;
		let rewards: number[] = [];

		if (promo.reward_tiers) {
			for (let tier of promo.reward_tiers) {
				const maxQty = tier.max_purchase_quantity;
				const minQty = tier.min_purchase_quantity;
				const maxAmt = tier.max_purchase_amount;
				const minAmt = tier.min_purchase_amount;

				const reward =
					tier.reward_amount_per_order ?? (cart.total * (tier.reward_percent ?? 0)) / 100;

				if (minQty && maxQty) {
					if (maxQty == -1 && cartSize >= minQty) {
						rewards.push(reward);
						continue;
					}
					if (cartSize >= minQty && cartSize < maxQty) {
						rewards.push(reward);
						continue;
					}
				}

				if (minAmt && maxAmt) {
					if (maxAmt == -1 && cart.total >= minAmt) {
						rewards.push(reward);
						continue;
					}
					if (cart.total >= minAmt && cart.total < maxAmt) {
						rewards.push(reward);
						continue;
					}
				}
			}
		}

		if (rewards.length) {
			return Math.max(...rewards);
		}
		return 0;
	}

	function cartSizeRange() {
		if (maxCartSize < 2) {
			maxCartSize = 2;
		} else if (maxCartSize > 6) {
			maxCartSize = 6;
		}

		if (minCartSize < 2) {
			minCartSize = 2;
		} else if (minCartSize > 6) {
			minCartSize = 6;
		}
		if (minCartSize > maxCartSize) {
			minCartSize = maxCartSize;
		}
	}
	function minTotalRange() {
		if (minCartTotal < 100) {
			minCartTotal = 100;
		} else if (minCartTotal > 10000) {
			minCartTotal = 10000;
		}
	}
</script>

{#snippet options()}
	<Label>
		Min Cart Size
		<NumberInput onchange={cartSizeRange} bind:value={minCartSize} />
		<Tooltip>The minimum number of items in the cart. 2-4 is usually a good value.</Tooltip>
	</Label>

	<Label>
		Max Cart Size
		<NumberInput onchange={cartSizeRange} bind:value={maxCartSize} />
		<Tooltip>The maximum number of items in the cart. 4-5 is usually a good value.</Tooltip>
	</Label>

	<Label>
		Minimum Cart Total
		<NumberInput prefix="$" onchange={minTotalRange} bind:value={minCartTotal} />
		<Tooltip>The minimum total cost of the items in the cart</Tooltip>
	</Label>
{/snippet}

<div class="space-y-4">
	<PromoTitle {promo} />

	<div class="flex flex-col justify-center gap-4">
		<OptionsCard
			products={productsSimpleName}
			bind:carts
			bind:requiredProducts
			{minCartSize}
			{maxCartSize}
			{minCartTotal}
			{options}
			{selected_product_ids}
		/>
		<ResultsCard {carts} {requiredProducts} {rewardAmount} />
	</div>
</div>
