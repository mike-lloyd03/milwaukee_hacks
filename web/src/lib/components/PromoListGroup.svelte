<script lang="ts">
	import type { Promotion } from '$lib/types';
	import { Listgroup } from 'flowbite-svelte';
	import { Fzf } from 'fzf';
	import HighlightText from './HighlightText.svelte';

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

	const promosFzf = new Fzf(links, {
		selector: (link) => link.title
	});
	const results = $derived(promosFzf.find(searchTerm ?? ''));
</script>

{#if filteredLinks.length > 0}
	<Listgroup
		active
		items={results.map((r) => ({
			title: r.item.title,
			experience_tag: r.item.experience_tag,
			href: r.item.href,
			indices: r.positions
		}))}
		let:item={result}
		class="mx-auto w-2xl"
	>
		<HighlightText str={result.title} indices={result.indices} />
		{#if includeExperienceTag}
			({result.experience_tag})
		{/if}
	</Listgroup>
{:else}
	No results
{/if}
