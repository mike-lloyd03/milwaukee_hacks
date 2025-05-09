<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { Label, NumberInput, Tooltip } from 'flowbite-svelte';
	import { simplifyName } from '$lib/utils';

	import type { PageData } from './$types';
	import OptionsCard from '$lib/components/OptionsCard.svelte';
	import ResultsCard from '$lib/components/ResultsCard.svelte';
	import PromoTitle from '$lib/components/PromoTitle.svelte';
	import { onMount } from 'svelte';

	let { data }: { data: PageData } = $props();

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let minCartSize: number = $state(0);
	let maxCartSize: number = $state(0);
	let minCartTotal: number = $state(0);

	let products = data.products.map((p) => {
		p.product_label = simplifyName(p);
		return p;
	});

	onMount(() => {
		let maxCart = 0;
		let minTotal = 0;

		if (data.promo.reward_tiers) {
			const first_tier = data.promo.reward_tiers[0];

			if (first_tier.minPurchaseQuantity && first_tier.maxPurchaseQuantity) {
				// Item quantity based promo
				data.promo.reward_tiers.forEach((tier) => {
					if (tier.minPurchaseQuantity) {
						if (tier.minPurchaseQuantity > maxCart) {
							maxCart = tier.minPurchaseQuantity;
						}
					}
				});

				minCartSize = maxCart;
				maxCartSize = maxCart;
				minCartTotal = minTotal;
			} else if (first_tier.minPurchaseAmount && first_tier.maxPurchaseAmount) {
				// Cart total based promo
				data.promo.reward_tiers.forEach((tier) => {
					if (tier.minPurchaseAmount) {
						if (tier.minPurchaseAmount > minTotal) {
							minTotal = tier.minPurchaseAmount;
						}
					}
				});

				minCartSize = 2;
				maxCartSize = 5;
				minCartTotal = minTotal;
			}
		}
	});

	function newRewardAmount(cart: Cart): number {
		const cartSize = cart.items.length;
		let rewards: number[] = [];

		if (data.promo.reward_tiers) {
			for (let tier of data.promo.reward_tiers) {
				const maxQty = tier.maxPurchaseQuantity;
				const minQty = tier.minPurchaseQuantity;
				const maxAmt = tier.maxPurchaseAmount;
				const minAmt = tier.minPurchaseAmount;

				const reward = tier.rewardAmountPerOrder ?? (cart.total * (tier.rewardPercent ?? 0)) / 100;

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
		<Tooltip>The maximum number of items in the cart. 4-5 is usually a good value.</Tooltip>
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
	<PromoTitle promo={data.promo} />

	<div class="flex flex-col justify-center gap-4">
		<OptionsCard
			{products}
			bind:carts
			bind:requiredProducts
			{minCartSize}
			{maxCartSize}
			{minCartTotal}
			{options}
		/>
		<ResultsCard {carts} {requiredProducts} rewardAmount={newRewardAmount} />
	</div>
</div>
