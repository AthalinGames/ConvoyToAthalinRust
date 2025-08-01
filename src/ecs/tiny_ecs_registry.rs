use std::collections::HashMap;
use std::path::Component;
use std::sync::atomic::{AtomicU32, Ordering};
use anyhow::{anyhow, Result};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Entity {
    id: u32
}

impl Entity {
    fn new() -> Entity {
        static COUNTER: AtomicU32 = AtomicU32::new(0);
        Entity{
            id: COUNTER.fetch_add(1, Ordering::Relaxed)
        }
    }
}

pub(crate) trait ContainerInterface {
    fn clear(&mut self);
    fn size(&self) -> usize;
    fn remove(&mut self, e: &Entity, keep_order: bool);
    fn pop_back(&mut self);
    fn has(&self, e: &Entity) -> bool;
}

pub(crate) struct ComponentContainer<Component> {
    map_entity_component_id: HashMap<u32, usize>,
    registered: bool,
    components: Vec<Component>,
    entities: Vec<Entity>
}

impl ContainerInterface for ComponentContainer<Component<'_>> {
    fn clear(&mut self) {
        self.map_entity_component_id.clear();
        self.components.clear();
        self.entities.clear();
    }

    fn size(&self) -> usize {
        self.components.len()
    }

    fn remove(&mut self, e: &Entity, keep_order: bool) {
        if !self.has(e) {
            return;
        }
        let component_id = self.map_entity_component_id[&e.id];
        if keep_order {
            self.components.remove(component_id);
            self.entities.remove(component_id);
        } else {
           self.components.swap_remove(component_id);
            self.entities.swap_remove(component_id);
        }
        self.map_entity_component_id.remove(&e.id);
    }

    fn pop_back(&mut self) {
        if let Some(e) = self.entities.last() {
            if self.map_entity_component_id.contains_key(&e.id) {
                self.map_entity_component_id.remove(&e.id);
            }
            self.components.pop();
            self.entities.pop();
        }
    }

    fn has(&self, e: &Entity) -> bool {
        self.map_entity_component_id.contains_key(&e.id)
    }
}

impl ComponentContainer<Component<'_>> {
    fn insert(&mut self, e: &Entity, c: Component, check_for_duplicates: bool) -> Result<&mut Component<'_>> {
        if check_for_duplicates && self.has(e) {
            return Err(anyhow!("Insertion Error"))
        }
        self.map_entity_component_id.insert(e.id, self.components.len());
        self.components.push(c);
        self.entities.push(e.clone());
        Ok(self.components.last_mut().unwrap())
    }
}