use super::sufferable::Sufferable;

/**攻击的能力：*/
pub trait Attackable {
    fn is_collision(sufferable: impl Sufferable) -> bool;
    fn notify_attack(sufferable: impl Sufferable);
}