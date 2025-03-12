<script lang="ts">
	import '../app.css';
	import { DarkMode, Heading } from 'flowbite-svelte';
	let { children } = $props();
</script>

<svelte:head>
	<title>Milwaukee Hacks</title>
</svelte:head>

<DarkMode />

<div class="text-center">
	<Heading tag="h1" class="mb-6"><a href="/">Milwaukee Hacks</a></Heading>
	{@render children()}
</div>
