<script lang="ts">
	import { Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { formatCurrency } from '$lib/utils';

	let { data }: { data: PageData } = $props();
</script>

<div class="flex">
	<img
		src={data.product.image_primary_url.replace('<SIZE>', '400')}
		alt="tool"
		class="rounded-md"
	/>
	<div>
		<Heading tag="h3"
			><a
				href={`https://www.homedepot.com${data.product.canonical_url}`}
				class="hover:underline"
				target="_blank">{data.product.product_label}</a
			></Heading
		>
		<Heading tag="h4">{data.product.brand_name}</Heading>
		<P>{formatCurrency(data.product.pricing_value)}</P>
	</div>
</div>
{JSON.stringify(data.product)}
