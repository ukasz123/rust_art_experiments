use std::fmt::Debug;

use nannou::{
    color::Alpha,
    lyon::lyon_tessellation::StrokeOptions,
    noise::{self, NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Point {
    coords: Point2,
    coords_prev: Point2,
    color: Hsl,
    direction: Point2,
    speed: f32,
}

impl Point {
    fn new(starting_coords: Point2, color: Hsl, direction: Point2, speed: Option<f32>) -> Point {
        Point {
            coords: starting_coords,
            coords_prev: starting_coords,
            color,
            direction: direction.normalize(),
            speed: speed.unwrap_or(1.0),
        }
    }
}

struct Model {
    points: Vec<Point>,
    h: f32,
    w: f32,
    noise: Perlin,
    mouse_position: Option<Point2>,
}

impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("points", &self.points)
            .field("h", &self.h)
            .field("w", &self.w)
            .finish()
    }
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let noise = noise::Perlin::new();
    Model {
        points: vec![],
        h: window.h(),
        w: window.w(),
        noise,
        mouse_position: None,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() == 1 {
        frame.clear(BLACK);
    }

    let draw = app.draw();
    draw.rect().hsla(1.0, 1.0, 0.0, 0.01).w_h(model.w, model.h);
    for p in &model.points {
        draw.line()
            .start(p.coords)
            .end(p.coords_prev)
            .caps_round()
            .stroke_weight(3.0)
            .join_round()
            .color(p.color);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for p in model.points.iter_mut() {
        p.coords_prev = p.coords;
        p.coords = p.coords + p.direction * p.speed;
        if (random_range(0.0, 1.0)) < 0.1 {
            let v = NoiseFn::get(&model.noise, [p.coords.x.into(), p.coords.y.into()]);
            let v = map_range(v, -1.0, 1.0, -0.15, 0.15) * TAU_F64;
            let new_direction = p.direction.rotate(v.to_f32().unwrap());
            p.direction = new_direction;
        }
        if random_range(0.0, 1.0) < 0.01 {
            p.direction
                .rotate(PI * if random::<bool>() { 0.5 } else { -0.5 });
        }

        p.direction = match (p.coords.x, p.coords.y) {
            (x, _) if x <= -0.5 * model.w || x >= 0.5 * model.w => {
                Point2::new(-p.direction.x, p.direction.y)
            }
            (_, y) if y <= -0.5 * model.h || y >= 0.5 * model.h => {
                Point2::new(p.direction.x, -p.direction.y)
            }
            _ => p.direction,
        }
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            MouseMoved(pos) => model.mouse_position = Some(pos),
            MousePressed(_) => {
                if let Some(pos) = model.mouse_position {
                    let rays = random_range(3, 10);
                    for _ in 0..rays {
                        let color = hsl(
                            random_range(0.0, 0.5) * 2.0,
                            random_range(0.7, 1.0),
                            random_range(0.5, 0.8),
                        );
                        let v = NoiseFn::get(
                            &model.noise,
                            [(0.01 * pos.x).into(), (0.01 * pos.y).into()],
                        );

                        let direction = vec2(0.0, 1.0)
                            .rotate(v.to_f32().unwrap() * TAU * random_range(0.6, 1.4));
                        model.points.push(Point::new(
                            pos,
                            color,
                            direction,
                            Some(random_range(1.0, 4.0)),
                        ));
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
}
