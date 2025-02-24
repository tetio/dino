use raylib::prelude::*;

pub struct Mob {
    pub position: Vec<i32>,
    pub speed: f32,
    pub image: Texture2D,
}

impl Mob {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_ex(
            &self.image,
            Vector2::new(self.position[0] as f32, self.position[1] as f32),
            0.0,
            0.25,
            Color::LIGHTCORAL,
        );
    }

    pub fn get_rectangle(&self) -> Rectangle {
        Rectangle::new(
            self.position[0] as f32,
            self.position[1] as f32,
            self.image.width as f32,
            self.image.height as f32,
        )
    }
}
