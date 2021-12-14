use macroquad::prelude::*;

use ergo::*;
/// Makes working with transforms much easier; more `ergo`nomic.
mod ergo {
    use macroquad::prelude::*;

    pub use transforms::*;
    pub mod transforms {
        use macroquad::prelude::*;
        use std::f32::consts::TAU;

        pub fn shift(x: f32, y: f32) -> Mat4 {
            Mat4::from_translation(vec3(x, y, 0.0))
        }

        pub fn flip_x() -> Mat4 {
            Mat4::from_scale(vec3(-1.0, 1.0, 1.0))
        }
        pub fn flip_y() -> Mat4 {
            Mat4::from_scale(vec3(1.0, -1.0, 1.0))
        }
        pub fn flip_xy() -> Mat4 {
            Mat4::from_scale(vec3(-1.0, -1.0, 1.0))
        }

        pub fn rotate_rad(radians: f32) -> Mat4 {
            Mat4::from_rotation_z(radians)
        }
        pub fn rotate_deg(degrees: f32) -> Mat4 {
            rotate_rad(degrees.to_radians())
        }
        pub fn rotate_turns(turns: f32) -> Mat4 {
            rotate_rad(TAU * turns)
        }

        pub fn upscale(scale: f32) -> Mat4 {
            Mat4::from_scale(vec3(scale, scale, 0.0))
        }
        pub fn downscale(scale: f32) -> Mat4 {
            upscale(1.0 / scale)
        }
    }

    pub fn apply_transform(gl: &mut QuadGl, transform: Mat4, f: impl FnOnce(&mut QuadGl)) {
        gl.push_model_matrix(transform);
        f(gl);
        gl.pop_model_matrix();
    }
    pub fn apply_transforms(gl: &mut QuadGl, transforms: &[Mat4], f: impl FnOnce(&mut QuadGl)) {
        for &transform in transforms {
            gl.push_model_matrix(transform);
        }
        f(gl);
        for _ in 0..transforms.len() {
            gl.pop_model_matrix();
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
    pub fn draw_canvas(target: Canvas, gl: &mut QuadGl) {
        apply_transforms(
            gl,
            &[shift(-0.5, -0.5), downscale(target.texture.height())],
            |_| {
                draw_texture(target.texture, 0.0, 0.0, WHITE);
            },
        );
    }
}

fn nyoom(rad: f32) -> Mat4 {
    Mat4::from_rotation_ypr(rad, rad, rad)
}

#[macroquad::main("title")]
async fn main() {
    // gl for transforms
    let gl = unsafe { get_internal_gl().quad_gl };
    // camera for canvases
    let cam = &mut Camera2D::default();
    set_camera(cam);

    // initialize canvases
    let outer = new_canvas(256, 256);
    let inner = new_canvas(64, 64);

    // main loop
    loop {
        // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
        let time = get_time() as f32;

        // outermost layer just has a gray background
        clear_background(DARKBLUE);
        paint_canvas(outer, cam, |cam| {
            clear_background(DARKGREEN);
            paint_canvas(inner, cam, |_| {
                clear_background(DARKBROWN);
                apply_transforms(gl, &[shift(-0.5, 0.0), flip_y(), downscale(256.0)], |_gl| {
                    draw_text("sample text", 0.0, 0.0, 64.0, YELLOW);
                });
                apply_transform(gl, rotate_turns(time / 3.0), |_| {
                    draw_line(0.0, 0.0, 0.0, 1.0, 0.25 * 0.25, BLUE);
                })
            });

            // arbitrary
            let rotation = rotate_turns(time / 5.0);
            let translation = shift((time * 2.0).sin() / 2.0, 0.0);

            // notice the difference in order of rotation and translation
            apply_transforms(gl, &[rotation, translation], |gl| draw_canvas(inner, gl));
            apply_transforms(gl, &[translation, rotation], |gl| draw_canvas(inner, gl));
            // comment one out to find out which is which
        });
        // draw rotating outer layer
        apply_transform(gl, nyoom(time / 8.0), |gl| draw_canvas(outer, gl));

        // end frame
        next_frame().await
    }
}
