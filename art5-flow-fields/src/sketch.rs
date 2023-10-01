use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};

#[derive(Debug)]
pub struct Triangle {
    pub a: Point2,
    pub b: Point2,
    pub c: Point2,
    pub color: Hsl,
}
#[derive(Debug)]
pub struct Model {
    pub height: f32,
    pub width: f32,
    pub mouse_position: Option<Point2>,
    // other params
    pub noise: Perlin,
    pub triangles: Vec<Triangle>,
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
        .title("art5-flow-fields")
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

    let noise = Perlin::default();

    let w = window.w() * 0.5 * 1.2;
    let h = window.h() * 0.5 * 1.2;
    let triangle_side = 20.0;
    let mut triangles = Vec::new();
    let mut i = -w;
    while i <= w {
        let mut j = -h;
        while j <= h {
            let a = pt2(i, j);
            let b = pt2(i + triangle_side, j);
            let c = pt2(i, j + triangle_side);
            let color = hsl(
                map_range(
                    NoiseFn::get(
                        &noise,
                        [
                            (i + random_range(0.15,0.25) * triangle_side).to_f64().unwrap() * 0.01,
                            (j + random_range(0.15,0.25) * triangle_side).to_f64().unwrap() * 0.01,
                            0.3,
                        ],
                    )
                    .to_f32()
                    .unwrap(),
                    -1.0,
                    1.0,
                    0.0,
                    1.0,
                ),
                0.7,
                0.1,
            );
            triangles.push(Triangle { a, b, c, color });

            let a = pt2(i + triangle_side, j + triangle_side);
            let b = pt2(i + triangle_side, j);
            let c = pt2(i, j + triangle_side);
            let color = hsl(
                map_range(
                    NoiseFn::get(
                        &noise,
                        [
                            (i + random_range(0.75,0.85) * triangle_side).to_f64().unwrap() * 0.01,
                            (j + random_range(0.75,0.85) * triangle_side).to_f64().unwrap() * 0.01,
                            0.7,
                        ],
                    )
                    .to_f32()
                    .unwrap(),
                    -1.0,
                    1.0,
                    0.0,
                    1.0,
                ),
                0.7,
                0.7,
            );
            triangles.push(Triangle { a, b, c, color });
            j += triangle_side;
        }
        i += triangle_side;
    }

    Model {
        height: window.h(),
        width: window.w(),
        mouse_position: None,
        noise,
        triangles,
    }
}

/// Model update
fn update(_app: &App, model: &mut Model, _update: Update) {

    let compute_delta = |p: Point2|{
        
        let x = map_range(
            NoiseFn::get(
                &model.noise,
                [
                    p.x.to_f64().unwrap() * 0.01,
                    p.y.to_f64().unwrap() * 0.01,
                    0.0,
                ],
            )
            .to_f32()
            .unwrap(),
            -1.0,
            1.0,
            -0.1,
            0.1,
        );
        let y = map_range(
            NoiseFn::get(
                &model.noise,
                [
                    p.x.to_f64().unwrap() * 0.01,
                    p.y.to_f64().unwrap() * 0.01,
                    1.0,
                ],
            )
            .to_f32()
            .unwrap(),
            -1.0,
            1.0,
            -0.1,
            0.1,
        );
        pt2(x, y)
    };

    for t in model.triangles.iter_mut() {
        t.a += compute_delta(t.a);
        t.b += compute_delta(t.b);
        t.c += compute_delta(t.c);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    for t in &model.triangles {
        draw.tri().color(t.color).points(t.a, t.b, t.c).finish();
    }

    draw.text(&format!("f: {}", app.fps()))
        .font_size(32)
        .color(BLACK)
        .align_text_top()
        .y(model.height * 0.5)
        .finish();
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}

/// Event handler
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => model.mouse_position = Some(pos),
        MousePressed(_) => {}
        _ => (),
    }
}
