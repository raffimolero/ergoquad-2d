use std::f32::consts::TAU;

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

        pub fn rotate_x(pitch: f32) -> Mat4 {
            Mat4::from_rotation_x(pitch)
        }
        pub fn rotate_y(yaw: f32) -> Mat4 {
            Mat4::from_rotation_y(yaw)
        }
        pub fn rotate_z(roll: f32) -> Mat4 {
            Mat4::from_rotation_z(roll)
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

fn window_conf() -> Conf {
    Conf {
        window_title: "title".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // resource folder
    set_pc_assets_folder("../../../Resources");
    let font = load_ttf_font("fonts/VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");
    // gl for transforms
    let gl = unsafe { get_internal_gl().quad_gl };
    // camera for canvases
    let cam = &mut Camera2D::default();

    set_camera(cam);

    // initialize canvases
    let outer = new_canvas(512, 512);
    let [mut x, mut y, mut yaw, mut pitch, mut roll] = [0.0; 5];
    let mut zoom = 1.0;
    let inner = new_canvas(128, 128);

    // main loop
    loop {
        // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
        let time = get_time() as f32;
        // for some reason this uses f32s already
        let delta = get_frame_time();

        // check mouse position
        // mouse goes downwards, while transforms go upwards
        let mouse = mouse_position_local();
        let (_scroll_x, scroll_y) = mouse_wheel();
        {
            let mouse_sens = 0.5;
            let scroll_sens = 0.25;
            yaw = (mouse.x / zoom - x) * mouse_sens;
            pitch = (-mouse.y / zoom - y) * mouse_sens;
            zoom *= (2_f32).powf(scroll_y * scroll_sens);
        }

        // check keypresses
        {
            use KeyCode::*;
            // Quit on Esc
            if let Some(Escape) = get_last_key_pressed() {
                return;
            }

            // WASD movement, y goes up
            if is_key_down(S) {
                y -= delta;
            }
            if is_key_down(W) {
                y += delta;
            }
            if is_key_down(A) {
                x -= delta;
            }
            if is_key_down(D) {
                x += delta;
            }

            // roll is stored clockwise, transforms go counterclockwise
            let sensitivity = 0.5;
            if is_key_down(Q) {
                roll -= delta * sensitivity;
            }
            if is_key_down(E) {
                roll += delta * sensitivity;
            }
        }

        // outermost layer just has a gray background
        clear_background(DARKBLUE);
        paint_canvas(outer, cam, |cam| {
            clear_background(DARKGREEN);
            paint_canvas(inner, cam, |_| {
                clear_background(DARKBROWN);
                apply_transforms(gl, &[flip_y()], |_gl| {
                    let params = TextParams {
                        font,
                        font_size: 64,
                        font_scale: 1.0 / 256.0,
                        color: ORANGE,
                        ..Default::default()
                    };
                    draw_text_ex("Sample Text", -0.75, 0.0, params);
                });
                apply_transform(gl, rotate_z(time / 3.0 * TAU), |_| {
                    draw_line(0.0, 0.0, 0.0, 1.0, 0.25 * 0.25, BLUE);
                })
            });

            // arbitrary
            let rotation = rotate_z(time / 5.0 * TAU);
            let translation = shift((time * 2.0).sin() / 2.0, 0.0);

            // notice the difference in order of rotation and translation
            apply_transforms(gl, &[rotation, translation], |gl| draw_canvas(inner, gl));
            apply_transforms(gl, &[translation, rotation], |gl| draw_canvas(inner, gl));
            // comment one out to find out which is which

            apply_transforms(gl, &[flip_y()], |_gl| {
                let params = TextParams {
                    font_size: 64,
                    font_scale: 1.0 / 512.0,
                    font_scale_aspect: 1.0,
                    color: YELLOW,
                    ..Default::default()
                };
                draw_text_ex(&format!("Mouse X: {}", mouse.x), -0.375, 0.125, params);
                draw_text_ex(&format!("Mouse Y: {}", mouse.y), -0.375, -0.125, params);
            });
        });
        // draw rotating outer layer
        // https://en.wikipedia.org/wiki/Aircraft_principal_axes
        // if objects face the screen, positive is pitch down yaw your left roll counterclockwise (XYZ)
        apply_transforms(
            gl,
            &[
                upscale(zoom),
                shift(x, y),
                rotate_x(pitch * TAU),
                rotate_y(-yaw * TAU),
                rotate_z(-roll * TAU),
            ],
            |gl| {
                draw_canvas(outer, gl);
            },
        );

        // end frame
        next_frame().await
    }
}
