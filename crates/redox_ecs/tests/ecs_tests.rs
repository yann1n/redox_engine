#[cfg(test)]
mod ecs_core_tests {
    use redox_ecs::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Position(f32, f32, f32);
    // No manual impl Component needed – blanket implementation covers it

    #[derive(Debug, PartialEq, Clone)]
    struct Health(f32);

    #[derive(Debug, PartialEq, Clone)]
    struct Name(String);

    #[test]
    fn test_entity_allocation() {
        let mut world = World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();
        assert_ne!(e1.id(), e2.id());
        assert_eq!(e1.generation(), 0);
        assert_eq!(e2.generation(), 0);
    }

    #[test]
    fn test_add_component_and_query() {
        let mut world = World::new();
        let entity = world.spawn();
        world.add_component(entity, Position(1.0, 2.0, 3.0));
        world.add_component(entity, Health(100.0));

        let query = Query::<Position>::new();
        let positions: Vec<&Position> = query.iter(&world).collect();
        assert_eq!(positions.len(), 1);
        assert_eq!(*positions[0], Position(1.0, 2.0, 3.0));

        let health_query = Query::<Health>::new();
        let healths: Vec<&Health> = health_query.iter(&world).collect();
        assert_eq!(healths.len(), 1);
        assert_eq!(*healths[0], Health(100.0));
    }

    #[test]
    fn test_remove_component() {
        let mut world = World::new();
        let entity = world.spawn();
        world.add_component(entity, Position(1.0, 2.0, 3.0));
        world.add_component(entity, Health(100.0));

        // Remove Health
        let removed = world.remove_component::<Health>(entity);
        assert!(removed);

        let health_query = Query::<Health>::new();
        let healths: Vec<&Health> = health_query.iter(&world).collect();
        assert_eq!(healths.len(), 0);

        let pos_query = Query::<Position>::new();
        let positions: Vec<&Position> = pos_query.iter(&world).collect();
        assert_eq!(positions.len(), 1);
    }

    #[test]
    fn test_despawn() {
        let mut world = World::new();
        let entity = world.spawn();
        world.add_component(entity, Position(1.0, 2.0, 3.0));

        world.despawn(entity);

        let pos_query = Query::<Position>::new();
        let positions: Vec<&Position> = pos_query.iter(&world).collect();
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_hierarchy() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();

        world.add_component(parent, Name("Parent".to_string()));
        world.add_component(child, Name("Child".to_string()));

        world.add_component(child, Parent(parent));
        // Normally we'd also add a Children component to parent, but that's application logic.

        // Just test that components are present
        let query = Query::<Parent>::new();
        let parents: Vec<&Parent> = query.iter(&world).collect();
        assert_eq!(parents.len(), 1);
        assert_eq!(parents[0].0, parent);
    }

    #[test]
    fn test_events() {
        struct TestEvent(i32);
        let mut events = Events::<TestEvent>::new();
        events.send(TestEvent(42));
        events.update(); // swap buffers

        let mut reader = EventReader::new(&events);
        let collected: Vec<i32> = reader.iter().map(|e| e.0).collect();
        assert_eq!(collected, vec![42]);
    }
}