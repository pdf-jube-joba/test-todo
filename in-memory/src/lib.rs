use std::collections::HashMap;

use defs::Id;

struct Memory {
    new_key: defs::Id,
    map: HashMap<defs::Id, defs::Todo>,
}

impl defs::Repository for Memory {
    fn new() -> Self
    where
        Self: Sized,
    {
        Memory {
            new_key: Id(0),
            map: HashMap::new(),
        }
    }
    fn add(&mut self, todo: defs::Todo) -> defs::Id {
        let key = self.new_key.clone();
        self.map.insert(key.clone(), todo);
        key
    }
    fn check(&mut self, id: Id) -> Option<()> {
        let v = self.map.get_mut(&id)?;
        v.0 = !v.0; 
        Some(())
    }
    fn list(&self) -> Vec<Id> {
        self.map.keys().cloned().collect()
    }
}
