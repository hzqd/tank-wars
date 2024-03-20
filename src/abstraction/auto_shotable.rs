use super::view_behaviors::ViewBehaviors;

/**自动射击*/
pub trait AutoShotable {
    fn auto_shot() -> impl ViewBehaviors;
}