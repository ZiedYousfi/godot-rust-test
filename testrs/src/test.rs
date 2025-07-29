use godot::prelude::*;
use godot::classes::{ISprite2D, Sprite2D};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Test {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Test {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Test class initialized");
        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }
    fn physics_process(&mut self, delta: f64) {
        let _ = delta;
        let _ = self.speed * delta + self.angular_speed * delta;
    }
}
