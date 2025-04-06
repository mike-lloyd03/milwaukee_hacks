<script lang="ts">
	import { Heading, Listgroup } from 'flowbite-svelte';

	const links = [
		{ name: 'M18 Buy More Save More - Spring 2025', href: '/promos/2025-q2-m18-bmsm' },
		{ name: 'M12 Buy More Save More - Spring 2025', href: '/promos/2025-q2-m12-bmsm' },
		{
			name: 'Landscaping Buy More Save More - Spring 2025',
			href: '/promos/2025-q2-landscaping-bmsm'
		}
	];
</script>

<div class="mx-auto my-6">
	<Heading tag="h2" class="mb-4">Promos</Heading>
	<Listgroup active items={links} let:item class="mx-auto w-96">
		{item.name}
	</Listgroup>
</div>
