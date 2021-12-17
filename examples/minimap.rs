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

// TODO: Fix orientation, currently flipped upside-down.
// Note: Do not change the camera's zoom. This will ripple across canvases.
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
    set_camera(cam);

    let mut mouse = mouse_position_local();
    let mut mouse_prev;

    // initialize canvases
    let minimap = new_canvas(512, 512);
    let object = new_canvas(128, 128);
    let [mut x, mut y, mut rot] = [0.75, 0.75, 0.0];
    let mut zoom = 0.25;

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
            let sensitivity = TAU / 2.0; // no i will not use pi
            if is_key_down(Q) {
                rot -= delta * sensitivity;
            }
            if is_key_down(E) {
                rot += delta * sensitivity;
            }
        }

        paint_canvas(minimap, cam, |cam| {
            clear_background(DARKGREEN);
            draw_line(0.0, 0.0, 0.0, 1.0, 1.0 / 32.0, MAGENTA);
            paint_canvas(object, cam, |_| {
                clear_background(DARKBROWN);
                let params = TextParams {
                    font,
                    font_size: 64,
                    font_scale: 1.0 / 256.0,
                    color: ORANGE,
                    ..Default::default()
                };
                draw_text_ex("Sample Text", -0.75, 0.0, params);
                apply(gl, rotate_cw(time / 3.0 * TAU), |_| {
                    draw_line(0.0, 0.0, 0.0, 1.0, 0.25 * 0.25, BLUE);
                })
            });

            // arbitrary
            let rotate = rotate_cw(time / 5.0 * TAU);
            let shift = shift((time * 2.0).sin() / 2.0, 0.0);

            apply(gl, downscale(2.0), |gl| {
                // notice the difference in order of rotation and translation
                apply(gl, rotate * shift, |gl| draw_canvas(object, gl, 1.0));
                apply(gl, shift * rotate, |gl| draw_canvas(object, gl, 1.0));
                // comment one out to find out which is which
            });
            draw_multiline_text(
                &format!("Mouse X: {}\nMouse Y: {}", mouse.x, mouse.y),
                -1.0 + 1.0 / 32.0,
                -1.0 + 1.0 / 8.0,
                0.125,
                TextParams {
                    font_size: 64,
                    font_scale: 1.0 / 512.0,
                    font_scale_aspect: 1.0,
                    color: YELLOW,
                    ..Default::default()
                },
            );
        });

        // draw map
        draw_canvas(minimap, gl, 1.0);
        let params = TextParams {
            font_size: 64,
            font_scale: 1.0 / 512.0,
            font_scale_aspect: 1.0,
            color: YELLOW,
            ..Default::default()
        };
        draw_rectangle_lines(-1.0, -1.0, 2.0, 2.0, 1.0 / 32.0, RED);

        // draw minimap
        let minimap_transform = shift(x, y) * rotate_cw(rot) * upscale(zoom);
        apply(gl, minimap_transform, |gl| {
            draw_canvas(minimap, gl, 0.5);
            draw_rectangle_lines(-1.0, -1.0, 2.0, 2.0, 1.0 / 32.0, YELLOW);
        });

        draw_multiline_text(
            &format!("Mouse X: {}\nMouse Y: {}", mouse.x, mouse.y),
            -1.0 + 1.0 / 32.0,
            -1.0 + 1.0 / 8.0,
            0.125,
            params,
        );

        let inner_mouse = minimap_transform
            .inverse()
            .transform_point3(vec3(mouse.x, mouse.y, 0.0));
        draw_circle(inner_mouse.x, inner_mouse.y, 1.0 / 64.0, YELLOW);

        let outer_mouse = minimap_transform.transform_point3(vec3(mouse.x, mouse.y, 0.0));
        draw_circle(outer_mouse.x, outer_mouse.y, 1.0 / 64.0, RED);

        // end frame
        next_frame().await
    }
}
