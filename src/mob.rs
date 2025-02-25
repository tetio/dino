use raylib::prelude::*;

pub struct Mob {
    pub position: Vector2,
    pub speed: f32,
    pub image: Texture2D,
}

impl Mob {
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
        self.position = Vector2::new(self.position.x * self.speed, self.position.y * self.speed) ;
    }
}
