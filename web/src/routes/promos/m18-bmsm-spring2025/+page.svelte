<script lang="ts">
	import { bmsm, Cart } from '$lib/pkg/algorithm';
	import { getProducts, formatCurrency } from '$lib/utils';
	import { P, Card, NumberInput, Label, Heading, Button, Tooltip } from 'flowbite-svelte';

	let cartSize = $state(4);
	let minTotal = $state(1000);
	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let products = getProducts('m18BmsmSpring2025');

	function getCarts(event: MouseEvent) {
		event.preventDefault();
		const allCarts = bmsm(products, cartSize, minTotal, requiredProducts);
		carts = allCarts.slice(0, 10);
	}

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

<div class="space-y-4 text-center">
	<div class="flex flex-col justify-center gap-4 md:flex-row">
		<Card>
			<Heading tag="h5">Options</Heading>
			<div class="my-4 flex flex-col gap-4">
				<Label>
					Required Products
					<div
						class="flex h-64 flex-col overflow-y-auto rounded-md border border-gray-300 p-2 dark:border-gray-500"
					>
						{#each products as product (product.name)}
							<label class="flex items-center text-sm font-medium text-gray-900 dark:text-gray-300">
								<input
									class="text-primary-600 focus:ring-primary-500 dark:focus:ring-primary-600 me-2 h-4 w-4 rounded border-gray-300 bg-gray-100 focus:ring-2 dark:border-gray-500 dark:bg-gray-600 dark:ring-offset-gray-800"
									type="checkbox"
									name="requiredProduct"
									value={product.name}
									bind:group={requiredProducts}
								/>
								{product.name}
							</label>
						{/each}
					</div>
					<Tooltip>Items which must appear in the cart (the items you want to hack)</Tooltip>
				</Label>

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

				<Button color="red" onclick={getCarts}>Calculate</Button>
			</div>
		</Card>

		<Card>
			<Heading tag="h5">Possible Carts</Heading>
			<div class="my-4 space-y-2">
				{#if carts.length > 0}
					{#key currentCart}
						<div class="rounded-md border border-gray-300 p-2 dark:border-gray-500">
							<Heading tag="h6">Option {currentCart + 1}</Heading>
							{#each carts[currentCart].items as item (item)}
								<div class="flex justify-between">
									<P color={requiredProducts.includes(item.name) ? 'text-red-500' : undefined}>
										{item.name}
									</P>
									<P>{formatCurrency(item.price)}</P>
								</div>
							{/each}
							<hr />
							<div class="flex justify-between">
								<P>Cart Total</P>
								<P>{formatCurrency(carts[currentCart].total)}</P>
							</div>
							<div>
								<P></P>
							</div>
						</div>
						<div>
							<Heading tag="h6">Estimated Price After Promo</Heading>
							{#each carts[currentCart].items as item}
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
