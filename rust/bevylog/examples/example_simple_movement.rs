//! Bevy 0.19 入门示例：用方向键 / WASD 移动一个带 Player 标签的文本，演示组件、资源、系统查询与帧率无关移动。

use bevy::{prelude::*, text::FontSourceTemplate};

// 中文字体路径
const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

// 玩家组件：这里给它加上 speed 字段（移动速度），
// 演示 Bevy 惯用做法——把实体自己的数据挂在组件上，而不是在系统里硬编码。
// 补充：组件即使没有字段也能用，称为「标记组件」，仅用来给实体打标签方便筛选。
// #[derive(Component)] 让这个结构体可以被挂到实体上作为组件
// bsn! 宏要求组件实现 Clone + Default（宏内部用模板反射构造实体）
#[derive(Component, Clone, Default)]
struct Player {
    // 移动速度，单位：像素/秒
    speed: f32,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 插入资源：ClearColor 决定每帧清屏的背景色（这里设为黑色）
        // 「资源」是全局共享的数据，不依附于某个实体，系统通过 Res/ResMut 读取；
        // 注意：建议先 add_plugins 再 insert_resource / add_systems，养成习惯，
        // 避免后续遇到依赖插件初始化的资源时出顺序问题。
        .insert_resource(ClearColor(Color::BLACK))
        // Startup 调度：启动时执行一次，用于初始化场景
        .add_systems(Startup, setup)
        // Update 调度：每帧执行一次，move_player 每帧读取输入并移动玩家
        .add_systems(Update, move_player)
        .run()
}

fn setup(mut commands: Commands) {
    // 生成 2D 相机（必须有一个 Camera2d 才能看到画面）
    commands.spawn(Camera2d);
    // 生成文本作为「玩家」：把文本实体挂上 Player 组件，这样 move_player
    // 就能按 Player 组件过滤找到它并读取它的 speed 字段
    // spawn_scene + bsn! 宏声明式构建实体
    commands.spawn_scene(bsn! {
        // 玩家组件：初始化速度为 300 像素/秒
        Player { speed: 300.0 }
        Text2d::new("Movement")
        TextColor(Color::WHITE)
        // TextFont：用中文字体渲染，字号 30 像素
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(30.0),
        }
        // 位置：原点（屏幕中心）
        Transform::default()
    });
}

fn move_player(
    // 键盘输入：ButtonInput 是资源，记录每个按键的按下/松开状态；Res 表示只读访问
    input: Res<ButtonInput<KeyCode>>,
    // 时间资源（提供帧时间等，注意与 Timer 组件区分）
    time: Res<Time>,
    // 查询玩家实体：Single 表示「期望恰好有一个」匹配实体（数量不是 1 会运行时报错）；
    // 这里同时取 &mut Transform（可写位置）和 &Player（只读读取速度）；
    // 查询 tuple 里包含 &Player，会自动筛选出带 Player 组件的实体（无需额外写 With<Player>）。
    // 参数本身不需要 mut：下面用 into_inner() 按值消耗 Single 取出内部 tuple，
    // 真正需要 mut 的是解构出来的 transform 绑定（见下方注释）。
    // 变量名用 player_query（而非 player_transform），因为它实际是 (Transform, Player) 元组，
    // 不仅仅是 Transform。
    player_query: Single<(&mut Transform, &Player)>,
    // 系统本地状态：每个系统实例独有一份、跨帧保留（不会共享给别的系统）。
    // 这里记录上一帧的方向，用来检测「方向是否变化」，只在变化时打印日志，避免每帧刷屏。
    // Local<Vec2> 首次调用时用 Vec2::default() 初始化（即 Vec2::ZERO）
    mut last_direction: Local<Vec2>,
) {
    // 解构查询结果：into_inner() 消耗 Single，取出内部的 tuple：
    // - transform 的类型是 &mut Transform（单层可变引用），绑定声明 mut 是 Rust 2024
    //   edition 的严格要求（通过引用修改字段时绑定本身也必须是 mut）；
    // - 不要再加 ref mut，否则会变成 &mut &mut Transform 这种不必要的双层引用。
    // - player 的类型是 &Player（只读借用）。
    let (mut transform, player) = player_query.into_inner();

    // 用一个二维向量累加方向：每个方向同时支持「方向键」和「WASD」
    // any_pressed 接收一组按键，其中任意一个按下即返回 true（比逐个 || 更简洁）
    //
    // 注意：这里用 4 个独立的 if（不是 if/else if），所以「同时按对向键」会相互抵消，
    // 例如同时按 ← 和 →，direction.x = -1 + 1 = 0，该方向不动——这是预期行为，避免混乱。
    // Vec2::ZERO == Vec2 { x: 0.0, y: 0.0 }
    let mut direction = Vec2::ZERO;
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction.x -= 1.0; // 向左 = x 减小
    }
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction.x += 1.0; // 向右 = x 增大
    }
    if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction.y += 1.0; // 向上 = y 增大（2D 中 +y 朝上）
    }
    if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction.y -= 1.0; // 向下 = y 减小
    }

    // 只在方向发生变化时打印日志（按下/松开瞬间各触发一次），避免每帧刷屏。
    // 用 info! 级别，Bevy 默认就会显示（无需配置 RUST_LOG）；
    // 若觉得太吵可改成 debug!，再用 RUST_LOG=debug 开启。
    // 日志同时打印当前屏幕位置，方便确认「真的在动」。
    if direction != *last_direction {
        let pos = transform.translation.truncate(); // truncate() 把 Vec3 去 z 轴转 Vec2
        if direction == Vec2::ZERO {
            info!("按键松开，停止移动，当前位置 = ({:.1}, {:.1})", pos.x, pos.y);
        } else {
            // 说明：direction 是原始输入向量（分量都是 -1/0/1）；
            // 下面做移动时会 normalize() 把长度归一为 1，因此斜向的 (1,1) 和
            // 直线的 (1,0) 移动速度是一样的（帧率无关 + 方向无关）。
            info!(
                "按键生效，方向 = ({}, {})，当前位置 = ({:.1}, {:.1})",
                direction.x, direction.y, pos.x, pos.y
            );
        }
        // 记下本次方向，供下一帧比较
        *last_direction = direction;
    }

    // 只有真的按了方向键才移动（同时避免对零向量 normalize 得到 NaN）
    if direction != Vec2::ZERO {
        // normalize() 将向量长度归一化为 1（这样斜向移动不会比直线快）
        // time.delta_secs() 返回帧时间（即自上一帧到当前帧的时间，单位秒）
        // player.speed（像素/秒）× delta_secs（秒/帧）= 像素/帧，
        // 这样无论帧率高低，每秒移动距离都一致（帧率无关移动）
        let delta = direction.normalize() * player.speed * time.delta_secs();
        // delta.extend(0.0) 将 Vec2 转为 Vec3（z 轴为 0），再加到 translation（位置）上
        transform.translation += delta.extend(0.0);
    }
}
