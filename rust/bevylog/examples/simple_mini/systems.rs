use bevy::prelude::*;

use crate::components::{Bullet, Enemy, Player};
use crate::messages::BulletFired;
use crate::resources::Score;

// 场景初始化：生成相机、玩家、敌人。
pub fn setup(mut commands: Commands) {
    // 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);

    // 玩家：蓝色方块（50×50），起始位置屏幕下方
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, -300.0, 0.0),
        Player,
    ));

    // 敌人：红色方块（40×40），生成多个用于测试碰撞和计分。
    // 前两个敌人间距仅 50 像素（< 2×碰撞阈值 60），用来验证 break 修复：
    //   子弹从两者中间穿过时，没有 break 会重复加分（+200），有 break 只加一次（+100）。
    // 显式标注 f32：浮点字面量默认是 f64，而数组需要统一类型
    let enemy_positions: [(f32, f32); 5] = [
        (100.0, 200.0),  // 敌人 1
        (150.0, 200.0),  // 敌人 2（和敌人 1 间距 50，靠近用于测试 break）
        (-100.0, 200.0), // 敌人 3
        (0.0, 250.0),    // 敌人 4
        (200.0, 150.0),  // 敌人 5
    ];
    for (x, y) in enemy_positions {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
            Transform::from_xyz(x, y, 0.0),
            Enemy,
        ));
    }

    // 底部操作提示文本
    commands.spawn((
        Text2d::new("WASD 移动 | 空格射击"),
        TextFont {
            font_size: FontSize::Px(30.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, -340.0, 0.0),
    ));
}

// 玩家移动：WASD 控制方向，帧率无关的速度（像素/秒 × dt = 本帧位移）。
pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // Single 查询：期望恰好一个 Player 实体，多个/0 个时 panic（与其他示例风格一致）
    mut player: Single<&mut Transform, With<Player>>,
) {
    // 本帧移动步长（像素）= 速度（像素/秒）× 帧时间（秒）
    let move_step = 300.0 * time.delta_secs();
    if input.pressed(KeyCode::KeyW) {
        player.translation.y += move_step;
    }
    if input.pressed(KeyCode::KeyS) {
        player.translation.y -= move_step;
    }
    if input.pressed(KeyCode::KeyA) {
        player.translation.x -= move_step;
    }
    if input.pressed(KeyCode::KeyD) {
        player.translation.x += move_step;
    }
}

// 射击：按空格发送 BulletFired 消息（携带玩家当前位置）。
// 用 MessageWriter 发送消息，spawn_bullet 系统用 MessageReader 接收。
pub fn shoot(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut writer: MessageWriter<BulletFired>,
) {
    // just_pressed 检测「本帧刚按下」，避免按住时连续发射
    // let 链（edition 2024）：if 条件中用 && 连接 let 模式匹配
    // player.single() 返回 Result，let Ok(tf) 只在恰好一个玩家时执行
    if input.just_pressed(KeyCode::Space)
        && let Ok(tf) = player.single()
    {
        writer.write(BulletFired {
            position: tf.translation.xy(),
        });
    }
}

// 生成子弹：接收 BulletFired 消息，在消息携带的位置生成子弹实体。
pub fn spawn_bullet(
    mut reader: MessageReader<BulletFired>,
    mut commands: Commands,
) {
    for bullet in reader.read() {
        commands.spawn((
            // 黄色小方块（10×20）作为子弹
            Sprite::from_color(Color::srgb(1.0, 0.9, 0.0), Vec2::new(10.0, 20.0)),
            Transform::from_xyz(bullet.position.x, bullet.position.y, 0.0),
            Bullet { speed: 500.0 }, // 速度 500 像素/秒（向上飞，见 move_bullet）
        ));
    }
}

// 子弹移动：所有子弹每帧向上移动（speed × dt）。
// 子弹永远向上飞（+Y 方向），简化设计——不需要考虑玩家朝向。
pub fn move_bullet(
    mut query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut tf, bullet) in &mut query {
        tf.translation.y += bullet.speed * time.delta_secs();
    }
}

// 清理越界子弹：子弹飞出屏幕上方后销毁，避免无用的实体堆积。
// 500.0 是屏幕上方边界（世界坐标，比默认窗口半高 360 留有余量）。
pub fn cleanup_bullets(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
) {
    for (entity, tf) in &bullets {
        if tf.translation.y > 500.0 {
            commands.entity(entity).despawn();
        }
    }
}

// 碰撞检测：子弹与敌人的距离检测，命中后销毁双方并加分。
pub fn check_collision(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for (b_entity, b_tf) in &bullets {
        for (e_entity, e_tf) in &enemies {
            // 计算子弹与敌人的 3D 距离（z=0 时等价于 2D 距离）
            let dist = b_tf.translation.distance(e_tf.translation);
            // 碰撞阈值 30.0 像素（子弹 10×20、敌人 40×40，30 是折中值）
            if dist < 30.0 {
                // 记录加分前的分数，用于日志显示变化
                let old_score = score.total;
                commands.entity(b_entity).despawn();
                commands.entity(e_entity).despawn();
                score.total += 100;
                // 详细日志：记录击中时的坐标、距离和分数变化，方便调试碰撞逻辑
                info!(
                    "[碰撞] 击中敌人\n  \
                     子弹坐标   = ({:.0}, {:.0})\n  \
                     敌人坐标   = ({:.0}, {:.0})\n  \
                     碰撞距离   = {:.1}（阈值 30.0）\n  \
                     分数变化   = {} → {}（+100）",
                    b_tf.translation.x, b_tf.translation.y,
                    e_tf.translation.x, e_tf.translation.y,
                    dist,
                    old_score, score.total,
                );
                // break：一颗子弹只能击中一个敌人！
                // commands.despawn 是延迟执行（帧末），本帧子弹仍在 Query 中，
                // 不 break 的话内层循环会继续检查同一子弹和下一个敌人，导致重复加分。
                break;
            }
        }
    }
}
