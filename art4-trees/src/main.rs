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
    let h = window.h();
    let sd = Box::new(move |point: &Point2| vec2(point.x, h));
    Model {
        height: window.h(),
        width: window.w(),
        mouse_position: None,
        delay: 0,
        tree: Tree::new(window.mid_bottom()+vec2(0.0, 1.0), Hsl::new(random_range(100.0, 140.0), 1.0, 0.5)),
        sun_direction: sd,
    }
}

const UPDATE_DELAY: u128 = 1200;
/// Model update
fn update(app: &App, model: &mut Model, _update: Update) {
    let delay = app.duration.since_prev_update.as_millis() + model.delay;

    if delay > UPDATE_DELAY {
        let trunk: &mut Branch = &mut model.tree.trunk;
        trunk.advance(1.0 * app.duration.since_start.as_secs_f32(), &model.sun_direction);
    } else {
        model.delay = delay;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() == 0 {
        frame.clear(BLACK);
    }

    let draw = app.draw();
    model.tree.trunk.draw(&draw);

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
