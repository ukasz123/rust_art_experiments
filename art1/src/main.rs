use nannou::{color::rgb::Rgba, geom::point, noise::{NoiseFn, utils::NoiseMap}, prelude::*, ease::map};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[derive(Debug)]
struct Point {
    coords: Point2,
    color: Hsl,
}

#[derive(Debug)]
struct Model {
    points: Vec<Point>,
}

fn model(app: &App) -> Model {
    let window = app.window_rect();
    let spacing: f32 = 30.0;
    let h = window.h();
    let w = window.w();
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
                random_range(0.6, 1.0),
            );
            points.push(Point { coords: p, color })
        }
    }
    let model = Model { points };

    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // let noise = nannou::noise::Perlin::new();
    // let noise = nannou::noise::Fbm::new();
    let noise = nannou::noise::RidgedMulti::new();
    let m = 0.0001;
    for ele in model.points.iter_mut() {
        let v = NoiseFn::get(&noise, [(m * ele.coords.y).into(), (m * ele.coords.x).into()]);
        let noise_value_map = deg_to_rad(map_range(v, -1.0, 1.0, -360.0, 360.0));
        ele.coords += pt2(noise_value_map.cos(), noise_value_map.sin());
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() == 0 {
        frame.clear(BLACK);
    }
    let draw = app.draw();

    for p in &model.points {
        draw.ellipse().color(p.color).xy(p.coords).w_h(2.0, 2.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap()
}
