use super::{attackable::Attackable, view_behaviors::ViewBehaviors};

/**受攻的能力：*/
pub trait Sufferable {
    fn notify_suffer(attackable: impl Attackable) -> [impl ViewBehaviors];
}