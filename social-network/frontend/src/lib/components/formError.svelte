<script>
    export let error = "";
</script>

<div class="error-message-container">
    <p>{error}</p>
</div>

<style lang="scss">
    .error-message-container {
        background: $--c0-t75;
        border: $--border-width solid $--color-error;
        box-shadow: 0 0 15px -10px $--c1;
        border-radius: $--border-radius;
    }

    .error-message-container > p {
        font-size: 1.2rem;
        font-weight: bold;
        text-align: center;
        color: $--color-error;
    }
</style>
