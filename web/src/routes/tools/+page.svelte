<script lang="ts">
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	console.log(data.products);
</script>

<div>
	<ul>
		{#each data.products as product}
			<li>
				<a href="/tools/{product.item_id}">{product.product_label}</a>
			</li>
		{/each}
	</ul>
</div>
