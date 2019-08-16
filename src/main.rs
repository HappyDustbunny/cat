use nannou::prelude::*;

fn main() {
    nannou::app(model)
                // .event(event)
                // .update(update)
                .view(view)
                .run();
}

struct Model {
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
    Model { _window }
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
            _app.main_window().set_cursor_position(200, 100).unwrap();
        }
        // Mouse events
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseWheel(_amount, _phase) => {}
        MouseEntered => {}
        MouseExited => {}

        // Touch events
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}

        // Window events
        Moved(_pos) => {}
        Resized(_size) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
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


fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().color(rgba(0.0, 0.0, 0.0, 0.05));
    //
    // draw.ellipse().x_y(0.0, 0.0).radius(150.0).color(rgba(1.0, 0.0, 0.0, 0.95));
    // draw.ellipse().x_y(100.0, 100.0).radius(150.0).color(rgba(0.0, 1.0, 0.0, 0.05));

    draw.to_frame(app, &frame).unwrap();
}
