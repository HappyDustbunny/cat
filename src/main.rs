use nannou::prelude::*;

fn main() {
    nannou::app(model)
                .event(event)
                .update(update)
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
    .key_pressed(key_pressed)
    .with_fullscreen(monitor)
    .with_title("Fern")
    .view(view)
    .build()
    .unwrap();
    Model { _window }
}

fn current_monitor() {
    MonitorID
}

fn event(_app: &App, _model: &mut Model, _update: Update) {
    nannou::event::Event::WindowEvent {
        id: _, raw: _, simple: _,
    }
}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    if let KeyPressed(_key);
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {

}


fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();

    draw.background().rgba(0.0, 0.0, 0.0, 0.0);

    draw.ellipse().x_y(100.0, 100.0).radius(1.0).color(RED);

    draw.to_frame(app, frame).unwrap();
}
