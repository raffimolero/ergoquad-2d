use crate::prelude::*;

pub fn apply(gl: &mut QuadGl, matrix: Mat4, f: impl FnOnce(&mut QuadGl)) {
    gl.push_model_matrix(matrix);
    f(gl);
    gl.pop_model_matrix();
}

pub fn draw_multiline_text(text: &str, x: f32, mut y: f32, line_spacing: f32, params: TextParams) {
    for chunk in text.split('\n') {
        draw_text_ex(&chunk, x, y, params);
        y += line_spacing;
    }
}

pub type Canvas = RenderTarget;

pub fn new_canvas(w: u32, h: u32) -> Canvas {
    render_target(w, h)
}
pub fn paint_canvas(
    target: Canvas,
    camera: &mut Camera2D,
    instructions: impl FnOnce(&mut Camera2D),
) {
    let previous_target = camera.render_target.replace(target);
    set_camera(camera);
    instructions(camera);
    camera.render_target = previous_target;
    set_camera(camera);
}
pub fn draw_canvas(source: Canvas, gl: &mut QuadGl, opacity: f32) {
    apply(
        gl,
        shift(-1.0, -1.0) * downscale(source.texture.height() / 2.0),
        |_| {
            draw_texture(
                source.texture,
                0.0,
                0.0,
                Color::from_vec(vec4(1.0, 1.0, 1.0, opacity)),
            );
        },
    );
}
