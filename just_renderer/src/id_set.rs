use std::{
    collections::{
        hash_map::{Values, ValuesMut},
        HashMap,
    },
    hash::Hash,
    ops::AddAssign,
};

// Just normal HashMap with
pub struct IdSet<Key, V> {
    entries: HashMap<Key, V>,
    last_id: Key,
}

impl<Key, V> IdSet<Key, V>
where
    Key: Eq + Hash + Default + AddAssign<u32> + Copy,
{
    pub fn empty() -> Self {
        Self {
            entries: HashMap::new(),
            last_id: Default::default(),
        }
    }
    pub fn insert(&mut self, value: V) -> Key {
        self.last_id += 1;
        self.entries.insert(self.last_id, value);

        return self.last_id;
    }

    pub fn get_mut(&mut self, key: &Key) -> Option<&mut V> {
        self.entries.get_mut(key)
    }

    pub fn remove(&mut self, key: &Key) -> Option<V> {
        self.entries.remove(key)
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, Key, V> {
        self.entries.values_mut()
    }

    pub fn values(&self) -> Values<'_, Key, V> {
        self.entries.values()
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
