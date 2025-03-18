<script lang="ts">
	import { Cart } from '$lib/pkg/algorithm';
	import { P, Heading } from 'flowbite-svelte';

	import type { PageData } from './$types';
	import OptionsCard from './OptionsCard.svelte';
	import ResultsCard from './ResultsCard.svelte';

	let { data }: { data: PageData } = $props();

	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let products = data.products;
</script>

<div class="space-y-4 text-center">
	<div class="flex flex-col justify-center gap-4 md:flex-row">
		<OptionsCard {products} bind:carts bind:requiredProducts />
		<ResultsCard {carts} {requiredProducts} />
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
			choose "Calculate". You will be presented with three different cart options you can buy at
			Home Depot to get the maximum savings amount on the tool you want.
		</P>
	</div>
</div>
