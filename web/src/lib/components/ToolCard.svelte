<script lang="ts">
	import type { Product } from '$lib/types';
	import { formatCurrency, simplifyName } from '$lib/utils';
	import HighlightText from './HighlightText.svelte';

	interface Props {
		product: Product;
		link: string;
		hlIndices?: Set<number>;
	}

	const { product, link, hlIndices }: Props = $props();
</script>

<div>
	<div class="my-2 rounded-md bg-gray-200 px-3 py-1 dark:bg-gray-700">
		<a href={link} class="flex items-center py-2 text-sm font-medium">
			<div class="flex w-full items-center justify-between">
				<div class="flex items-center gap-2">
					<img
						src={product.image_primary_url.replace('<SIZE>', '65')}
						alt="tool"
						class="rounded-md"
					/>
					<HighlightText str={simplifyName(product.product_label)} indices={hlIndices} />
				</div>
				{formatCurrency(product.pricing.value)}
			</div>
		</a>
	</div>
</div>
