<script lang="ts">
	import Music from 'components/Music.svelte';
	import Button from 'components/Button.svelte';
	import toast from 'svelte-french-toast';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api';

	const version = '1.0-beta.1';

	let exeName = 'Unset';
	let gameName = 'Please open an executable.';

	let path = '';
	let loaded = false;

	let tracks: number[] = [];

	const toastOptions = {
		style: 'background: #333; color: #fff;'
	};

	async function openExecutable() {
		const result: string | string[] | null = await open({ multiple: false });
		if (!result) return;

		path = result as string;

		try {
			const response: string = await invoke('open', { path });
			if (response === 'Unknown') return;

			exeName = response;
			if (response === 'SCES_01000') gameName = 'Kula World';
			else if (response === 'SLUS_00724') gameName = 'Roll Away';
			else gameName = 'Kula Quest';

			let tracksRaw: Uint16Array = await invoke('get_tracks', { game: exeName, path });
			for (let i = 0; i < 16; i++) {
				tracks[i] = Number(((tracksRaw[i] * 4) / 75).toFixed(2));
			}

			toast.success('Successfully loaded executable!', toastOptions);
			loaded = true;
		} catch (err) {
			toast.error(err as string, toastOptions);
		}
	}

	async function patchExecutable() {
		const tracksRaw: number[] = [];
		for (let i = 0; i < 16; i++) {
			tracksRaw[i] = Math.floor((tracks[i] * 75) / 4);
		}

		await invoke('set_tracks', { game: exeName, path: path as string, tracks: tracksRaw })
			.then(() => toast.success('Successfully patched executable!', toastOptions))
			.catch((err) => toast.error(err, toastOptions));
	}
</script>

<main>
	<div class="main-layout">
		<div class="container">
			<div class="header">
				<div class="info">
					<h1 class="name">{exeName}</h1>
					<p class="description">
						{#if loaded}
							Game type: <span class="highlight">{gameName}</span>
						{:else}
							{gameName}
						{/if}
					</p>
				</div>
				<div class="actions">
					<Button type="secondary" on:click={openExecutable}>Open</Button>
				</div>
			</div>

			<div class="separate" />

			<Music {tracks} disabled={!loaded} />

			<div class="separate" />

			<div class="button">
				<Button type="primary" on:click={patchExecutable} disabled={!loaded}>Patch</Button>
			</div>
		</div>
	</div>
	<p class="version">Version {version}</p>
</main>

<style>
	.button {
		display: flex;
		justify-content: center;
	}

	:global(body) {
		background: #181818;
	}

	.main-layout {
		user-select: none;
		display: flex;
		flex-direction: column;
		align-items: center;
		margin-top: 20px;
		margin-inline: 20px;
	}

	.container {
		max-width: 500px;
	}

	.header {
		display: flex;
		gap: 4rem;
		justify-content: space-between;
		align-items: center;
	}

	.name {
		font-size: 1.2rem;
		color: #eee;
		margin-bottom: 5px;
	}

	.description {
		font-size: 1rem;
		color: #7d7d7d;
	}

	.highlight {
		color: #5686ce;
	}

	.version {
		font-size: 0.8rem;
		color: #474747;
		position: absolute;
		bottom: 10px;
		right: 15px;
		user-select: none;
	}

	.separate {
		border-top: 2px solid rgba(135, 135, 135, 10%);
		border-radius: 5px;
		margin-top: 25px;
		margin-bottom: 25px;
	}
</style>
