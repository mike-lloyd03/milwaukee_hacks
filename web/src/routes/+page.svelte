<script lang="ts">
	import { Heading, Listgroup } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { isFuture } from '$lib/utils';

	let { data }: { data: PageData } = $props();

	const activeLinks = data.promos
		.filter((p) => {
			return isFuture(p.end_date);
		})
		.map((p) => {
			return {
				name: p.name,
				href: `/promos/${p.item_id}`
			};
		});

	const inactiveLinks = data.promos
		.filter((p) => {
			return !isFuture(p.end_date);
		})
		.map((p) => {
			return {
				name: p.name,
				href: `/promos/${p.item_id}`
			};
		});
</script>

<div class="mx-auto my-6">
	<Heading tag="h2" class="mb-4">Promos</Heading>
	<Heading tag="h3" class="mb-4">Active</Heading>
	<Listgroup active items={activeLinks} let:item class="mx-auto w-96">
		{item.name}
	</Listgroup>

	<div class="my-16">
		<Heading tag="h3" class="mb-4">Expired Promos</Heading>
		<Listgroup active items={inactiveLinks} let:item class="mx-auto w-96">
			{item.name}
		</Listgroup>
	</div>
</div>
