use godot::classes::{ISprite2D, Input, Sprite2D};
use godot::prelude::*;

enum PlayerDirectionHorizontal {
    Left,
    Right,
}

enum PlayerDirectionVertical {
    Up,
    Down,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        let input = Input::singleton();

        let mut horizontal_direction = PlayerDirectionHorizontal::Right;
        let mut vertical_direction = PlayerDirectionVertical::Down;

        let rotation = self.base_mut().get_rotation();

        if input.is_action_pressed("ui_left") {
            horizontal_direction = PlayerDirectionHorizontal::Left;
        } else if input.is_action_pressed("ui_right") {
            horizontal_direction = PlayerDirectionHorizontal::Right;
        }

        if input.is_action_pressed("ui_up") {
            vertical_direction = PlayerDirectionVertical::Up;
        } else if input.is_action_pressed("ui_down") {
            vertical_direction = PlayerDirectionVertical::Down;
        }

        let mut velocity = Vector2::UP.rotated({
            match vertical_direction {
                PlayerDirectionVertical::Up => rotation,
                PlayerDirectionVertical::Down => rotation + std::f32::consts::PI,
            }
        }) * self.speed as f32;
        
        match horizontal_direction {
            PlayerDirectionHorizontal::Left => velocity.x *= -1.0,
            PlayerDirectionHorizontal::Right => velocity.x *= 1.0,
        }
        match vertical_direction {
            PlayerDirectionVertical::Up => velocity.y *= -1.0,
            PlayerDirectionVertical::Down => velocity.y *= 1.0,
        }

        self.base_mut().translate(velocity * delta as f32);

        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
}
