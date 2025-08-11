<script lang="ts">
	import type { Cart, Product } from '$lib/pkg/algorithm';
	import { formatCurrency, formatPercent, uniqueProducts } from '$lib/utils';
	import { P, Card, Heading, Button } from 'flowbite-svelte';

	interface Props {
		carts: Cart[];
		requiredProducts: string[];
		rewardAmount: (cart: Cart) => number;
	}

	let { carts, requiredProducts, rewardAmount }: Props = $props();

	let currentCart = $state(0);

	function iterateCart(i: number) {
		if (currentCart + i > carts.length - 1) {
			currentCart = 0;
		} else if (currentCart + i < 0) {
			currentCart = carts.length - 1;
		} else {
			currentCart += i;
		}
	}

	function calculatePromoPrice(cart: Cart, product: Product): number {
		const promoPrice = (1 - rewardAmount(cart) / cart.total) * product.price;
		return promoPrice;
	}
</script>

<Card size="xl" class="mx-auto">
	<Heading tag="h5">
		{#if carts.length > 1}
			Possible Carts
		{:else}
			Cart
		{/if}
	</Heading>
	<div class="my-4 space-y-2">
		{#if carts.length > 0}
			{#key currentCart}
				<div class="rounded-md border border-gray-300 p-2 dark:border-gray-500">
					{#if carts.length > 1}
						<Heading tag="h6">Option {currentCart + 1}</Heading>
					{/if}
					{#each carts[currentCart].items as item (item)}
						<div class="flex items-center justify-between py-2">
							<P color={requiredProducts.includes(item.name) ? 'text-red-500' : undefined}>
								{item.name}
							</P>
							<P>{formatCurrency(item.price)}</P>
						</div>
					{/each}
					<hr />
					<div class="py-2">
						<div class="flex justify-between">
							<P>Subtotal</P>
							<P>{formatCurrency(carts[currentCart].total)}</P>
						</div>
						<div class="flex justify-between">
							<P>Reward Amount</P>
							<P>-{formatCurrency(rewardAmount(carts[currentCart]))}</P>
						</div>
						<div class="flex justify-between">
							<P>Grand Total</P>
							<P>{formatCurrency(carts[currentCart].total - rewardAmount(carts[currentCart]))}</P>
						</div>
					</div>
					<div>
						<P></P>
					</div>
				</div>

				<div>
					<Heading tag="h6">Estimated Prices After Hack</Heading>
					{#each uniqueProducts(carts[currentCart].items) as item (item)}
						<div class="flex items-center justify-between">
							{#if requiredProducts.includes(item.name)}
								<P>{item.name}</P>
								<div class="flex flex-col items-end">
									<P>{formatCurrency(calculatePromoPrice(carts[currentCart], item))}</P>
									<div>
										<P size="sm" opacity={75}>
											Save
											{formatPercent(
												1 - calculatePromoPrice(carts[currentCart], item) / item.price
											)}
										</P>
									</div>
								</div>
							{/if}
						</div>
					{/each}
				</div>

				{#if carts.length > 1}
					<Button color="alternative" onclick={() => iterateCart(-1)}>Prev</Button>
					<Button color="alternative" onclick={() => iterateCart(1)}>Next</Button>
				{/if}
			{/key}
		{:else}
			<div>Awaiting Input</div>
		{/if}
	</div>
</Card>
