/* 显示的视图，定义显示规范 */
pub trait ViewBehaviors {
    /* 定义属性，用实现类实现 */
    fn draw();
    //碰撞检测：
    fn check_collision(x1: u16, x2: u16, y1: u16, y2: u16, w1: u16, w2: u16, h1: u16, h2: u16) -> bool;
}