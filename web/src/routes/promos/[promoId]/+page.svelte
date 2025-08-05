<script lang="ts">
	import type { PageData } from './$types';
	import Bmsm from './BMSM.svelte';
	import Bogo from './BOGO.svelte';
	let { data }: { data: PageData } = $props();
</script>

<div>
	{#if data.promo.experience_tag == 'BMSM'}
		<Bmsm promo={data.promo} products={data.products} />
	{:else if data.promo.experience_tag == 'BOGO'}
		<Bogo promo={data.promo} products={data.products} />
	{/if}
</div>
