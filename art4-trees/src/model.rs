use std::fmt::Debug;

use nannou::{
    prelude::{vec2, Hsl, Point2, ToPrimitive, Vec2Rotate, PI},
    rand::{self, random, random_range, seq::SliceRandom},
    Draw,
};

type SunDirectionFn = Box<dyn Fn(&Point2) -> Point2>;

pub struct Model {
    pub height: f32,
    pub width: f32,
    pub mouse_position: Option<Point2>,
    pub delay: u128,
    // other params
    pub tree: Tree,
    pub sun_direction: SunDirectionFn,
}

#[derive(Debug)]
pub struct Tree {
    pub trunk: Branch,
}

impl Tree {
    pub fn new(base: Point2, color: Hsl) -> Tree {
        Tree {
            trunk: Branch::new(base, vec2(0.0, 1.0), color),
        }
    }
}

#[derive(Debug)]
pub struct Branch {
    pub base: Point2,
    pub tip: Point2,
    pub growth_direction: Point2,
    pub children: Vec<Branch>,
    pub color: Hsl,
    resources: f32,
    stopped: bool,
}
const NEW_BRANCH_RESOURCES_LEVEL: f32 = 10.0;
const GROWTH_MIN_RESOURCES_LEVEL: f32 = 5.0;
const BRANCH_COST_MIN: f32 = 6.0;
const BRANCH_COST_MAX: f32 = 10.0;

impl Branch {
    pub fn new(base: Point2, direction: Point2, color: Hsl) -> Branch {
        Branch {
            base: base,
            tip: base,
            growth_direction: direction.normalize(),
            children: vec![],
            resources: 0.0,
            stopped: false,
            color: color,
        }
    }

    pub(crate) fn advance(&mut self, additional_resources: f32, sun_direction_fn: &SunDirectionFn) {
        if additional_resources == 0.0 {
            return;
        }
        let accumulation_multiplier = if self.stopped { 0.05 } else { 1.0 };
        let accumulate = random_range(
            0.0,
            accumulation_multiplier * additional_resources
                / (self.children.len() + 3).to_f32().unwrap(),
        );

        let mut remaining = additional_resources - accumulate;

        for c in self.children.iter_mut().rev() {
            let chunk = random_range(0.0, remaining);
            c.advance(chunk, sun_direction_fn);
            remaining = remaining - chunk;
        }
        self.resources += accumulate + remaining;

        if self.resources > NEW_BRANCH_RESOURCES_LEVEL {
            if random_range(0.0, 1.0) < 0.25 {
                self.resources = self.resources - random_range(BRANCH_COST_MIN, BRANCH_COST_MAX);
                let new_branch_direction = self
                    .growth_direction
                    .rotate(PI * if random::<bool>() { 0.5 } else { -0.5 });
                let sun_direction = (sun_direction_fn(&self.tip) - self.tip).normalize();
                let combined_direction = (2.25 * new_branch_direction + sun_direction)
                    .normalize()
                    .rotate(PI * random_range(-0.05, 0.05));
                let new_branch = Branch::new(self.tip, combined_direction, child_color(self.color));
                self.children.push(new_branch);
            }
        }
        if !self.stopped && self.resources > GROWTH_MIN_RESOURCES_LEVEL {
            if random_range(0.0, 1.0) < 0.15 {
                let sun_direction = (sun_direction_fn(&self.tip) - self.tip).normalize();
                if sun_direction != self.growth_direction && random_range(0.0, 1.0) < 0.25 {
                    self.stopped = true;
                    let growth = self.resources;
                    self.resources = 0.0;
                    let new_direction = self.growth_direction + sun_direction;
                    let mut continuation = Branch::new(self.tip, new_direction, self.color);
                    continuation.advance(growth, sun_direction_fn);
                    self.children.push(continuation);
                } else {
                    let growth = random_range(1.0, 0.5 * GROWTH_MIN_RESOURCES_LEVEL);
                    self.resources = self.resources - growth;
                    self.tip = self.tip + growth * 4.0 * self.growth_direction;
                }
            }
        }
    }

    pub(crate) fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.base)
            .end(self.tip)
            .color(self.color)
            .weight(2.0);
        for b in &self.children {
            b.draw(draw);
        }
    }
}

fn child_color(color: Hsl) -> Hsl {
    let hue = color.hue;
    let hue_rad = hue.to_radians();
    let hue_rad = hue_rad + PI * random_range(-0.15, 0.15);
    let hue = hue_rad.to_degrees();
    Hsl::new(hue, color.saturation, color.lightness)
}
