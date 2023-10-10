/* WebGL Prelude */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
extern crate js_sys;

/// Initializes the WebGL context within a given canvas (as a plaintext id).
///
/// # Arguments
///
/// * `canvas_id` - A string slice holding the id of the target HTML canvas.
///
/// # Example
///
/// ```
/// let gl: WebGlRenderingContext = init_webgl_context(canvas_id).unwrap();
/// let shader_program: WebGlProgram = setup_shaders(&gl).unwrap();
/// 
/// let vertices: [f32; 9] = [
///     0.0, 1.0, 0.0, // top
///     -1.0, -1.0, 0.0, // bottom left
///     1.0, -1.0, 0.0, // bottom right
/// ];
///
/// setup_vertices(&gl, &vertices, &shader_program);
/// ```
pub fn init_webgl_context(canvas_id: &str)
    -> Result<WebGlRenderingContext, JsValue> {
    // Get canvas from document using `canvas_id`.
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = 
        canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Bind the WebGl context to the canvas.
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    // Set the viewport size of the WebGL context to be 100% of the canvas.
    gl.viewport(
        0,
        0,
        canvas.width().try_into().unwrap(),
        canvas.height().try_into().unwrap(),
    );

    Ok(gl)
}

