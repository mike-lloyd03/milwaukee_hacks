<script lang="ts">
	import type { ProductDB } from '$lib/dbTypes';
	import { formatCurrency, simplifyName } from '$lib/utils';

	interface Props {
		product: ProductDB;
	}

	let { product }: Props = $props();
</script>

<div class="flex w-full items-center justify-between">
	<div class="flex items-center">
		<img src={product.image_primary_url.replace('<SIZE>', '65')} alt="tool" class="rounded-md" />
		<a
			href={`https://www.homedepot.com${product.canonical_url}`}
			class="mx-2 hover:underline"
			target="_blank">{simplifyName(product.product_label)}</a
		>
	</div>
	{formatCurrency(product.pricing.value)}
</div>
