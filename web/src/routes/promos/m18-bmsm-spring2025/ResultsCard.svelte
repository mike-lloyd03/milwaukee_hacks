<script lang="ts">
	import type { Cart } from '$lib/pkg/algorithm';
	import { formatCurrency } from '$lib/utils';
	import { P, Card, Heading, Button } from 'flowbite-svelte';

	interface Props {
		carts: Cart[];
		requiredProducts: string[];
	}

	let { carts, requiredProducts }: Props = $props();

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

	function discountAmount(cartTotal: number): number {
		if (cartTotal < 350) {
			return 0;
		} else if (cartTotal < 600) {
			return 80;
		} else if (cartTotal < 800) {
			return 180;
		} else if (cartTotal < 1000) {
			return 280;
		} else {
			return 400;
		}
	}

	function calculatePromoPrice(cartTotal: number, productPrice: number): string {
		const promoPrice = (1 - discountAmount(cartTotal) / cartTotal) * productPrice;
		return formatCurrency(promoPrice);
	}
</script>

<Card size="md">
	<Heading tag="h5">Possible Carts</Heading>
	<div class="my-4 space-y-2">
		{#if carts.length > 0}
			{#key currentCart}
				<div class="rounded-md border border-gray-300 p-2 dark:border-gray-500">
					<Heading tag="h6">Option {currentCart + 1}</Heading>
					{#each carts[currentCart].items as item (item)}
						<div class="flex items-center justify-between py-2">
							<P color={requiredProducts.includes(item.name) ? 'text-red-500' : undefined}>
								{item.name}
							</P>
							<P>{formatCurrency(item.price)}</P>
						</div>
					{/each}
					<hr />
					<div class="flex justify-between py-2">
						<P>Cart Total</P>
						<P>{formatCurrency(carts[currentCart].total)}</P>
					</div>
					<div>
						<P></P>
					</div>
				</div>
				<div>
					<Heading tag="h6">Estimated Price After Promo</Heading>
					{#each carts[currentCart].items as item (item)}
						<div class="flex justify-between">
							{#if requiredProducts.includes(item.name)}
								<P>{item.name}</P>
								<P>{calculatePromoPrice(carts[currentCart].total, item.price)}</P>
							{/if}
						</div>
					{/each}
				</div>
				<Button color="alternative" onclick={() => iterateCart(-1)}>Prev</Button>
				<Button color="alternative" onclick={() => iterateCart(1)}>Next</Button>
			{/key}
		{:else}
			<div>Awaiting Input</div>
		{/if}
	</div>
</Card>
