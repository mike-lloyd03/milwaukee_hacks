<script lang="ts">
	import type { Promotion } from '$lib/types';
	import { Heading } from 'flowbite-svelte';

	const { promo }: { promo: Promotion } = $props();
</script>

<Heading tag="h4">{promo.long_description ?? promo.short_description}</Heading>
{#if promo.long_description}
	<Heading tag="h5">{promo.short_description}</Heading>
{/if}
<Heading tag="h6">{promo.start_date} - {promo.end_date}</Heading>
