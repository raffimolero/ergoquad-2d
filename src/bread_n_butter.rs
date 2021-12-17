use crate::prelude::*;

pub fn transform(gl: &mut QuadGl, matrix: Mat4, f: impl FnOnce(&mut QuadGl)) {
    gl.push_model_matrix(matrix);
    f(gl);
    gl.pop_model_matrix();
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
pub fn draw_canvas(target: Canvas, gl: &mut QuadGl) {
    transform(
        gl,
        shift(-1.0, -1.0) * downscale(target.texture.height() / 2.0),
        |_| {
            draw_texture(target.texture, 0.0, 0.0, WHITE);
        },
    );
}
