<script lang="ts">
	import { Heading } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { formatCurrency, simplifyName } from '$lib/utils';
	import PromoListGroup from '$lib/components/PromoListGroup.svelte';
	import { PUBLIC_ENV } from '$env/static/public';

	let { data }: { data: PageData } = $props();
</script>

<div class="flex">
	<a href={`https://www.homedepot.com${data.product.canonical_url}`} target="_blank" class="w-full">
		<img
			src={data.product.image_primary_url.replace('<SIZE>', '400')}
			alt="tool"
			class="rounded-md"
		/>
	</a>
	<div class="space-y-4">
		<Heading tag="h3">
			<a
				href={`https://www.homedepot.com${data.product.canonical_url}`}
				class="hover:underline"
				target="_blank">{simplifyName(data.product.product_label)}</a
			>
		</Heading>

		<Heading tag="h4">{data.product.brand_name}</Heading>

		<Heading tag="h6">{formatCurrency(data.product.pricing.value)}</Heading>

		<div class="space-y-2">
			<Heading tag="h4">Available Promotions:</Heading>
			{#if data.promos.length > 0}
				<PromoListGroup promos={data.promos} selectedProduct={data.product.item_id} />
			{:else}
				No promotions available
			{/if}
		</div>
	</div>
</div>

{#if PUBLIC_ENV == 'dev'}
	<pre class="text-left">
        {JSON.stringify(data.product, null, 4)}
    </pre>
{/if}
