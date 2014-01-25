use rsfml::graphics::RenderWindow;
use input::Input;
use world::World;

pub trait Entity {
	fn update(&self, dt: f32, world: &World, input: &Input) -> UpdateResult;
	fn draw(&self, window: &mut RenderWindow);
	fn clone(&self) -> ~Entity;
}

impl Clone for ~Entity {
	fn clone(&self) -> ~Entity {
		return self.clone();
	}
}

pub struct UpdateResult {
	new_entities: ~[~Entity]
	// TODO: Add events here.
}
