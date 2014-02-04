use std::f32;

use rsfml::system::Vector2f;
use rsfml::graphics::{RenderWindow, FloatRect};

use input::Input;
use math;
use std::iter::Iterator;
use entity::{Entity, EntityUpdateResult};
use entity::world::World;
use entity::player_bullet::PlayerBullet;
use entity::renderer::Renderer;

pub struct Enemy {
    position: Vector2f,
    rotation: f32,
    renderer: Option<~Renderer:>
}

fn intersecting_with_bullet(enemy: &Enemy, world: &World) -> bool {
    let mut bullets = world.entities.iter().filter(|&e| (*e).is::<PlayerBullet>());

    for bullet in bullets {
        let bullet_entity = match bullet.as_ref::<PlayerBullet>() {
            Some(bullet_entity) => bullet_entity,
            None => fail!("Could not convert to player.")
        };

        let empty_rect = FloatRect::new(0.0,0.0,0.0,0.0);

        if FloatRect::intersects(
            &bullet_entity.rect().get_global_bounds(),
            &enemy.renderer.as_ref().map_or(empty_rect, |r| r.bounds()),
            &empty_rect){
            
            return true;
        }
    }

    return false;
}

impl Enemy {
    pub fn new(position: Vector2f, renderer: Option<~Renderer:>) -> Enemy {
        Enemy {
            position: position,
            rotation: 0.,
            renderer: renderer
        }
    }
}

impl Entity for Enemy {
    fn update(&self, dt: f32, world: &World, _input: &Input) -> EntityUpdateResult {
        let player_entity = match world.entities.iter().find(|&e| e.is_player()) {
            Some(player) => player,
            None => fail!("No player found in world.")
        };

        let direction = math::normalize(player_entity.position() - self.position);
        let new_position = self.position + direction * 100.0f32 * dt;
        let new_rotation = f32::atan2(direction.y, direction.x).to_degrees();

        let new_entities = if intersecting_with_bullet(self, world) {
            ~[]
        } else {
            ~[~Enemy {
                position: new_position,
                rotation: new_rotation,
                renderer: self.renderer.as_ref().map_or(None, |r| r.update(&new_position, new_rotation)),
            } as ~Entity:]
        };

        EntityUpdateResult {
            new_entities: new_entities
        }
    }
    
    fn position(&self) -> Vector2f
    {
        self.position
    }

    fn is_player(&self) -> bool
    {
        false
    }
    
    fn draw(&self, window: &mut RenderWindow) {
        self.renderer.as_ref().map(|r| r.draw(window));
    }

    fn clone(&self) -> ~Entity: {
        ~Enemy {
            position: self.position.clone(),
            rotation: self.rotation,
            renderer: self.renderer.as_ref().map_or(None, |r| r.clone()),
        } as ~Entity:
    }
}