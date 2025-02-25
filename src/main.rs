use raylib::prelude::*;
use dino::Dino;
use mob::Mob;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

mod dino;
mod mob;



fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Raylib Example")
        .build();
    
    rl.set_target_fps(12);

    let mut dino = Dino {
        position: vec![100, 100],
        speed: 10.0,
        image: rl.load_texture(&thread, "assets/dino.png").unwrap(),
    };

    let mob_image = rl.load_texture(&thread, "assets/mob.png").unwrap();

    let mut mob = Mob::init(Vector2::new(500 as f32, 100 as f32), &mob_image);

    let mut mob1 = Mob::init( Vector2::new(564.0, 167.0), &mob_image);

    let mut mob2 = Mob::init( Vector2::new(464.0, 267.0), &mob_image);

    let obstacle = Rectangle::new(400.0, 400.0, 200.0, 200.0);
    

    while !rl.window_should_close() {
        // Event handling
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            dino.position[1] -= dino.speed as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            dino.position[1] += dino.speed as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            dino.position[0] += dino.speed as i32;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            dino.position[0] -= dino.speed as i32;
        }

        // Update
        mob.update();
        mob1.update();
        mob2.update();


        let is_colliding = obstacle.check_collision_recs(&dino.get_rectangle());//CheckCollisionRecs(dino.getRectangle(), obstacle);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Congrats! You have a Dino!", 10, 10, 20, Color::DARKGRAY);
        d.draw_rectangle_lines_ex(obstacle, 3.0, Color::BLACK);
        dino.draw(&mut d);
        mob.draw(&mut d);
        mob1.draw(&mut d);
        mob2.draw(&mut d);
        if is_colliding {
            d.draw_rectangle_lines_ex(dino.get_rectangle(), 3.0, Color::RED);
        }
    }
}
