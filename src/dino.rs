use raylib::prelude::*;

pub struct Dino {
    pub position: Vec<i32>,
    pub speed: f32,
    pub image: Texture2D,
}

impl Dino {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(&self.image, self.position[0] as i32, self.position[1] as i32, Color::WHITE);
    }

    pub fn get_rectangle(&self) -> Rectangle {
        Rectangle::new(self.position[0] as f32, self.position[1] as f32, self.image.width as f32, self.image.height as f32)
    }
}