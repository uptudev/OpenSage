<script>
    let title = "My Garden";
    let init_props = {
        show_field: true,
        show_details: true,
    };

    import Header from '../components/Header.svelte';
    import Container from '../components/Container.svelte';
</script>

<div id="app">
    <Header title={title} props={init_props}/>
    <Container props={init_props} />
</div>

<style lang="scss">
    @import '../stylelibs/colours.scss';

    :global(body) {
        margin: 0 !important;
        background-color: $darkgrey; 
    }
</style>
