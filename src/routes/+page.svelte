<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import Tealinux from '$lib/assets/Vector.png';

	const isOnline = async () => {
		let online = await invoke('is_online');
		return online;
	};

	const setReadJSON = async () => {
		await invoke('set_read_json');
	};

	const setBlueprintJSON = async () => {
		await invoke('set_empty_blueprint');
	};

	onMount(() => {
		isOnline();
		setBlueprintJSON();
	});
</script>


<div class="flex items-center justify-center min-h-screen">
	<div class="text-center">
		<img src={Tealinux} alt="" class="mx-auto mb-4" />

		{#await setReadJSON() then _}
			<h1 class="font-archivo font-semibold text-6xl -tracking-[4.5%] mb-4">Welcome to Tealinux</h1>
			<h2 class="font-poppin mb-4 text-4xl tracking-normal">Press start to install</h2>

			<div class="p-2">
				<br />
				<div>
					<button
						on:click={() => (window.location.href = '/installation')}
						class=" bg-greenTealinux rounded-3xl hover:bg-green-800 text-white font-poppinsemibold text-xl py-[10px] px-28 disabled:bg-green-400 border-2 border-black/15"
					>
						START
					</button>
				</div>
			</div>
		{/await}
	</div>
</div>
