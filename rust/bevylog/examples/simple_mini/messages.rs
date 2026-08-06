use bevy::prelude::*;

// 子弹发射消息：shoot 系统发送，spawn_bullet 系统接收，实现跨系统通信。
// Message 是 Bevy 0.19 引入的轻量通信机制（类似 Event 但更简洁）：
// - 用 add_message::<T>() 注册（见 main.rs）
// - MessageWriter 发送（写端）
// - MessageReader 读取（读端）
#[derive(Message)]
pub struct BulletFired {
    // 发射位置（世界坐标）：取自玩家当前 Transform 的 xy 分量
    pub position: Vec2,
}
