<script lang="ts">
	import { Toggle, Tooltip } from 'flowbite-svelte';

	interface Props {
		selectProductsMode: boolean;
	}

	let { selectProductsMode = $bindable() }: Props = $props();
</script>

<div class="my-2">
	<div class="flex gap-2">
		Excluded Products
		<Toggle bind:checked={selectProductsMode} />
		<Tooltip placement="bottom">
			Toggle between selecting products to include in the cart and to exclude
		</Tooltip>
		Selected Products
	</div>
</div>
