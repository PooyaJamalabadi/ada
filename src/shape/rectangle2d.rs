use super::draw_line2d;
use crate::canvas::Canvas;
use crate::shape::Shape;
use crate::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rectangle2D {
    x: i32,
    y: i32,
    height: i32,
    width: i32,
}

impl Rectangle2D {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rectangle2D {
            x,
            y,
            width,
            height,
        }
    }
}

impl Shape for Rectangle2D {
    fn draw(&self, canvas: &mut Canvas, color: &Color) {
        draw_rect2d(self.x, self.y, self.width, self.height, canvas, color);
    }

    fn draw_filled(&self, canvas: &mut Canvas, color: &Color) {
        draw_rect2d_filled(self.x, self.y, self.width, self.height, canvas, color);
    }
}

pub fn draw_rect2d(x: i32, y: i32, width: i32, height: i32, canvas: &mut Canvas, color: &Color) {
    draw_line2d(x, y, x + width, y, canvas, color);
    draw_line2d(x, y, x, y + height, canvas, color);
    draw_line2d(x + width, y, x + width, y + height, canvas, color);
    draw_line2d(x, y + height, x + width, y + height, canvas, color);
}

pub fn draw_rect2d_filled(x: i32, y: i32, width: i32, height: i32, canvas: &mut Canvas, color: &Color) {
    for i in x..(x+width) {
        for j in y..(y+height) {
            canvas.draw_point(i, j, color);
        }
    }
}