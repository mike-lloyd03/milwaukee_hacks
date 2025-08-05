<script lang="ts">
	import type { ProductDB } from '$lib/dbTypes';
	import { formatCurrency } from '$lib/utils';

	interface Props {
		product: ProductDB;
		link: string;
	}

	const { product, link }: Props = $props();
</script>

<div>
	<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
		<a href={link} class="flex items-center py-2 text-sm font-medium">
			<div class="flex w-full items-center justify-between">
				<div class="flex items-center">
					<img
						src={product.image_primary_url.replace('<SIZE>', '65')}
						alt="tool"
						class="rounded-md"
					/>
					{product.product_label}
				</div>
				{formatCurrency(product.pricing_value)}
			</div>
		</a>
	</div>
</div>
