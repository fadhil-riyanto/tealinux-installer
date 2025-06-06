<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getShortLocale, getBlueprint } from '../global.js';
	import { getCurrency } from 'locale-currency';

	let searchTerm = '';

	let locales = [];
	let showLocales = false;
	let selectedLocale = null;
	let isOpen = false;
	let filteredLocales = [];

	const getLocale = async () => {
		invoke('get_locale_json').then((response) => {
			locales = JSON.parse(response);
			filteredLocales = locales;
			showLocales = true;
		});
	};

	const handleSetLocale = async () => {
		await invoke('blueprint_set_locale', { locale: selectedLocale });
	};

	const selectLocale = (value) => {
		selectedLocale = value;
		isOpen = false;
		searchTerm = value;
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredLocales = locales.filter((locale) => locale.name.toLowerCase().includes(term));
	}

	function handleFocusIn() {
		isOpen = true;
	}

	$: searchTerm, filterOptions();

	let date = new Date();
	let dateOptions = {
		weekday: 'short',
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	};
	let number = 1234567.89;
	let price = 1234.56;

	let timePreview = '';
	let numberPreview = '';
	let currencyPreview = '';

	const handlePreview = () => {
		if (selectedLocale != null) {
			let short = getShortLocale(selectedLocale);

			timePreview = date.toLocaleDateString(short, dateOptions);
			numberPreview = number.toLocaleString(short);

			let currencyCode = getCurrency(short);

			let currency = new Intl.NumberFormat(short, {
				style: 'currency',
				currency: currencyCode
			});

			currencyPreview = currency.format(price);
		}
	};

	$: selectedLocale, handlePreview();

	onMount(() => {
		getLocale();
		getBlueprint().then((blueprint) => {
			if (blueprint.locale === null) {
				selectedLocale = 'en_US.UTF-8 UTF-8';
				searchTerm = 'en_US.UTF-8 UTF-8';
			} else {
				selectedLocale = blueprint.locale.main;
				searchTerm = blueprint.locale.main;
			}
		});
	});
</script>

<Sidebar />
<div class="relative w-full">
	<header
		class="flex items-center justify-center w-full gap-[10px] py-10 fixed top-0 bg-whiteTealinux z-30"
	>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
	</header>
	<section class="flex flex-col items-center justify-center h-screen pt-20">
		{#if showLocales}
			<!-- bg-slate-300 -->
			<form class=" text-center w-[50dvw] p-8 rounded-md h-[85dvh]">
				<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Locales</h1>

				<div class="relative max-w-md mx-auto mb-4">
					<h2 class="font-poppin text-left mb-2 font-medium">Main Locale</h2>
					<div
						class="relative flex items-center w-full h-[45px] rounded-lg overflow-hidden bg-grayTealinux border-2 border-black"
					>
						<input
							type="text"
							placeholder="Search..."
							class="peer h-full w-full outline-hidden bg-grayTealinux text-sm text-gray-700 pr-2 pl-8"
							bind:value={searchTerm}
							on:click={handleFocusIn}
							on:focus={handleFocusIn}
						/>
						<div class="inset-y-0 left-0 flex items-center pr-4">
							<!-- <img src={mag_glass} alt="" class="h-4 w-4" /> -->
							<svg
								width="20"
								height="21"
								viewBox="0 0 20 21"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									d="M19 19.5L14.65 15.15M17 9.5C17 13.9183 13.4183 17.5 9 17.5C4.58172 17.5 1 13.9183 1 9.5C1 5.08172 4.58172 1.5 9 1.5C13.4183 1.5 17 5.08172 17 9.5Z"
									stroke="black"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								/>
							</svg>
						</div>
					</div>

					{#if isOpen}
						<div
							class="absolute z-10 w-full bg-white border border-greyBorder rounded-b-xl max-h-[30vh] overflow-y-auto"
						>
							{#each filteredLocales as locale}
								<label
									class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-slate-400 last:border-none bg-slate-200 hover:bg-slate-100 transition-all"
									for={locale.name}
								>
									<input
										type="radio"
										name="locale"
										id={locale.name}
										class="h-6 w-6"
										value={locale.name}
										on:click={(e) => selectLocale(e.target.value)}
									/>
									<p>{locale.name}</p>
								</label>
							{/each}
						</div>
					{/if}
				</div>

				<div class="max-w-md mx-auto mb-4">
					<h2 class="font-poppin text-left mb-2 font-medium">Date and time</h2>
					<div
						class="relative flex items-center w-full h-[45px] rounded-lg overflow-hidden bg-grayTealinux border-2 border-black pr-2 pl-8"
					>
						<!-- <input type="text" id="" class="peer h-full w-full outline-hidden text-sm text-gray-700 pr-2 pl-[12px]"> -->
						<span>{timePreview}</span>
					</div>
				</div>

				<div class="max-w-md mx-auto mb-4">
					<h2 class="font-poppin text-left mb-2 font-medium">Number and currency</h2>
					<div
						class="relative flex items-center w-full h-[45px] rounded-lg overflow-hidden bg-grayTealinux border-2 border-black pr-2 pl-8"
					>
						<!-- <input type="text" id="" class="peer h-full w-full outline-hidden text-sm text-gray-700 pr-2 pl-[12px]"> -->
						<span>{numberPreview} - {currencyPreview}</span>
					</div>
				</div>
			</form>
			<div class="max-w-md mx-auto mt-[208px] fixed bottom-0 h-[15dvh] flex items-center">
				<div class="grid grid-cols-2 gap-[295px]">
					<a
						href="/installation/timezone"
						class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]"
						>Back</a
					>
					<a
						href="/installation/account"
						on:click={handleSetLocale}
						class="text-white bg-greenTealinux {selectedLocale
							? ''
							: ' brightness-75 pointer-events-none'}  focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-hidden"
						>Next</a
					>
				</div>
			</div>
		{/if}
	</section>
</div>
