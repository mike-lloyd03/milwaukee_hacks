<script lang="ts">
	import '../app.css';
	import { DarkMode, Navbar, NavBrand, NavHamburger, NavLi, NavUl } from 'flowbite-svelte';
	import Logo from '$lib/svg/Logo.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<title>Milwaukee Hacks</title>
</svelte:head>

<Navbar>
	<NavBrand href="/">
		<Logo />
		<span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white"
			>Milwaukee Hacks</span
		>
	</NavBrand>
	<NavHamburger />
	<NavUl
		ulClass="flex flex-col p-4 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:text-sm md:font-medium md:items-center"
	>
		<NavLi href="/">Home</NavLi>
		<NavLi href="/contact">Contact</NavLi>
		<NavLi><DarkMode /></NavLi>
	</NavUl>
</Navbar>

<div class="text-center">
	{@render children()}
</div>
