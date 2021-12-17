use ergoquad_2d::prelude::*;

use std::f32::consts::TAU;

fn window_conf() -> Conf {
    Conf {
        window_title: "WASD/Drag to move, Scroll to zoom, QE to rotate.".to_owned(),
        window_width: 512,
        window_height: 512,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

// TODO
#[macroquad::main(window_conf)]
async fn main() {
    // resource folder
    set_pc_assets_folder("examples/assets");
    // font
    let font = load_ttf_font("VarelaRound-Regular.ttf")
        .await
        .expect("rip varela round");

    // gl for transforms
    let gl = unsafe { get_internal_gl().quad_gl };
    // camera for canvases
    let cam = &mut Camera2D::default();
    cam.zoom = vec2(1.0, -1.0);
    set_camera(cam);

    let mut mouse = mouse_position_local();
    let mut mouse_prev;

    // initialize canvases
    let outer = new_canvas(512, 512);
    #[allow(unused_assignments)]
    let [mut x, mut y, mut rot] = [0.0; 3];
    let mut zoom = 1.0;
    let inner = new_canvas(128, 128);

    // main loop
    loop {
        // nearly every macroquad function uses f32 instead of f64 because that's what `Mat4`s are made of
        let time = get_time() as f32;
        // for some reason this uses f32s already
        let delta = get_frame_time();

        // check mouse
        // mouse goes downwards, while transforms go upwards
        mouse_prev = mouse;
        mouse = mouse_position_local();
        let mouse_delta = mouse - mouse_prev;

        // scroll goes up, transforms zoom in
        let (_scroll_x, scroll_y) = mouse_wheel();
        {
            // zoom
            let scroll_sens = 0.25;
            zoom *= (2_f32).powf(scroll_y * scroll_sens);

            // drag controls
            if is_mouse_button_down(MouseButton::Left) {
                x += mouse_delta.x;
                y += mouse_delta.y;
            }
        }

        // check keypresses
        {
            use KeyCode::*;
            // Quit on Esc
            if let Some(Escape) = get_last_key_pressed() {
                return;
            }

            // WASD movement, y goes down
            if is_key_down(W) {
                y -= delta;
            }
            if is_key_down(S) {
                y += delta;
            }
            if is_key_down(A) {
                x -= delta;
            }
            if is_key_down(D) {
                x += delta;
            }

            // rotation, clockwise
            let sensitivity = 0.5;
            if is_key_down(Q) {
                rot -= delta * sensitivity;
            }
            if is_key_down(E) {
                rot += delta * sensitivity;
            }
        }

        clear_background(DARKBLUE);
        paint_canvas(outer, cam, |cam| {
            clear_background(DARKGREEN);
            paint_canvas(inner, cam, |_| {
                clear_background(DARKBROWN);
                let params = TextParams {
                    font,
                    font_size: 64,
                    font_scale: 1.0 / 256.0,
                    color: ORANGE,
                    ..Default::default()
                };
                draw_text_ex("Sample Text", -0.75, 0.0, params);
                transform(gl, rotate_cw(time / 3.0 * TAU), |_| {
                    draw_line(0.0, 0.0, 0.0, 1.0, 0.25 * 0.25, BLUE);
                })
            });

            // arbitrary
            let rotate = rotate_cw(time / 5.0 * TAU);
            let shift = shift((time * 2.0).sin() / 2.0, 0.0);

            // notice the difference in order of rotation and translation
            transform(gl, rotate * shift, |gl| draw_canvas(inner, gl));
            transform(gl, shift * rotate, |gl| draw_canvas(inner, gl));
            // comment one out to find out which is which
        });

        draw_canvas(outer, gl);
        let params = TextParams {
            font_size: 64,
            font_scale: 1.0 / 512.0,
            font_scale_aspect: 1.0,
            color: YELLOW,
            ..Default::default()
        };

        // draw outer layer
        let mut outer_tl_corner = Default::default();
        let mut mouse_transformed = Default::default();
        transform(
            gl,
            shift(x, y) * upscale(zoom) * rotate_cw(rot * TAU),
            |gl| {
                draw_canvas(outer, gl);
                let params = TextParams {
                    font_size: 64,
                    font_scale: 1.0 / 512.0,
                    font_scale_aspect: 1.0,
                    color: YELLOW,
                    ..Default::default()
                };

                let text = format!("Transform: {:#?}", gl.model());
                let mut y = -0.25;
                for chunk in text.split('\n') {
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

                draw_text_ex(&format!("Mouse X: {}", mouse.x), -0.375, 0.125, params);
                draw_text_ex(&format!("Mouse Y: {}", mouse.y), -0.375, -0.125, params);
                outer_tl_corner = gl.model().transform_point3(vec3(-1.0, -1.0, 0.0));

                mouse_transformed = gl
                    .model()
                    .inverse()
                    .transform_point3(vec3(mouse.x, mouse.y, 0.0));
            },
        );

        draw_circle(outer_tl_corner.x, outer_tl_corner.y, 1.0 / 64.0, WHITE);
        draw_circle(mouse_transformed.x, mouse_transformed.y, 1.0 / 64.0, WHITE);

        // end frame
        next_frame().await
    }
}
