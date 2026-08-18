use bevy::prelude::*;

// 玩家标记组件：给玩家实体打标签，用于 Query 过滤（With<Player>）。
// 没有字段，仅用作标记（和 Sun、Planet 等标记组件同理）。
#[derive(Component)]
pub struct Player;

// 敌人标记组件：被子弹击中后销毁并加分（见 check_collision 系统）。
#[derive(Component)]
pub struct Enemy;

// 子弹组件：存储子弹的运动参数。
#[derive(Component)]
pub struct Bullet {
    // 子弹速度（像素/秒）：move_bullet 系统用 speed × dt 更新子弹位置
    pub speed: f32,
}
