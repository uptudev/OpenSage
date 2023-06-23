<script>
    export let title;
    export let props;
</script>

<div id="header" class="header">
    <h1 class="header-text">{title}</h1>
</div>

<style lang="scss">
    @import '../stylelibs/colours.scss';

    .header {
        background-color: $green;
        border-radius: 0 0 16px 16px;
        border-bottom: 2px solid $darkgreen;
        box-shadow: 0 2px 2px $darkgrey;
    }

    .header-text {
        margin: 0;
        padding: 4px;
        font-size: 30px;
        text-align: center;
        line-height: 1;
        text-shadow: 0 2px 2px $darkgreen;
    }
</style>
