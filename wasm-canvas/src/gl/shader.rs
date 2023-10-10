/* Shaders */

use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
extern crate js_sys;

/// A slightly more ergonomic solution than calling 
/// `WebGlRenderingContext::FRAGMENT_SHADER` with every shader creation.
///
/// Represents the shader type of the wanted shader.
pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

/// Creates a shader in the given WebGL context. This must still be linked to 
/// the context via the `gl.attach_shader()` function *(see example)*. 
///
/// # Arguments
///
/// * `gl` - The target WebGL rendering context, retrieved via 
/// [`init_webgl_context()`].
/// * `shader_type` - The [`ShaderType`], either [`ShaderType::VERTEX`] or 
/// [`ShaderType::FRAGMENT`].
/// * `source` - The source code of the shader as a string slice.
///
/// # Example
///
/// ```
/// let vertex_shader = create_shader(
///     &gl,
///     ShaderType::VERTEX,
///     vertex_shader_source,
/// )
/// .unwrap();
/// let fragment_shader = create_shader(
///     &gl,
///     ShaderType::FRAGMENT,
///     fragment_shader_source,
/// )
/// .unwrap();
///
/// let shader_program = gl.create_program().unwrap();
/// gl.attach_shader(&shader_program, &vertex_shader);
/// gl.attach_shader(&shader_program, &fragment_shader);
/// gl.link_program(&shader_program);
/// ```
pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: ShaderType,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    // Match the `ShaderType` enum to the corresponding WebGlRenderingContext 
    // u32 value, permashadowing the variable in the process.
    let shader_type: u32 = match shader_type {
        ShaderType::VERTEX => WebGlRenderingContext::VERTEX_SHADER,
        ShaderType::FRAGMENT => WebGlRenderingContext::FRAGMENT_SHADER,
    };

    // Create the shader object.
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    // Set the source code of the shader object to be the input argument and 
    // compile to a WebGL shader.
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    
    // Ensure the compilation passed successfully, returning the shader or an 
    // error, depending on the result.
    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(JsValue::from_str(
            &gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into()),
        ))
    }
}

/// Sets up basic shaders for simple solid-colour geometry.
///
/// # Arguments
///
/// * `gl` - The WebGL context.
pub fn setup_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {
    let vertex_shader_source = "
        attribute vec3 coordinates;
        void main(void) {
            gl_Position = vec4(coordinates, 1.0);
        }
    ";

    let fragment_shader_source = "
        precision mediump float;
        uniform vec4 fragColor;
        void main(void) {
            gl_FragColor = fragColor;
        }
    ";

    let vertex_shader = create_shader(
        &gl,
        ShaderType::VERTEX,
        vertex_shader_source,
    ).unwrap();

    let fragment_shader = create_shader(
        &gl,
        ShaderType::FRAGMENT,
        fragment_shader_source,
    ).unwrap();

    let shader_program = gl.create_program().unwrap();
    gl.attach_shader(&shader_program, &vertex_shader);
    gl.attach_shader(&shader_program, &fragment_shader);
    gl.link_program(&shader_program);

    if gl
        .get_program_parameter(&shader_program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        gl.use_program(Some(&shader_program));
        Ok(shader_program)
    } else {
        return Err(JsValue::from_str(
            &gl.get_program_info_log(&shader_program)
                .unwrap_or_else(|| "Unknown error linking program".into()),
        ));
    }
}

/// Sets up vertices into a WebGL buffer array.
///
/// # Arguments
///
/// * `gl` - The WebGL context.
/// * `vertices` - The array of vertices to draw, as triplet XYZ coordinates.
/// * `shader_program` - The shader program to add the buffer to.
pub fn setup_vertices(
    gl: &WebGlRenderingContext,
    vertices: &[f32],
    shader_program: &WebGlProgram
) {
    let vertices_array = unsafe { js_sys::Float32Array::view(&vertices) };
    let vertex_buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    let coordinates_location = 
        gl.get_attrib_location(&shader_program, "coordinates");

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.vertex_attrib_pointer_with_i32(
        coordinates_location as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    gl.enable_vertex_attrib_array(coordinates_location as u32);
}
