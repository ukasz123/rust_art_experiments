use nannou::prelude::*;

mod model;
use model::*;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

/// creates a initial [Model] instance.
fn model(app: &App) -> Model {
    let window = app.window_rect();

    Model {
        height: window.h(),
        width: window.w(),
        mouse_position: None,
    }
}

/// Model update
fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    if app.elapsed_frames() == 1 {
        frame.clear(BLACK);
    }

    let draw = app.draw();
    draw.text("Hello, {{project-name}}!")
        .color(hsl(
            (app.elapsed_frames().to_f32().unwrap() / 180.0).fract(),
            1.0,
            0.5,
        ))
        .font_size(36);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}

/// Event handler
fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            MouseMoved(pos) => model.mouse_position = Some(pos),
            MousePressed(_) => {
                println!("Mouse pressed at {:?}", model.mouse_position);
            }
            _ => (),
        },
        _ => (),
    }
}
