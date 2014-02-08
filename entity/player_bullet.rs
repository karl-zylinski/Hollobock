use rsfml::system::{Vector2f};
use rsfml::graphics::{RenderWindow, RectangleShape};

use input::Input;
use entity::world::World;
use entity::PlayerBullet;
use entity::{Entity, EntityUpdateResult};
use entity::EntityTrait;

pub struct PlayerBulletStruct {
    position: Vector2f,
    direction: Vector2f,
    velocity: f32
}

impl PlayerBulletStruct {
    pub fn rect(&self) -> RectangleShape {
        let mut rectangle = match RectangleShape::new() {
            Some(rectangle) => rectangle,
            None() => fail!("Error, cannot create rectangle.")
        };

        let size = Vector2f::new(10., 10.);
        let origin = size * 0.5f32;
        
        rectangle.set_size(&size);
        rectangle.set_origin(&origin);
        rectangle.set_position(&self.position);

        return rectangle;
    }
}

impl EntityTrait for PlayerBulletStruct {
    fn update(&self, dt: f32, _world: &World, _input: &Input) -> EntityUpdateResult {
        let new_bullet = PlayerBulletStruct {
            position: self.position + self.direction * self.velocity * dt,
            direction: self.direction,
            velocity: self.velocity
        };

        return EntityUpdateResult { new_entities: ~[PlayerBullet(~new_bullet)] };
    }

    fn draw(&self, window: &mut RenderWindow) {
        window.draw(&self.rect());
    }

    fn position(&self) -> Vector2f
    {
        self.position
    }
    
    fn clone(&self) -> Entity {
        return PlayerBullet(~PlayerBulletStruct {
            position: self.position.clone(),
            direction: self.direction.clone(),
            velocity: self.velocity
        });
    }
}