use super::{sufferable::Sufferable, attackable::Attackable, blockable::Blockable, destroyable::Destroyable, movable::Movable};

pub trait Tank: Movable + Destroyable + Blockable + Attackable + Sufferable {}