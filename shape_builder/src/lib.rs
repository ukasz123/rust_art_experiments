use nannou::prelude::ToPrimitive;

pub trait ShapeBuilder<Point> {
    fn add_bezier_curve(self, points: Vec<Point>) -> Self;
}

pub trait ShapePath<'a> {
    fn as_contour(self) -> nannou::draw::Drawing<'a, nannou::draw::primitive::Path>;
    fn as_fill(self) -> nannou::draw::Drawing<'a, nannou::draw::primitive::Path>;
}

pub trait ShapeBuilderFactory<'a, Point, Builder>
where
    Builder: ShapeBuilder<Point>,
{
    fn start_shape(self, start: Point) -> Builder;
}

pub trait GenerateEdge<Point> {
    fn generate_edge(self, start: Point) -> dyn Iterator<Item = Point>;
}

pub struct NannouDrawShapeBuilder<'a, T> {
    draw: nannou::draw::Drawing<'a, T>,
    points: Vec<nannou::glam::Vec2>,
}

impl<'a, T> ShapeBuilder<nannou::glam::Vec2> for NannouDrawShapeBuilder<'a, T> {
    fn add_bezier_curve(self, points: Vec<nannou::glam::Vec2>) -> Self {
        let mut existing_points = self.points;
        // println!("existing_points: {:?}", existing_points);
        let last_point = existing_points.pop().unwrap();

        let mut input = vec![last_point];
        input.extend(points);
        // println!("input: {:?}", input);
        let mut new_points = bezier_curve(input);
        // println!("new_points: {:?}", new_points);
        existing_points.append(&mut new_points);
        return NannouDrawShapeBuilder {
            draw: self.draw,
            points: existing_points,
        };
    }
}

impl<'a> ShapePath<'a> for NannouDrawShapeBuilder<'a, nannou::draw::primitive::PathInit> {
    fn as_contour(self) -> nannou::draw::Drawing<'a, nannou::draw::primitive::Path> {
        let points = self.points;
        self.draw.stroke().points(points)
    }
    fn as_fill(self) -> nannou::draw::Drawing<'a, nannou::draw::primitive::Path> {
        let points = self.points;
        self.draw.fill().points(points)
    }
}

fn bezier_curve<Point>(points: Vec<Point>) -> Vec<Point>
where
    Point: std::ops::Mul<f32, Output = Point> + std::ops::Add<Point, Output = Point> + Copy,
{
    let nn = points.len();
    let n = (nn - 1).to_u128().unwrap();
    let divisions = 10 * n;
    let delta = 1.0 / divisions as f64;
    let points = (0..=divisions)
        .map(|t_delta| t_delta.to_f64().unwrap() * delta)
        .map(|td| {
            let mut result = points[0] * (1.0 - td).powi((nn).to_i32().unwrap()).to_f32().unwrap();
            for i in 1..nn {
                let i_128 = i.to_u128().unwrap();
                let w_i = points[i]
                    * (binomial_coefficient(n, i_128).to_f64().unwrap()
                        * (1.0 - td).powi((nn - i - 1).to_i32().unwrap())
                        * td.powi(i.to_i32().unwrap()))
                    .to_f32()
                    .unwrap();
                result = result + w_i;
            }
            result
        });
    points.collect::<Vec<_>>()
}

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}
fn binomial_coefficient(n: u128, i: u128) -> u128 {
    factorial(n) / factorial(i) / factorial(n - i)
}

#[cfg(test)]
mod test {
    use nannou::lyon::geom::euclid::vec2;

    use crate::bezier_curve;

    #[test]
    fn test_powi() {
        assert_eq!((0.0_f32).powi(0), 1.0)
    }

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(super::binomial_coefficient(5, 0), 1);
        assert_eq!(super::binomial_coefficient(5, 1), 5);
        assert_eq!(super::binomial_coefficient(5, 2), 10);
        assert_eq!(super::binomial_coefficient(5, 3), 10);
        assert_eq!(super::binomial_coefficient(5, 4), 5);
        assert_eq!(super::binomial_coefficient(5, 5), 1);
    }
    #[test]
    fn test_bezier_curve() {
        let points = bezier_curve(vec![vec2::<f32, f32>(0.0, 0.0), vec2(0.0, 10.0)]);
        assert_eq!(
            points,
            vec![
                vec2(0.0, 0.0,),
                vec2(0.0, 2.0,),
                vec2(0.0, 4.0,),
                vec2(0.0, 6.0,),
                vec2(0.0, 8.0,),
                vec2(0.0, 10.0)
            ]
        );
    }
    #[test]
    fn test_bezier_curve1() {
        let points = bezier_curve(vec![vec2::<f32, f32>(0.0, 0.0), vec2(150.0, 100.0)]);
        assert_eq!(
            points,
            vec![
                vec2(0.0, 0.0,),
                vec2(30.0, 20.0,),
                vec2(60.0, 40.0,),
                vec2(90.0, 60.0,),
                vec2(120.0, 80.0,),
                vec2(150.0, 100.0)
            ]
        );
    }

    #[test]
    fn test_bezier_curve_quadratic() {
        let points = bezier_curve(vec![
            vec2::<f32, f32>(0.0, 0.0),
            vec2(50.0, 50.0),
            vec2(100.0, 0.0),
        ]);
        assert_eq!(
            points,
            vec![
                vec2(0.0, 0.0,),
                vec2(20.0, 20.0,),
                vec2(40.0, 40.0,),
                vec2(60.0, 40.0,),
                vec2(80.0, 20.0,),
                vec2(100.0, 0.0)
            ]
        );
    }
}
impl<'a>
    ShapeBuilderFactory<
        'a,
        nannou::glam::Vec2,
        NannouDrawShapeBuilder<'a, nannou::draw::primitive::PathInit>,
    > for nannou::draw::Drawing<'a, nannou::draw::primitive::PathInit>
{
    fn start_shape(
        self,
        start: nannou::glam::Vec2,
    ) -> NannouDrawShapeBuilder<'a, nannou::draw::primitive::PathInit> {
        NannouDrawShapeBuilder {
            draw: self,
            points: vec![start],
        }
    }
}
