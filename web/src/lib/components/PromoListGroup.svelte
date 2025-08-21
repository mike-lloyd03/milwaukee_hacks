<script lang="ts">
	import type { Promotion } from '$lib/types';
	import { Listgroup, ListgroupItem } from 'flowbite-svelte';
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
		id: p.promotion_id,
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
	<Listgroup active class="mx-auto text-gray-700">
		{#each results as result (result.item.id)}
			<ListgroupItem active>
				<a href={result.item.href} class="mx-auto text-center">
					<HighlightText str={result.item.title} indices={result.positions} />
					{#if includeExperienceTag}
						({result.item.experience_tag})
					{/if}
				</a>
			</ListgroupItem>
		{/each}
	</Listgroup>
{:else}
	<div>No results</div>
{/if}
