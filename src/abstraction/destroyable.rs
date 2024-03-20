use crate::abstraction::view_behaviors::ViewBehaviors;

//自我回收/销毁的能力：
pub trait Destroyable {
    fn is_destroyed() -> bool;
    fn show_destroy() -> [impl ViewBehaviors];
}
