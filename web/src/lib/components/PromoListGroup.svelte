<script lang="ts">
	import type { Promotion } from '$lib/types';
	import { Listgroup } from 'flowbite-svelte';

	interface Props {
		promos: Promotion[];
		includeExperienceTag?: boolean;
		selectedProduct?: string;
		searchTerm?: string;
	}

	let { promos, includeExperienceTag, selectedProduct, searchTerm }: Props = $props();

	let links = promos.map((p) => ({
		title: p.long_description ?? p.short_description,
		experience_tag: p.experience_tag,
		href: `/promos/${p.promotion_id}${selectedProduct ? `?products=${selectedProduct}` : ''}`
	}));

	let filteredLinks = $derived(
		links.filter((l) => !searchTerm || l.title.toLowerCase().includes(searchTerm.toLowerCase()))
	);
</script>

{#if filteredLinks.length > 0}
	<Listgroup active items={filteredLinks} let:item class="mx-auto w-2xl">
		{item.title}
		{#if includeExperienceTag}
			({item.experience_tag})
		{/if}
	</Listgroup>
{:else}
	No results
{/if}
