use raylib::prelude::*;
use rand::prelude::*;

use crate::SCREEN_WIDTH;

pub struct Mob<'a> {
    pub position: Vector2,
    pub speed: f32,
    pub image: &'a Texture2D,
    pub direction: Vector2
}

impl<'a> Mob<'a> {

    pub fn init(position: Vector2, image: &'a Texture2D) -> Mob<'a> {
        let mut rnd = rand::rng();
        Mob {
            position,
            speed: (rnd.next_u32() % 10) as f32,
            image,
            direction: Vector2::new(-1.0, 0.0)
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_ex(
            &self.image,
            Vector2::new(self.position.x, self.position.y),
            0.0,
            1.00,
            Color::LIGHTCORAL,
        );
    }

    pub fn get_rectangle(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.image.width as f32,
            self.image.height as f32,
        )
    }

    pub fn update(&mut self) {
        self.position = Vector2::new(self.position.x + self.direction.x * self.speed, self.position.y + self.direction.y * self.speed) ;
        if self.position.x < 0.0 {
            self.position.x = SCREEN_WIDTH as f32;
        }
        if self.position.x > SCREEN_WIDTH as f32 {
            self.position.x = 0.0;
        }
    }
}
