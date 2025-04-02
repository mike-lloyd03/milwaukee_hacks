<script lang="ts">
	import { Heading, Listgroup } from 'flowbite-svelte';

	const links = [
		{ name: 'M18 Buy More Save More - Spring 2025', href: '/promos/m18-bmsm-spring2025' },
		{ name: 'M12 Buy More Save More - Spring 2025', href: '/promos/m12-bmsm-spring2025' }
	];
</script>

<div class="mx-auto my-6">
	<Heading tag="h2" class="mb-4">Promos</Heading>
	<Listgroup active items={links} let:item class="mx-auto w-96">
		{item.name}
	</Listgroup>
</div>
