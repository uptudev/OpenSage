<script>
    import { onMount } from "svelte";
    import init, {test} from "./pkg/wasm_canvas.js";

    const CANVAS_ID = "triangle";

    onMount(() => {
        run();

        const colorChangerForm = document.getElementById("color-changer");
        colorChangerForm.addEventListener("submit", (e) => {
            e.preventDefault();

            const color = [
                clampRGBValue(e.target.elements.red.value),
                clampRGBValue(e.target.elements.green.value),
                clampRGBValue(e.target.elements.blue.value),
                1.0,
            ];

            test(CANVAS_ID, color);
        });

    });

    async function run() {
        await init();
        const color = [1.0, 0.0, 0.0, 1.0];
        test(CANVAS_ID, color);
    }
    
    function clampRGBValue(value) {
        return parseFloat((parseFloat(value) / 255 || 0).toFixed(2));
    }
</script>

<head>
    <meta charset="UTF-8" />
    <title>WebGL with Rust + WebAssembly</title>
  </head>
  <body>
    <canvas id="triangle" width="400" height="400"> </canvas>

    <form id="color-changer">
      <input
        type="number"
        id="red"
        name="color"
        placeholder="Red"
        value="23"
        min="0"
        max="255"
      />
      <input
        type="number"
        id="green"
        name="color"
        placeholder="Green"
        value="22"
        min="0"
        max="255"
      />
      <input
        type="number"
        id="blue"
        name="color"
        placeholder="Blue"
        value="243"
        min="0"
        max="255"
      />
      <input type="submit" value="Change color" />
    </form>
  </body>
