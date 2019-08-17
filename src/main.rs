use nannou::prelude::*;

fn main() {
    nannou::app(model)
                // .event(event)
                // .update(update)
                .view(view)
                .run();
}

struct Model {
    // Rhombe corners
    top_pt:  Vector2,
    bottom_pt: Vector2,
    right_pt:  Vector2,
    left_pt:  Vector2,
    mid_top_right_pt:  Vector2,
    mid_top_left_pt:  Vector2,
    mid_bottom_right_pt:  Vector2,
    mid_bottom_left_pt:  Vector2,
    center_pt:  Vector2,

    _window: WindowId,
}

fn model(app: &App) -> Model {
    let _window = app
    .new_window()
    .with_transparency(true)
    .event(event)
    // .key_pressed(key_pressed)
    // .with_fullscreen(monitor)
    // .with_title("Fern")
    // .view(view)
    .build()
    .unwrap();

    let top_pt = Vector2::new(100.0, 50.0);
    let bottom_pt = Vector2::new(100.0, -50.0);
    let right_pt = Vector2::new(0.0, 0.0);
    let left_pt = Vector2::new(200.0, 0.0);
    let mid_top_right_pt = Vector2::new(150.0, 25.0);
    let mid_top_left_pt = Vector2::new(50.0, 25.0);
    let mid_bottom_right_pt = Vector2::new(150.0, -25.0);
    let mid_bottom_left_pt = Vector2::new(50.0, -25.0);
    let center_pt = Vector2::new(100.0, 0.0);

    Model {top_pt,
        bottom_pt,
        right_pt,
        left_pt,
        mid_top_right_pt,
        mid_top_left_pt,
        mid_bottom_right_pt,
        mid_bottom_left_pt,
        center_pt,

        _window }
}
//
// fn draw_rhombe(draw: &app::Draw, model: &mut Model) {
//     // let draw = app.draw();
//
//     draw.line().start(model.top_pt).end(model.right_pt).color(RED);
//     draw.line().start(model.right_pt).end(model.bottom_pt).color(RED);
//     draw.line().start(model.bottom_pt).end(model.left_pt).color(RED);
//     draw.line().start(model.left_pt).end(model.top_pt).color(RED);
//     draw.line().start(model.mid_top_right_pt).end(model.mid_bottom_left_pt).color(RED);
//     draw.line().start(model.mid_bottom_right_pt).end(model.mid_top_left_pt).color(RED);
//
//     println!("rap");
// }

fn contract_to(model: &mut Model, left_point: Vector2) {
    let new_top = left_point + (model.top_pt - left_point) / 2.0;
    let new_bottom = left_point + (model.bottom_pt - left_point) / 2.0;
    let new_right = left_point + (model.right_pt - left_point) / 2.0;

    model.left_pt = left_point;
    model.right_pt = new_right;
    model.top_pt = new_top;
    model.bottom_pt = new_bottom;
    model.mid_top_left_pt = left_point + (new_top - left_point) / 2.0;
    model.mid_top_right_pt = new_right + (new_top - new_right) / 2.0;
    model.mid_bottom_left_pt = left_point + (new_bottom - left_point) / 2.0;
    model.mid_bottom_right_pt = new_right + (new_bottom - new_right) / 2.0;
    model.center_pt = left_point + (new_right - left_point) / 2.0;

    // draw_rhombe(draw, model);
}

// fn current_monitor() {
//     MonitorID
// }
//
fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {
            _app.main_window().set_cursor_position(100, 200).unwrap();
        }
        KeyReleased(_key) => {
            contract_to(_model, _model.left_pt);
            _app.main_window().set_cursor_position(200, 100).unwrap();
        }
        // // Mouse events
        // MouseMoved(_pos) => {}
        // MousePressed(_button) => {}
        // MouseReleased(_button) => {}
        // MouseWheel(_amount, _phase) => {}
        // MouseEntered => {}
        // MouseExited => {}
        //
        // // Touch events
        // Touch(_touch) => {}
        // TouchPressure(_pressure) => {}
        //
        // // Window events
        // Moved(_pos) => {}
        // Resized(_size) => {}
        // HoveredFile(_path) => {}
        // DroppedFile(_path) => {}
        // HoveredFileCancelled => {}
        // Focused => {}
        // Unfocused => {}
        // Closed => {}
        _other => {}
    }

}

// fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
//     if let KeyPressed(_key);
// }
//
// fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
//     _app.main_window().set_cursor_position(100, 200).unwrap();
//
// }


fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(rgb(0.0, 0.0, 0.0));

    draw.line().start(model.top_pt).end(model.right_pt).color(RED);
    draw.line().start(model.right_pt).end(model.bottom_pt).color(RED);
    draw.line().start(model.bottom_pt).end(model.left_pt).color(RED);
    draw.line().start(model.left_pt).end(model.top_pt).color(RED);
    draw.line().start(model.mid_top_right_pt).end(model.mid_bottom_left_pt).color(RED);
    draw.line().start(model.mid_bottom_right_pt).end(model.mid_top_left_pt).color(RED);

    // draw.background().color(rgba(0.0, 0.0, 0.0, 0.05));
    //
    // draw.ellipse().x_y(0.0, 0.0).radius(150.0).color(rgba(1.0, 0.0, 0.0, 0.95));
    // draw.ellipse().x_y(100.0, 100.0).radius(150.0).color(rgba(0.0, 1.0, 0.0, 0.05));

    draw.to_frame(app, &frame).unwrap();
}
