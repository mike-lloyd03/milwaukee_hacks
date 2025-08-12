<script lang="ts">
	import { Heading, P } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { isFuture } from '$lib/utils';
	import PromoListGroup from '$lib/components/PromoListGroup.svelte';
	import SearchInput from '$lib/components/SearchInput.svelte';

	let { data }: { data: PageData } = $props();

	let promosFilter = $state('');
	const filteredTerms = ['gloves', 'vests', 'glasses', 'respirators'];

	let promos = data.promos.filter(
		(l) =>
			!filteredTerms.some((t) =>
				(l.long_description ?? l.short_description).toLowerCase().includes(t)
			)
	);

	let activePromos = promos.filter((p) => isFuture(p.end_date));

	let inactivePromos = promos.filter((p) => !isFuture(p.end_date));
</script>

<div class="mx-auto space-y-2">
	<Heading tag="h2" class="mb-4">Promos</Heading>
	<P class="text-center">All active promotion</P>

	<SearchInput bind:value={promosFilter} />

	{#if activePromos.length}
		<PromoListGroup promos={activePromos} searchTerm={promosFilter}></PromoListGroup>
	{/if}

	{#if inactivePromos.length}
		<div class="my-16">
			<Heading tag="h3" class="mb-4">Expired Promos</Heading>
			<PromoListGroup promos={inactivePromos}></PromoListGroup>
		</div>
	{/if}
</div>
