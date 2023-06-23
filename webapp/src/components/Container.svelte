<script>
    import Details from './DetailsContainer.svelte'
    import Field from './FieldContainer.svelte'
    export let props;

    function updateProps(show_field, show_details) {
        props = {
            show_field,
            show_details,
        }
    }

    function toggleDetails() {
        props.show_details = !props.show_details;
    }

    function toggleField() {
        props.show_field = !props.show_field;
    }

</script>
<div>
<button on:click={toggleField}>Toggle Field</button>
<button on:click={toggleDetails}>Toggle Details</button>
<div id="container" class="container">
    {#if props.show_field}
        <Field />
    {/if}

    {#if props.show_details}
        <Details />
    {/if}
</div>
</div>
<style lang="scss">
    
    .container {
        display: flex;
        flex-direction: row;
        height: calc(100vh - 40px);
        width: 100vw;
    }

</style>
