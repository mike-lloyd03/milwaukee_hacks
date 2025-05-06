<script lang="ts">
	import type { PromotionDB } from '$lib/dbTypes';
	import { Heading } from 'flowbite-svelte';

	const { promo }: { promo: PromotionDB } = $props();
</script>

<Heading tag="h2">{promo.name}</Heading>
<Heading tag="h6">{promo.start_date} - {promo.end_date}</Heading>
