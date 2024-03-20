use inherit::Inherit;
use crate::Direction;

#[derive(Debug, Default)]
pub struct Common {
    //宽高：
    pub width: u8,
    pub height: u8,
}

#[derive(Inherit, Debug, Default)]
pub struct ActObj {
    //继承：
    pub _c: Common,
    //位置：
    pub x: u16,
    pub y: u16,
}

#[derive(Inherit, Debug, Default)]
pub struct MovObj {
    //继承：
    pub _a: ActObj,
    //方向：
    pub direction: Direction,
    //速度：
    pub speed: u8,
}

#[derive(Inherit, Debug, Default)]
pub struct InteractObj {
    //继承：
    pub _m: MovObj,
    //血量：
    pub hp: u8,
    //攻：
    pub attack_power: u8,
    //防：
    pub defense_power: u8,
}
