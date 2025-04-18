<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { P, Heading, Label, NumberInput, Tooltip } from 'flowbite-svelte';
	import { simplifyName } from '$lib/utils';

	import type { PageData } from './$types';
	import OptionsCard from '$lib/components/OptionsCard.svelte';
	import ResultsCard from '$lib/components/ResultsCard.svelte';

	let { data }: { data: PageData } = $props();

	let products = data.products.map((p) => {
		p.product_label = simplifyName(p);
		return p;
	});

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let cartSize = $state(4);
	let minTotal = $state(1000);

	function cartSizeRange() {
		if (cartSize < 2) {
			cartSize = 2;
		} else if (cartSize > 6) {
			cartSize = 6;
		}
	}
	function minTotalRange() {
		if (minTotal < 100) {
			minTotal = 100;
		} else if (minTotal > 10000) {
			minTotal = 10000;
		}
	}

	function rewardAmount(cart: Cart): number {
		if (cart.total < 350) {
			return 0;
		} else if (cart.total < 600) {
			return 80;
		} else if (cart.total < 800) {
			return 180;
		} else if (cart.total < 1000) {
			return 280;
		} else {
			return 400;
		}
	}
</script>

{#snippet options()}
	<Label>
		Max Cart Size
		<NumberInput onchange={cartSizeRange} bind:value={cartSize} />
		<Tooltip>The maximum number of items in the cart. 4-5 is usually a good value.</Tooltip>
	</Label>

	<Label>
		Minimum Cart Total
		<NumberInput prefix="$" onchange={minTotalRange} bind:value={minTotal} />
		<Tooltip>The minimum total cost of the items in the cart</Tooltip>
	</Label>
{/snippet}

<div class="space-y-4 text-center">
	<div class="flex flex-col justify-center gap-4">
		<OptionsCard
			{products}
			bind:carts
			bind:requiredProducts
			minCartSize={2}
			bind:maxCartSize={cartSize}
			bind:minCartTotal={minTotal}
			{options}
		/>
		<ResultsCard {carts} {requiredProducts} {rewardAmount} />
	</div>

	<div class="space-y-2">
		<Heading tag="h5">How This Works</Heading>
		<P>
			This promo offers different dollar values off the total price once different cart total values
			are met. If your cart totals $350 or more, you get $80 off. $600 or more, you'll get $180 off.
			To maximize savings on this deal, put at least $1000 in your cart and receive $400 off the
			total.
		</P>
		<P>
			The hacking part of this is the same as any other hack at Home Depot: return the items you
			don't want. You'll effectively get the tools you want for about 40% off.
		</P>
		<P>
			In the form above, choose the tool (or tools) you want from the "Required Products" field and
			choose "Calculate". You will be presented with different cart options you can buy at Home
			Depot to get the maximum savings amount on the tool you want.
		</P>
	</div>
</div>
