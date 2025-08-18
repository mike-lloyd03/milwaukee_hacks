<script lang="ts">
	import type { Product } from '$lib/types';
	import { formatCurrency, simplifyName } from '$lib/utils';

	interface Props {
		product: Product;
		link: string;
		hlIndices?: Set<number>;
	}

	const { product, link, hlIndices }: Props = $props();
</script>

{#snippet hlChars()}
	{#if hlIndices}
		{#each simplifyName(product.product_label) as char, i (i)}
			{#if hlIndices.has(i)}
				<b>{char}</b>
			{:else}
				{char}
			{/if}
		{/each}
	{:else}
		{simplifyName(product.product_label)}
	{/if}
{/snippet}

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
					{@render hlChars()}
					<!-- {simplifyName(product.product_label)} -->
				</div>
				{formatCurrency(product.pricing.value)}
			</div>
		</a>
	</div>
</div>
