use godot::engine::{AnimatedSprite2D, RigidBody2D, RigidBody2DVirtual};
use godot::prelude::*;
use rand::seq::SliceRandom;

#[derive(GodotClass)]
#[class(rustbase=Mob, init)]
pub struct Mob2 {
    pub min_speed: real,
    pub max_speed: real,

    #[base]
    base: Base<Mob>,
}

#[godot_api]
impl Mob2 {
    #[func]
    fn do_thing_idk(&mut self) {
        godot_print!("hello world!");
        self.base.bind_mut().amongus();
        godot_print!("Bye!");
        self.base.add_child(RigidBody2D::new_alloc().upcast());
        godot_print!("M!");
    }
}

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    pub min_speed: real,
    pub max_speed: real,

    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.base.queue_free();
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base.queue_free();
    }

    #[func]
    fn amongus(&mut self) {
        godot_print!("Hi I'm amongus");
    }
}

#[godot_api]
impl RigidBody2DVirtual for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Mob {
            min_speed: 150.0,
            max_speed: 250.0,
            base,
        }
    }

    fn ready(&mut self) {
        let mut sprite = self
            .base
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.play();
        let anim_names = sprite.get_sprite_frames().unwrap().get_animation_names();

        // TODO use pick_random() once implemented
        let anim_names = anim_names.to_vec();
        let mut rng = rand::thread_rng();
        let animation_name = anim_names.choose(&mut rng).unwrap();

        sprite.set_animation(animation_name.into());
    }
}
