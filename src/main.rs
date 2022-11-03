use macroquad::{prelude::*, rand::srand};
use ::rand::random;

#[macroquad::main("BasicShapes")]
async fn main() {
    srand(1);
    let mut points: Vec<Point> = Vec::new();
    for y in 0..screen_height() as u32 {
        for x in 0..screen_width() as u32{
            if random::<u32>() % 10000 == 0 {
                points.push(Point::new(x as f32,y as f32))
            }
        }
    }
    loop {
        for point in &mut points{
            point.update_pos();
        }
        clear_background(BLACK);
        for p1 in 0..points.len(){
            for p2 in p1..points.len(){
                let tmp = points[p1].distance(points[p2]);
                let t2 = 1.0 - tmp/200.0;
                if t2 >= 0.0 {
                    let c = Color::new(1.0, 1.0, 1.0, t2);
                    draw_line(points[p1].get_x() as f32, points[p1].get_y() as f32, points[p2].get_x() as f32, points[p2].get_y() as f32, 1f32, c)
                }
            }
        }

        next_frame().await
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
    of_x: f32,
    of_y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        return Point { x, y, of_x:random::<f32>() * 2.0 - 1.0, of_y:random::<f32>() * 2.0 - 1.0 };
    }

    pub fn get_x(&self) -> f32 {
        return self.x;
    }

    pub fn get_y(&self) -> f32 {
        return self.y;
    }

    // pub fn update_pos(&mut self) {
    //     self.x += self.of_x;
    //     if self.x > screen_width() {self.x = 0.0}
    //     if self.x < 0.0 {self.x = screen_width()}
    //     self.y += self.of_y;
    //     if self.y > screen_height() {self.y = 0.0}
    //     if self.y < 0.0 {self.y = screen_height()}
    // }

    pub fn update_pos(&mut self) {
        self.x += self.of_x;
        if self.x > screen_width() || self.x < 0.0 {self.of_x = -self.of_x}
        self.y += self.of_y;
        if self.y > screen_height() || self.y < 0.0 {self.of_y = -self.of_y}
    }

    pub fn distance(&self, p: Point) -> f32 {
        return f32::sqrt(
            f32::powf(p.x as f32 - self.x as f32, 2.0)
                + f32::powf(p.y as f32 - self.y as f32, 2.0),
        );
    }
}