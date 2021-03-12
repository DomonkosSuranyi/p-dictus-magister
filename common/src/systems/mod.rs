pub use entity_delete::EntityDeleteSystemDesc;
pub use health::HealthUpdateSystemDesc;
pub use physics::PhysicsSystem;
pub use lifespan::LifespanSystem;

mod entity_delete;
mod health;
mod physics;
mod lifespan;