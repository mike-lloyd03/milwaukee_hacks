<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { P, Heading } from 'flowbite-svelte';
	import { simplifyName } from '$lib/utils';

	import type { PageData } from './$types';
	import OptionsCard from './OptionsCard.svelte';
	import ResultsCard from './ResultsCard.svelte';

	let { data }: { data: PageData } = $props();

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let products = data.products.map((p) => {
		p.product_label = simplifyName(p);
		return p;
	});
</script>

<div class="space-y-4">
	<div class="flex flex-col justify-center gap-4">
		<OptionsCard {products} bind:carts bind:requiredProducts />
		<ResultsCard {carts} {requiredProducts} />
	</div>

	<div class="space-y-2">
		<Heading tag="h5">How This Works</Heading>
		<P>
			This promo offers different dollar values off the total price once a certain number of items
			have been added to the cart. Buy two items, get $100 back. Buy three, $200 back.
		</P>
		<P>
			So to hack this, add the item you want to the cart and add two more of the cheapest items on
			the promo. Return the items you don't want.
		</P>
		<P>
			In the form above, choose the tool (or tools) you want from the "Required Products" field and
			choose "Calculate". If an item is not in stock, you can exclude it from your cart by toggling
			the slider above the product list. You will be presented with different cart options you can
			buy at Home Depot to get the maximum savings amount on the tool you want.
		</P>
	</div>
</div>
