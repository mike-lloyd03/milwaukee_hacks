<script lang="ts">
	import '../app.css';
	import { DarkMode, Navbar, NavBrand, NavHamburger, NavLi, NavUl } from 'flowbite-svelte';
	import Logo from '$lib/svg/Logo.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>Milwaukee Hacks</title>
</svelte:head>

<div class="fixed top-0 w-full">
	<Navbar class="mx-auto max-w-6xl">
		<NavBrand href="/">
			<Logo />
			<span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
				Milwaukee Hacks
			</span>
		</NavBrand>

		<NavHamburger />

		<NavUl
			ulClass="flex flex-col p-4 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:text-sm md:font-medium md:items-center"
			slideParams={{ delay: 0, duration: 250 }}
		>
			<NavLi href="/">Home</NavLi>
			<NavLi href="/tools">Tools</NavLi>
			<NavLi href="/promos">Promos</NavLi>
			<NavLi href="/contact">Contact</NavLi>
			<NavLi><DarkMode /></NavLi>
		</NavUl>
	</Navbar>
</div>

<div class="mx-auto mt-32 mb-16 max-w-6xl text-center text-gray-900 dark:text-gray-300">
	<div class="mx-2">
		{@render children()}
	</div>
</div>
