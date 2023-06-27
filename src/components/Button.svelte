<script lang="ts">
	export let type: string;
	export let disabled = false;
</script>

<button class="button {type}" {disabled} on:click>
	<slot />
</button>

<style lang="scss">
	.button {
		font-size: 0.9rem;
		font-weight: 500;
		color: #eee;
		border: none;
		outline: none;
		padding: 10px 25px;
		border-radius: 4px;
		box-shadow: rgba(0, 0, 0, 0.24) 0px 3px 8px;

		&.primary {
			background: #206f98;
		}

		&.secondary {
			background: #353535;
		}
	}

	.button:not(:disabled) {
		cursor: pointer;

		&.primary:hover {
			background: #267faf;
		}

		&.secondary:hover {
			background: #3f3f3f;
		}
	}
</style>
