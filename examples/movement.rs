use ergoquad::prelude::*;

use std::f32::consts::TAU;

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
    set_pc_assets_folder("examples/assets");
    let font = load_ttf_font("VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");
    // gl for transforms
    let gl = unsafe { get_internal_gl().quad_gl };
    // camera for canvases
    let cam = &mut Camera2D::default();

    set_camera(cam);

    // initialize canvases
    let outer = new_canvas(512, 512);
    #[allow(unused_assignments)]
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
                transform(gl, flip_y(), |_gl| {
                    let params = TextParams {
                        font,
                        font_size: 64,
                        font_scale: 1.0 / 256.0,
                        color: ORANGE,
                        ..Default::default()
                    };
                    draw_text_ex("Sample Text", -0.75, 0.0, params);
                });
                transform(gl, rotate_z(time / 3.0 * TAU), |_| {
                    draw_line(0.0, 0.0, 0.0, 1.0, 0.25 * 0.25, BLUE);
                })
            });

            // arbitrary
            let rotate = rotate_z(time / 5.0 * TAU);
            let shift = shift((time * 2.0).sin() / 2.0, 0.0);

            // notice the difference in order of rotation and translation
            transform(gl, rotate * shift, |gl| draw_canvas(inner, gl));
            transform(gl, shift * rotate, |gl| draw_canvas(inner, gl));
            // comment one out to find out which is which
        });
        // draw rotating outer layer
        // https://en.wikipedia.org/wiki/Aircraft_principal_axes
        // if objects face the screen, positive is pitch down yaw your left roll counterclockwise (XYZ)
        let mut outer_bl_corner = Default::default();
        transform(
            gl,
            upscale(zoom)
                * shift(x, y)
                * rotate_x(pitch * TAU)
                * rotate_y(-yaw * TAU)
                * rotate_z(-roll * TAU),
            |gl| {
                draw_canvas(outer, gl);
                transform(gl, flip_y() * closer(0.125), |gl| {
                    let params = TextParams {
                        font_size: 64,
                        font_scale: 1.0 / 512.0,
                        font_scale_aspect: 1.0,
                        color: YELLOW,
                        ..Default::default()
                    };

                    transform(gl, further(1.0 / 16.0), |gl| {
                        let text = format!("Transform: {:#?}", gl.model());
                        let mut y = -0.25;
                        for chunk in text.split('\n')
                        // .as_bytes()
                        // .chunks(16)
                        // .map(|chunk| chunk.into_iter().map(|&b| b as char).collect::<String>())
                        {
                            draw_text_ex(
                                &chunk,
                                -0.75,
                                y,
                                TextParams {
                                    font_scale: 1.0 / 1024.0,
                                    ..params
                                },
                            );
                            y += 1.0 / 16.0;
                        }
                    });

                    draw_text_ex(&format!("Mouse X: {}", mouse.x), -0.375, 0.125, params);
                    draw_text_ex(&format!("Mouse Y: {}", mouse.y), -0.375, -0.125, params);
                });
                outer_bl_corner = gl.model().transform_point3(vec3(-0.5, -0.5, 0.0));
            },
        );

        draw_circle(outer_bl_corner.x, outer_bl_corner.y, 1.0 / 64.0, WHITE);

        // end frame
        next_frame().await
    }
}
