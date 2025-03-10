<script lang="ts">
	import { bmsm, Cart } from '$lib/pkg/milwaukee_hacks_rs';
	import { getProducts } from '$lib/products';
	import { Product } from '$lib/pkg/milwaukee_hacks_rs.js';

	let cartSize = $state(5);
	let minTotal = $state(1000);
	let requiredProducts: string[] = $state([]);
	let carts: Cart[] = $state([]);
	let products: Product[] = $state([]);

	$effect(() => {
		products = getProducts();
	});

	function getCarts(event: MouseEvent) {
		event.preventDefault();
		const allCarts = bmsm(products, cartSize, minTotal, requiredProducts);
		carts = allCarts.slice(0, 3);
	}
</script>

<div class="text-center">
	<h1 class="text-lg font-bold text-red-700">Milwaukee Hacks</h1>
	<div class="flex justify-center">
		<div class="flex flex-col">
			<label>
				Max Cart Size
				<input type="number" name="cartSize" bind:value={cartSize} />
			</label>

			<label>
				Minimum Cart Total
				<input type="number" name="minTotal" bind:value={minTotal} />
			</label>
			<div class="flex h-64 flex-col overflow-y-auto">
				{#each products as product (product.name)}
					<label>
						<input
							type="checkbox"
							name="requiredProduct"
							value={product.name}
							bind:group={requiredProducts}
						/>
						{product.name}
					</label>
				{/each}
			</div>
			<button onclick={getCarts}>Calculate</button>
		</div>

		<div>
			<h3>Best Carts:</h3>
			{#if carts.length > 0}
				{#each carts as cart, i (cart)}
					<div>
						<h4>Option {i + 1}</h4>
						{#each cart.items as item (item)}
							<p>{item.name} {item.price}</p>
						{/each}
						<p>Total: {cart.total}</p>
					</div>
				{/each}
			{/if}
		</div>
	</div>
</div>
