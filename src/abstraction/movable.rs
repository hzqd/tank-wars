use crate::Direction;
use super::blockable::Blockable;

/**移动的能力：*/
pub trait Movable {
    //判断移动体是否和阻塞体发生碰撞：
    fn will_collision(block: impl Blockable) -> Direction;

    //通知碰撞：
    fn notify_collision(direction: Direction, block: impl Blockable);
}