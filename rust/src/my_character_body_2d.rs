use godot::classes::Input;
use godot::global::move_toward;
use godot::prelude::*;
use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
struct MyCharacterBody2D {
    base: Base<CharacterBody2D>
}

impl MyCharacterBody2D {
    const SPEED : f32 = 300.0;
    const JUMP_VELOCITY : f32 = -400.0;    
}

#[godot_api]
impl ICharacterBody2D for MyCharacterBody2D {
    fn init(base: Base<CharacterBody2D>) -> Self {        
        Self { base }
    }
    
    fn physics_process(&mut self, delta: f64) { 
        let mut new_velocity : Vector2 = self.base().get_velocity();
        if !self.base().is_on_floor() {
            new_velocity += self.base().get_gravity() * (delta as f32);
        }
        let input = Input::singleton();
        if input.is_action_just_pressed("ui_accept") || input.is_action_just_pressed("ui_up") {
            new_velocity.y = MyCharacterBody2D::JUMP_VELOCITY;
        }
        let direction = input.get_axis("ui_left", "ui_right");
        if !direction.is_zero_approx() {
            new_velocity.x = direction * MyCharacterBody2D::SPEED;
        }else{
            new_velocity.x = move_toward(self.base().get_velocity().x.into(), 0.0, MyCharacterBody2D::SPEED.into()) as f32;
        }
        self.base_mut().set_velocity(new_velocity);
        self.base_mut().move_and_slide();
     }
    
}