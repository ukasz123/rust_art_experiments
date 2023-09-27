use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

#[derive(Debug)]
pub struct Model {
    pub height: f32,
    pub width: f32,
    pub mouse_position: Option<Point2>,
    // other params
}

pub async fn run_app() {
    app::Builder::new_async(|app| {
        Box::new(async move {
            create_window(app).await;
            model(app)
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    .update(update)
    .run_async()
    .await;
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .title("{{project-name}}")
        .event(event)
        // .key_pressed(key_pressed)
        // .key_released(key_released)
        // .mouse_pressed(mouse_pressed)
        // .mouse_moved(mouse_moved)
        // .mouse_released(mouse_released)
        // .mouse_wheel(mouse_wheel)
        // .touch(touch)
        .view(view)
        .build_async()
        .await
        .unwrap();
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
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => model.mouse_position = Some(pos),
        MousePressed(_) => {
            println!("Mouse pressed at {:?}", model.mouse_position);
        }
        _ => (),
    }
}
