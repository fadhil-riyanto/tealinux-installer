<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { getBlueprint } from '../global.js';

	let fullname, username, hostname, password, confirmPassword;
	let isAdministrator = false;
	let passwordsMatch = false;
	let passwordVisible = false;
	let passwordConfirmVisible = false;

	function togglePasswordVisibility() {
		passwordVisible = !passwordVisible;
	}

	function togglePasswordConfirmVisibility() {
		passwordConfirmVisible = !passwordConfirmVisible;
	}

	const handleSetAccount = async () => {
		if (password !== confirmPassword) {
			passwordsMatch = false;
			return;
		}
		passwordsMatch = true;

		await invoke('blueprint_set_account', { fullname, username, hostname, password });
	};

	$: if (password && confirmPassword && password === confirmPassword) {
		passwordsMatch = true;
	} else {
		passwordsMatch = false;
	}

	onMount(() => {
		getBlueprint().then((blueprint) => {
			if (blueprint.account !== null) {
				fullname = blueprint.account.fullname;
				username = blueprint.account.username;
				hostname = blueprint.account.hostname;
				password = blueprint.account.password;
				confirmPassword = blueprint.account.password;
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
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
	</header>
	<section class="flex flex-col items-center justify-center h-screen pt-20">
		<form class="flex flex-col h-[85dvh]">
			<h1 class="text-center mb-6 font-archivo text-[32px] font-bold">Create User</h1>

			<div class="max-w-md mx-auto mb-4">
				<div class="flex mb-2 items-center gap-x-2">
					<h2 class="font-poppin text-left font-medium">Full name</h2>
					{#if !fullname}
						<p class="text-red-500 text-[12px]">* Full name is required</p>
					{/if}
				</div>
				<div
					class="relative flex items-center w-[451px] h-[45px] rounded-lg overflow-hidden border-2 border-black bg-grayTealinux"
				>
					<input
						class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
						type="text"
						bind:value={fullname}
						placeholder="Full name"
					/>
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<div class="flex mb-2 items-center gap-x-2">
					<h2 class="font-poppin text-left font-medium">Username</h2>
					{#if !username}
						<p class="text-red-500 text-[12px]">* Username is required</p>
					{/if}
				</div>
				<div
					class="relative flex items-center w-[451px] h-[45px] rounded-lg overflow-hidden border-2 border-black bg-grayTealinux"
				>
					<input
						class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
						type="text"
						bind:value={username}
						placeholder="Username"
					/>
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<div class="flex mb-2 items-center gap-x-2">
					<h2 class="font-poppin text-left font-medium">Computer Name</h2>
					{#if !hostname}
						<p class="text-red-500 text-[12px]">* Computer Name is required</p>
					{/if}
				</div>
				<div
					class="relative flex items-center w-[451px] h-[45px] rounded-lg overflow-hidden border-2 border-black bg-grayTealinux"
				>
					<input
						class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
						type="text"
						bind:value={hostname}
						placeholder="Computer name"
					/>
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<div class="flex mb-2 items-center gap-x-2">
					<h2 class="font-poppin text-left font-medium">Password</h2>
					{#if !password}
						<p class="text-red-500 text-[12px]">* Password is required</p>
					{/if}
				</div>
				<div
					class="relative flex items-center w-[451px] h-[45px] rounded-lg overflow-hidden border-2 border-black bg-grayTealinux"
				>
					{#if passwordVisible}
						<input
							class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
							type="text"
							bind:value={password}
							placeholder="Enter your password"
						/>
					{:else}
						<input
							class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
							type="password"
							bind:value={password}
							placeholder="Enter your password"
						/>
					{/if}
					<svg
						class="mr-[16px]"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						on:click={togglePasswordVisibility}
						xmlns="http://www.w3.org/2000/svg"
					>
						<mask
							id="mask0_461_173"
							style="mask-type:luminance"
							maskUnits="userSpaceOnUse"
							x="0"
							y="0"
							width="24"
							height="24"
						>
							<path d="M24 0H0V24H24V0Z" fill="white" />
						</mask>
						<g mask="url(#mask0_461_173)">
							<path
								d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"
								fill="#757575"
							/>
						</g>
					</svg>
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<div class="flex mb-2 items-center gap-x-2">
					<h2 class="font-poppin text-left font-medium">Confirm Password</h2>
					{#if !password}
						<p class="text-red-500 text-[12px]">* Confirm your password</p>
					{/if}
				</div>
				<div
					class="relative flex items-center w-[451px] h-[45px] rounded-lg overflow-hidden border-2 border-black bg-grayTealinux"
				>
					{#if passwordConfirmVisible}
						<input
							class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
							type="text"
							bind:value={confirmPassword}
							placeholder="Confirm your password"
						/>
					{:else}
						<input
							class="peer h-full w-full outline-hidden text-sm text-black text-opacity-70 placeholder:text-black placeholder:text-opacity-40 pr-2 pl-[12px] bg-transparent"
							type="password"
							bind:value={confirmPassword}
							placeholder="Confirm your password"
						/>
					{/if}
					<svg
						class="mr-[16px]"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						on:click={togglePasswordConfirmVisibility}
						xmlns="http://www.w3.org/2000/svg"
					>
						<mask
							id="mask0_461_173"
							style="mask-type:luminance"
							maskUnits="userSpaceOnUse"
							x="0"
							y="0"
							width="24"
							height="24"
						>
							<path d="M24 0H0V24H24V0Z" fill="white" />
						</mask>
						<g mask="url(#mask0_461_173)">
							<path
								d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"
								fill="#757575"
							/>
						</g>
					</svg>
				</div>
				{#if passwordsMatch === false && password}
					<p class="text-red-500 text-[14px] mt-[5px]">Passwords do not match</p>
				{/if}
			</div>

			<!-- <div class="mt-[15px] flex items-center"> -->
			<!-- 	<input -->
			<!-- 		type="checkbox" -->
			<!-- 		class="before:content[''] peer relative h-8 w-8 cursor-pointer appearance-none rounded-full border border-gray-900/20 bg-gray-900/10 transition-all before:absolute before:top-2/4 before:left-2/4 before:block before:h-12 before:w-12 before:-translate-y-2/4 before:-translate-x-2/4 before:rounded-full before:bg-blue-gray-500 before:opacity-0 before:transition-opacity checked:border-gray-900 checked:bg-gray-900 checked:before:bg-gray-900 hover:scale-105 hover:before:opacity-0" -->
			<!-- 	/> -->
			<!-- 	<h3 class="ml-[10px] text-[16px] text-black">Make this user administrator</h3> -->
			<!-- </div> -->
		</form>
		<div
			class="max-w-md mx-auto mt-30 mt-[68px] h-[15dvh] fixed bottom-0 bg-whiteTealinux flex items-center"
		>
			<div class="grid grid-cols-2 gap-[295px]">
				<a
					href="/installation/locale"
					class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2"
					>Back</a
				>
				<a
					href="/installation/partition"
					on:click={handleSetAccount}
					class="text-white bg-greenTealinux {passwordsMatch && fullname && username && hostname
						? ''
						: ' brightness-75 pointer-events-none'}  focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-hidden"
					>Next</a
				>
			</div>
		</div>
	</section>
</div>
