<script lang="ts">
	import { Heading, Listgroup } from 'flowbite-svelte';

	const links = [
		{
			name: 'Landscaping Buy More Save More - Spring 2025',
			href: '/promos/2025-q2-landscaping-bmsm'
		}
		// {
		// 	name: 'Packout Buy More Save More - Summer 2025',
		// 	href: '/promos/2025-q3-packout-bmsm'
		// }
	];

	const oldPromoLinks = [
		{ name: 'M18 Buy More Save More - Spring 2025', href: '/promos/2025-q2-m18-bmsm' },
		{ name: 'M12 Buy More Save More - Spring 2025', href: '/promos/2025-q2-m12-bmsm' }
	];
</script>

<div class="mx-auto my-6">
	<Heading tag="h2" class="mb-4">Promos</Heading>
	<Heading tag="h3" class="mb-4">Active</Heading>
	<Listgroup active items={links} let:item class="mx-auto w-96">
		{item.name}
	</Listgroup>

	<div class="my-16">
		<Heading tag="h3" class="mb-4">Expired Promos</Heading>
		<Listgroup active items={oldPromoLinks} let:item class="mx-auto w-96">
			{item.name}
		</Listgroup>
	</div>
</div>
