<script lang="ts">
	import { Heading, Listgroup } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { isFuture } from '$lib/utils';

	let { data }: { data: PageData } = $props();

	interface Link {
		name: string;
		href: string;
	}

	const activeLinks: Link[] = data.promos
		.filter((p) => {
			return isFuture(p.end_date);
		})
		.map((p) => {
			return {
				name: p.long_description ?? p.short_description,
				href: `/promos/${p.promotion_id}`
			};
		});

	const inactiveLinks: Link[] = data.promos
		.filter((p) => {
			return !isFuture(p.end_date);
		})
		.map((p) => {
			return {
				name: p.long_description ?? p.short_description,
				href: `/promos/${p.promotion_id}`
			};
		});
</script>

<div class="mx-auto my-6">
	<Heading tag="h2" class="mb-4">Promos</Heading>

	{#if activeLinks.length}
		<Listgroup active items={activeLinks} let:item class="mx-auto w-2xl">
			{item.name}
		</Listgroup>
	{/if}

	{#if inactiveLinks.length}
		<div class="my-16">
			<Heading tag="h3" class="mb-4">Expired Promos</Heading>
			<Listgroup active items={inactiveLinks} let:item class="mx-auto w-96">
				{item.name}
			</Listgroup>
		</div>
	{/if}
</div>
