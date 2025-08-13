<script lang="ts">
	import type { Cart, ProductAlgo } from '$lib/pkg/algorithm';
	import InformationCircle from '$lib/svg/InformationCircle.svelte';
	import { formatCurrency, formatPercent, simplifyName, uniqueProducts } from '$lib/utils';
	import { P, Card, Heading, Button, Tooltip } from 'flowbite-svelte';

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

	function calculatePromoPrice(cart: Cart, product: ProductAlgo): number {
		const promoPrice = (1 - rewardAmount(cart) / cart.total) * product.price;
		return promoPrice;
	}
</script>

<Card size="xl" class="mx-auto">
	<div class="flex justify-center">
		<Heading tag="h5" class="w-fit">
			{#if carts.length > 1}
				Possible Carts
			{:else}
				Cart
			{/if}
		</Heading>
		<InformationCircle />
		<Tooltip
			>The tools required to get the best deal. Desired tools are in red. You must buy all of these
			items and return the undesired ones to get the deal</Tooltip
		>
	</div>
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
								{simplifyName(item.name)}
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
					<div class="flex justify-center">
						<Heading tag="h6" class="w-fit">Estimated Prices After Hack</Heading>
						<InformationCircle />
						<Tooltip>The price(s) of the products(s) after returning the undesired items</Tooltip>
					</div>

					{#each uniqueProducts(carts[currentCart].items) as item (item)}
						<div class="flex items-center justify-between">
							{#if requiredProducts.includes(item.name)}
								<P>{simplifyName(item.name)}</P>
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
