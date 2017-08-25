use bit_vec::BitVec;

/// Set of logical key-presses.
#[repr(usize)]
pub enum Key {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    RollLeft,
    RollRight,
}

pub struct PressedKeys {
    storage: BitVec<u32>,
}

impl PressedKeys {
    pub fn new() -> PressedKeys {
        PressedKeys { storage: BitVec::from_elem(1024, false) }
    }

    pub fn test(&self, key: Key) -> bool {
        self.storage.get(key as usize).unwrap_or(false)
    }

    pub fn set(&mut self, key: Key, state: bool) {
        self.storage.set(key as usize, state)
    }
}
