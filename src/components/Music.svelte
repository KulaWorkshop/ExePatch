<script lang="ts">
	export let tracks: number[];
	export let disabled: boolean;

	let xaIndex = '0';

	const trackMap: { [key: string]: number[] } = {
		'0': [2, 2, 2, 2],
		'1': [0, 3, 11, 12],
		'2': [8, 1, 5, 6],
		'3': [10, 9, 4, 7]
	};
</script>

<div class="music">
	<h2 class="title">Music</h2>
	<p class="description">Lengths of the music tracks in seconds.</p>

	<select class="select" bind:value={xaIndex} {disabled}>
		<option value="0">MUSIC_0.XA</option>
		<option value="1">MUSIC_1.XA</option>
		<option value="2">MUSIC_2.XA</option>
		<option value="3">MUSIC_3.XA</option>
	</select>
	{#if disabled}
		{#each { length: 4 } as _, i}
			<input class="input" value="Track {i + 1}" disabled />
		{/each}
	{:else}
		{#each { length: 4 } as _, i}
			<input
				class="input"
				type="number"
				pattern="^[0-9]+(\.[0-9]+)?$"
				step="0.01"
				bind:value={tracks[trackMap[xaIndex][i]]}
			/>
		{/each}
	{/if}
</div>

<style>
	.title {
		font-size: 1rem;
		color: #eee;
	}

	.description {
		font-size: 0.9rem;
		color: #7d7d7d;
		margin-top: 10px;
	}

	.select {
		background: none;
		color: #7d7d7d;
		display: block;
		margin-top: 20px;
		border: 1px solid rgba(82, 82, 82, 0.5);
		border-radius: 5px;
		box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
		width: 60%;
		padding: 10px;
	}

	.select:focus {
		outline: 1px solid rgba(82, 82, 82, 0.8);
	}

	.input {
		font-size: 0.8rem;
		padding: 10px;
		width: 60%;
		color: #7d7d7d;
		background: none;
		outline: none;
		margin-top: 20px;
		border: 1px solid rgba(82, 82, 82, 0.5);
		border-radius: 5px;
		box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;
	}

	.input:focus {
		border: 1px solid rgba(82, 82, 82, 0.8);
	}

	/* Chrome, Safari, Edge, Opera */
	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	/* Firefox */
	input[type='number'] {
		-moz-appearance: textfield;
	}
</style>
