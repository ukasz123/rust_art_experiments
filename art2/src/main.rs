use nannou::{
    color::{rgb::Rgba, Alpha},
    ease::{bounce, map},
    geom::point,
    noise::{utils::NoiseMap, NoiseFn},
    prelude::*,
};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Point {
    coords: [Point2; 3],
    color: Hsl,
}

impl Point {
    fn new(starting_coords: Point2, color: Hsl) -> Point {
        let coords = [starting_coords, starting_coords, starting_coords];
        Point { coords, color }
    }
}

#[derive(Debug)]
struct Model {
    points: Vec<Point>,
    h: f32,
    w: f32,
    mouse_pos: Option<Point2>,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let spacing: f32 = 15.0;
    let h = window.h() - 20.0;
    let w = window.w() - 20.0;
    let vertical_steps = (h / spacing).floor().to_usize().unwrap();
    let horizontal_steps = (w / spacing).floor().to_usize().unwrap();
    let mut points = vec![];
    for i in 0..vertical_steps {
        for j in 0..horizontal_steps {
            let p = Point2::new(
                (j as f32) * spacing + random_range::<f32>(-3.0, 3.0) - w * 0.5 + 4.0,
                (i as f32) * spacing + random_range::<f32>(-3.0, 3.0) - h * 0.5 - 4.0,
            );
            let color = hsl(
                random_range(0.0, 1.0),
                random_range(0.3, 1.0),
                random_range(0.3, 1.0),
            );
            points.push(Point::new(p, color))
        }
    }
    let model = Model {
        points,
        h,
        w,
        mouse_pos: Option::None,
    };
    model
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();

    for p in &model.points {
        for (index, coords) in p.coords.iter().enumerate() {
            draw.ellipse().xy(*coords).radius(2.0).color(Alpha {
                color: p.color,
                alpha: ((3 - index as u32) as f32) * 0.33,
            });
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = nannou::noise::Perlin::new();
    // let noise = nannou::noise::Fbm::new();
    // let noise = nannou::noise::RidgedMulti::new();
    let m = 0.05;
    let mouse_m = 0.01;
    let mouse_pt = match model.mouse_pos {
        Some(pos) => pos,
        None => pt2(0.0, 0.0),
    };
    model.points.par_iter_mut().for_each(|ele| {
        let dist = ele.coords[0].distance(mouse_pt);
        let dir = (mouse_pt - ele.coords[0]).normalize_or_zero();
        let c = dir * dist * mouse_m;
        let v = NoiseFn::get(
            &noise,
            [
                (m * ele.coords[0].y + c.x).into(),
                (m * ele.coords[0].x + c.y).into(),
            ],
        );
        let noise_value_map = deg_to_rad(map_range(v, -1.0, 1.0, -360.0, 360.0));
        ele.coords[0] = ele.coords[0] + pt2(noise_value_map.cos(), noise_value_map.sin()) * 2.0;
        ele.coords[1] = ele.coords[1] + 0.5 * (ele.coords[0] - ele.coords[1]);
        ele.coords[2] = ele.coords[2] + 0.5 * (ele.coords[1] - ele.coords[2]);
        let correction = pt2(
            correct_to_range(ele.coords[0].x, -0.5 * model.w, 0.5 * model.w),
            correct_to_range(ele.coords[0].y, -0.5 * model.h, 0.5 * model.h),
        );
        ele.coords[0] = ele.coords[0] + correction;
        ele.coords[1] = ele.coords[1] + correction;
        ele.coords[2] = ele.coords[2] + correction;
    });
}

fn correct_to_range(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        let n = max + (min - v);
        return n - v;
    }
    while v > max {
        let n = min + (v - max);
        return n - v;
    }
    0.0
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            MouseMoved(pos) => {
                model.mouse_pos = Some(pos);
            }
            _ => (),
        },
        _ => (),
    }
}
