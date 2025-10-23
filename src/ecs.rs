//! Simple Entity Component System (ECS)
//!
//! Provides a basic ECS architecture for organizing game objects.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Unique identifier for entities
pub type EntityId = u64;

/// Trait that all components must implement
pub trait Component: Any + 'static {
    /// Returns a unique identifier for this component type
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

// Implement Component for common types
impl Component for crate::math::Transform {}
impl Component for crate::math::Transform2D {}

/// An entity in the game world
#[derive(Debug, Clone)]
pub struct Entity {
    id: EntityId,
    name: String,
    active: bool,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    /// Create a new entity
    pub fn new(id: EntityId, name: String) -> Self {
        Self {
            id,
            name,
            active: true,
            components: HashMap::new(),
        }
    }

    /// Get the entity ID
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// Get the entity name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if the entity is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set entity active state
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Add a component to this entity
    pub fn add_component<T: Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }

    /// Get a reference to a component
    pub fn get_component<T: Component>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)?.downcast_ref::<T>()
    }

    /// Get a mutable reference to a component
    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components.get_mut(&type_id)?.downcast_mut::<T>()
    }

    /// Check if entity has a specific component
    pub fn has_component<T: Component>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains_key(&type_id)
    }

    /// Remove a component from this entity
    pub fn remove_component<T: Component>(&mut self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.remove(&type_id).is_some()
    }
}

/// A scene manages a collection of entities
pub struct Scene {
    entities: HashMap<EntityId, Entity>,
    next_entity_id: EntityId,
    name: String,
}

impl Scene {
    /// Create a new scene
    pub fn new(name: String) -> Self {
        log::info!("Created scene: {}", name);
        Self {
            entities: HashMap::new(),
            next_entity_id: 0,
            name,
        }
    }

    /// Get scene name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Create a new entity in this scene
    pub fn create_entity(&mut self, name: String) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;

        let entity = Entity::new(id, name);
        self.entities.insert(id, entity);

        log::debug!("Created entity with ID: {}", id);
        id
    }

    /// Get a reference to an entity
    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    /// Get a mutable reference to an entity
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    /// Remove an entity from the scene
    pub fn remove_entity(&mut self, id: EntityId) -> bool {
        self.entities.remove(&id).is_some()
    }

    /// Get all entities
    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    /// Get all entities (mutable)
    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    /// Get all active entities
    pub fn active_entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values().filter(|e| e.is_active())
    }

    /// Get all active entities (mutable)
    pub fn active_entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut().filter(|e| e.is_active())
    }

    /// Find entities with a specific component
    pub fn find_entities_with<T: Component>(&self) -> Vec<EntityId> {
        self.entities
            .values()
            .filter(|e| e.has_component::<T>())
            .map(|e| e.id())
            .collect()
    }

    /// Get count of entities
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Clear all entities from the scene
    pub fn clear(&mut self) {
        self.entities.clear();
        self.next_entity_id = 0;
        log::info!("Cleared scene: {}", self.name);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new("Default Scene".to_string())
    }
}

/// Helper macro to add multiple components at once
#[macro_export]
macro_rules! add_components {
    ($entity:expr, $($component:expr),* $(,)?) => {
        $(
            $entity.add_component($component);
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestComponent {
        value: i32,
    }
    impl Component for TestComponent {}

    #[test]
    fn test_entity_components() {
        let mut entity = Entity::new(0, "Test".to_string());
        entity.add_component(TestComponent { value: 42 });

        assert!(entity.has_component::<TestComponent>());
        assert_eq!(entity.get_component::<TestComponent>().unwrap().value, 42);
    }

    #[test]
    fn test_scene() {
        let mut scene = Scene::new("Test Scene".to_string());
        let id = scene.create_entity("TestEntity".to_string());

        assert_eq!(scene.entity_count(), 1);
        assert!(scene.get_entity(id).is_some());
    }
}
