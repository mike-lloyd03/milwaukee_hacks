<script lang="ts">
	import { page } from '$app/state';
	import type { PageData } from './$types';
	import Bmsm from './BMSM.svelte';
	import Bogo from './BOGO.svelte';
	let { data }: { data: PageData } = $props();

	const selected_product_ids = page.url.searchParams.get('products')?.split(',');
</script>

<div>
	{#if data.promo.experience_tag == 'BMSM'}
		<Bmsm promo={data.promo} products={data.products} {selected_product_ids} />
	{:else if data.promo.experience_tag == 'BOGO'}
		<Bogo promo={data.promo} products={data.products} {selected_product_ids} />
	{/if}
</div>

<pre class="text-left">
    {JSON.stringify(data.promo, null, 4)}
</pre>
