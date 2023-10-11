use nannou::color::{Alpha, IntoLinSrgba};
use nannou::draw::primitive::path::DrawingPath;
use nannou::draw::properties::ColorScalar;
use nannou::draw::Drawing;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use shape_builder::{ShapeBuilder, ShapeBuilderFactory, ShapePath};

trait DrawSelf {
    fn draw_self(&self, draw: &Draw, scale: f32, rotation: f32);

    fn born_time(&self) -> u64 {
        0
    }
}

pub struct Flower {
    pub start: Vec2,
    pub tip: Vec2,
    pub middle: f32,
    pub thickness: f32,
    pub color: Alpha<Hsl, f32>,
    pub leaf_count: u32,
    pub born_time: u64,
}

impl DrawSelf for Flower {
    fn draw_self(&self, draw: &Draw, scale: f32, rotation: f32) {
        let color = IntoLinSrgba::into_lin_srgba(self.color);

        draw_simple_flower(
            &draw,
            self.leaf_count,
            scale,
            rotation,
            self.start,
            self.tip,
            self.middle,
            self.thickness,
            color,
        );
    }

    fn born_time(&self) -> u64 {
        self.born_time
    }
}

pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Alpha<Hsl, f32>,
}
impl DrawSelf for Circle {
    fn draw_self(&self, draw: &Draw, scale: f32, _: f32) {
        draw.ellipse()
            .xy(self.center)
            .radius(self.radius * scale)
            .color(self.color)
            .finish();
    }
}

struct Model {
    pub height: f32,
    pub width: f32,
    pub mouse_position: Option<Point2>,
    // other params
    flowers: Vec<Box<dyn DrawSelf>>,
    pub last_flower_born_time: Option<u64>,
    pub pressed: bool,
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
    // .loop_mode(LoopMode::Wait)
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
        .title("art6-flowers")
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
        flowers: vec![],
        last_flower_born_time: None,
        pressed: false,
    }
}

/// Model update
fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(WHITESMOKE);
    
    for f in model.flowers.iter() {
        let t = (app.elapsed_frames() - f.born_time()).to_f32().unwrap() * 0.1;

        let rotation = PI * 0.015 * t;
        let scale = 1.0 + t * 0.01;

        f.draw_self(&draw, scale, rotation);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}

fn draw_simple_flower(
    draw: &Draw,
    leaf_count: u32,
    scale: f32,
    rotation: f32,
    start: Vec2,
    tip: Vec2,
    middle: f32,
    thickness: f32,
    color: impl IntoLinSrgba<ColorScalar>,
) {
    let color = color.into_lin_srgba();
    let tip_translated = tip - start;
    let m = (tip_translated) * middle;
    let m1 = m + vec2(0.0, thickness * 0.5);
    let m2 = m + vec2(0.0, thickness * -0.5);
    let root = vec2(0.0, 0.0);
    let draw = draw.xy(start);
    for i in 0..leaf_count {
        draw.path()
            .start_shape(root)
            .add_bezier_curve(vec![m1 * scale, tip_translated * scale])
            .add_bezier_curve(vec![root])
            .add_bezier_curve(vec![m2 * scale, tip_translated * scale])
            .add_bezier_curve(vec![root])
            .as_fill()
            .rotate(i.to_f32().unwrap() * TAU / (leaf_count.to_f32().unwrap()) + rotation)
            .color(color)
            .finish();
    }
}

fn draw_duo_colored_flower(
    draw: &Draw,
    leaf_count: u32,
    scale: f32,
    rotation: f32,
    start: Vec2,
    tip: Vec2,
    middle: f32,
    thickness: f32,
    color: &[impl IntoLinSrgba<ColorScalar> + Clone; 2],
) {
    draw_simple_flower(
        draw,
        leaf_count,
        scale,
        rotation,
        start,
        tip,
        middle,
        thickness,
        color[0].clone(),
    );
    draw_simple_flower(
        draw,
        leaf_count,
        scale,
        rotation,
        start,
        tip * 0.85,
        middle,
        thickness * 0.75,
        color[1].clone(),
    );
}

/// Event handler
fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    let t = _app.elapsed_frames();
    match event {
        MouseMoved(pos) => {
            model.mouse_position = Some(pos);
            if model.pressed && model.last_flower_born_time.map_or(true, |b| b + 12 < t) {
                let tip = pos + vec2(10.0 * random_range(3.0, 8.0), 0.0);
                let flower = Flower {
                    start: pos,
                    tip: tip,
                    middle: random_range(0.0, 1.0),
                    thickness: random_range(10.0, 40.0),
                    color: hsla(
                        random_range(0.0, 360.0),
                        random_range(0.4, 0.7),
                        random_range(0.2, 0.5),
                        0.85,
                    ),
                    leaf_count: random_range(3, 10) * 2,
                    born_time: t,
                };
                model.flowers.push(Box::new(flower));
                if random::<bool>() {
                    let circle = Circle {
                        center: pos,
                        radius: pos.distance(tip) * 0.15,
                        color:hsla(
                            random_range(0.0, 360.0),
                            random_range(0.4, 0.7),
                            random_range(0.3, 0.5),
                            0.95,
                        )
                    };
                    model.flowers.push(Box::new(circle));
                }
                model.last_flower_born_time = Some(t);
            }
        }
        MousePressed(_) => {
            model.pressed = true;
        }
        MouseReleased(_) => {
            model.pressed = false;
        }
        _ => (),
    }
}
trait QuadraticBezierCurveDraw<'a> {
    fn quadratic_bezier(self, points: &'a [Vec2; 3]) -> DrawingPath;
}

impl<'a> QuadraticBezierCurveDraw<'a> for Drawing<'a, nannou::draw::primitive::PathStroke> {
    fn quadratic_bezier(self, points: &'a [Vec2; 3]) -> DrawingPath {
        let points = (0..11)
            .map(|t_delta| t_delta.to_f32().unwrap() * 0.1)
            .map(|td| {
                (1.0 - td).pow(2) * points[0]
                    + 2.0 * (1.0 - td) * td * points[1]
                    + td.pow(2) * points[2]
            });
        self.points(points)
    }
}

fn leaf_points(
    start: Vec2,
    tip: Vec2,
    thickness: f32,
    middle: f32,
) -> impl IntoIterator<Item = Vec2> {
    let divisions = 20;
    let delta = 1.0 / divisions as f32;
    let m = (start + tip) * middle;
    let p1 = vec2(m.x + thickness * 0.5, m.y);
    let p2 = vec2(m.x + thickness * -0.5, m.y);
    let points1 = (0..(divisions + 1))
        .map(|t_delta| t_delta.to_f32().unwrap() * delta)
        .map(|td| (1.0 - td).pow(2) * start + 2.0 * (1.0 - td) * td * p1 + td.pow(2) * tip);
    let points2 = (0..(divisions + 1))
        .map(|t_delta| t_delta.to_f32().unwrap() * delta)
        .map(|td| (1.0 - td).pow(2) * tip + 2.0 * (1.0 - td) * td * p2 + td.pow(2) * start);
    points1.chain(points2).collect::<Vec<_>>()
}
