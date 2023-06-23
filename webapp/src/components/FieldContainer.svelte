<div id="field-container" class="field-container">
    <canvas id="field" class="field" />
    <canvas id="scale" class="scale" />
</div>

<style lang="scss">
    @import '../stylelibs/colours.scss';

    .field-container {
        display: flex;
        flex-direction: column;
        margin: 16px;
        width: 100%;
    }

    .field {
        background-color: $grey;
        border-radius: 16px 16px 0 0;
        border-top: 1px solid $lightgrey;
        height: 100%;
    }

    .scale {
        background-color: $green;
        border-radius: 0 0 16px 16px;
        border-top: 1px solid $lightgreen;
        border-bottom: 2px solid $darkgreen;
        height: 16px;
    }

</style>
