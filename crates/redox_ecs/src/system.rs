use crate::world::World;

/// Trait implemented by any logic that operates on the ECS `World`.
pub trait System {
    /// Runs the system logic.
    fn run(&mut self, world: &mut World);
}

/// A collection of systems to be executed in order.
pub struct SystemStage {
    systems: Vec<Box<dyn System>>,
}

impl SystemStage {
    /// Creates a new empty stage.
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the stage.
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    /// Runs all systems in order.
    pub fn run(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}