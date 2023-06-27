<script>
	export let value = '';
	export let minRows = 1;
	export let maxRows;

    export let placeholder = "";
    export let name = "";
    export let required = false;
	
	$: minHeight = `${1 + minRows * 1.2}em`;
	$: maxHeight = maxRows ? `${1 + maxRows * 1.2}em` : `auto`;
</script>

<div class="container">
	<pre
		aria-hidden="true"
		style="min-height: {minHeight}; max-height: {maxHeight}"
	>{value + '\n'}</pre>

	<textarea 
        bind:value
        placeholder={placeholder}
        name={name}
		class="textinput"
        required={required}
    ></textarea>	
</div>

<style lang="scss">
	.container {
		position: relative;
	}
	
	pre, textarea {
		font-family: inherit;
		padding: 0.5em;
		box-sizing: border-box;
		border: 2px solid $--c0-d15;
		border-radius: 5px;
		line-height: 1.2;
		overflow: hidden;
	}
	
	textarea {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		resize: none;
	}

	textarea:focus {
		border-color: $--c1;
	}
</style>