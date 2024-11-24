use std::collections::HashMap;

#[derive(Debug)]
pub enum Entity {
    File(String, usize),
    Dir(String, HashMap<String, Entity>),
}

impl Entity {
    pub fn size(&self) -> usize {
        match &self {
            Self::File(_, size) => *size,
            Self::Dir(_, entities) => entities.values().map(|e| e.size()).sum::<usize>(),
        }
    }

    pub fn new_dir(name: String) -> Entity {
        Entity::Dir(name, HashMap::new())
    }

    pub fn new_file(name: String, size: usize) -> Entity {
        Entity::File(name, size)
    }

    pub fn add_entity(&mut self, name: String, entity: Entity) {
        match self {
            Entity::Dir(_, children) => {
                children.insert(name, entity);
            }
            _ => unreachable!("Can't add file to a file"),
        }
    }

    pub fn get_dir(&mut self, name: &String) -> Option<&mut Entity> {
        match self {
            Entity::Dir(_, children) => {
                children.get_mut(name)
            }
            _ => unreachable!("Files doesn't have dirs inside of them"),
        }
    }
}
