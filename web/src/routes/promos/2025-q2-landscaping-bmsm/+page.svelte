<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { P, Heading } from 'flowbite-svelte';
	import { simplifyName } from '$lib/utils';

	import type { PageData } from './$types';
	import OptionsCard from '$lib/components/OptionsCard.svelte';
	import ResultsCard from '$lib/components/ResultsCard.svelte';
	import PromoTitle from '$lib/components/PromoTitle.svelte';

	let { data }: { data: PageData } = $props();

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let products = data.products.map((p) => {
		p.product_label = simplifyName(p);
		return p;
	});

	function rewardAmount(cart: Cart): number {
		if (cart.items.length == 3) {
			return cart.total * 0.1;
		} else if (cart.items.length == 4) {
			return cart.total * 0.2;
		} else if (cart.items.length >= 5) {
			return cart.total * 0.25;
		} else {
			return 0;
		}
	}
</script>

<div class="space-y-4">
	<PromoTitle promo={data.promo} />

	<div class="flex flex-col justify-center gap-4">
		<OptionsCard {products} bind:carts bind:requiredProducts minCartSize={5} maxCartSize={5} />
		<ResultsCard {carts} {requiredProducts} {rewardAmount} />
	</div>

	<div class="space-y-2">
		<Heading tag="h5">How This Works</Heading>
		<P>
			This promo offers different percentage values off the total price once a certain number of
			items have been added to the cart. Buy three items, get 10% off. Buy four, 15% off. Buy five
			or more, 25% off.
		</P>
		<P>
			So to hack this, add the item(s) you want to the cart and add other items from the promo until
			you have at least five items in your cart. Return the items you don't want.
		</P>
		<P>
			In the form above, choose the tool (or tools) you want from the "Required Products" field and
			choose "Calculate". If an item is not in stock, you can exclude it from your cart by toggling
			the slider above the product list. You will be presented with different cart options you can
			buy at Home Depot to get the maximum savings amount on the tool you want.
		</P>
	</div>
	<div>{JSON.stringify(data.promo)}</div>
</div>
