//! Bevy 0.19 入门示例：演示 UI 系统（Node / Button / Interaction / Text）。
//! 屏幕中央一个按钮，点击计数 +1，按钮颜色随悬停 / 按下状态变化。
//!
//! 学习重点：
//! - UI 用的是「屏幕坐标系」（固定在屏幕上），区别于 Text2d 的「世界坐标系」（随相机移动）。
//! - Node 是 UI 布局组件，用 Flexbox 布局（和 CSS Flexbox 概念一致）。
//! - Button 是标记组件，自动 require Node + FocusPolicy + Interaction，
//!   所以 spawn(Button) 就能得到一个可交互的按钮。
//! - Interaction 由 Bevy 自动更新（Pressed / Hovered / None），用 Changed<Interaction> 过滤只处理变化的实体。
//! - children! 宏用于声明 UI 父子层级（和 spawn 后手动 add_child 等价但更简洁）。

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // 插入资源：ClickCount 记录按钮被点击的次数。
        // 「资源」是全局共享的数据，UI 系统和计数逻辑都通过 ResMut 读写它。
        .insert_resource(ClickCount(0))
        // Startup 调度：启动时执行一次，初始化 UI 场景
        .add_systems(Startup, setup)
        // Update 调度：每帧执行一次，button_system 检测按钮交互状态变化并响应
        .add_systems(Update, button_system)
        .run()
}

// 点击计数资源：包裹一个 u32，记录按钮被点击的次数。
// 资源用 #[derive(Resource)] 派生，系统通过 Res（只读）/ ResMut（可写）访问。
#[derive(Resource)]
struct ClickCount(u32);

// 按钮三种状态的颜色常量（避免魔法数字，且方便统一调整配色）
// srgb 的三个参数是 R/G/B，范围 0.0~1.0
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15); // 默认：深灰
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25); // 悬停：浅灰
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35); // 按下：绿色

fn setup(mut commands: Commands) {
    // 生成 2D 相机：Camera2d 既能渲染 2D 世界内容，也能渲染 UI（不需要单独的 UI 相机）
    commands.spawn(Camera2d);

    // 生成 UI 根节点：一个铺满整个屏幕的容器，负责把按钮居中。
    // UI 的层级是树形结构：根 Node → 子 Button → 子 Text。
    commands.spawn((
        Node {
            // width / height 用 percent(100) 表示占父容器 100%（根节点即占满整个窗口）。
            // percent 和 px 是 Bevy 0.19 的便捷函数，等价于 Val::Percent(100.0) / Val::Px(150.0)
            width: percent(100),
            height: percent(100),
            // align_items 控制交叉轴子元素对齐：Center 表示垂直居中。
            // justify_content 控制主轴子元素对齐：Center 表示水平居中。
            // 这里主轴是水平、交叉轴是垂直，因为 flex_direction 默认是 Row（水平排列）；
            // 若改成 Column（垂直排列），主轴 / 交叉轴会互换。
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        // children! 宏声明子节点：这里只有一个 Button 子节点
        children![(
            // Button 是标记组件，自动 require Node + FocusPolicy::Block + Interaction。
            // 所以这里不需要显式写 Node 和 Interaction——Button 会自动带上它们。
            Button,
            // Node 组件定义按钮自身的布局和样式（虽然 Button 自动 require 了 Node，
            // 但我们这里显式写 Node 来定制按钮的尺寸和样式）。
            Node {
                width: px(150),
                height: px(65),
                // 按钮的边框宽度 5 像素（UiRect::all 表示四条边都一样）
                border: UiRect::all(px(5)),
                // 让按钮内的文字水平居中
                justify_content: JustifyContent::Center,
                // 让按钮内的文字垂直居中
                align_items: AlignItems::Center,
                // 圆角半径设为 MAX 让按钮变成完全圆角（胶囊形）
                border_radius: BorderRadius::MAX,
                ..default()
            },
            // 边框颜色：BorderColor::all 让四条边都是指定颜色
            BorderColor::all(Color::WHITE),
            // 背景色：初始用 NORMAL_BUTTON（深灰）
            BackgroundColor(NORMAL_BUTTON),
            // 按钮的子节点：一段文本，显示点击次数
            children![(
                // Text 是 UI 文本组件（区别于世界中的 Text2d）。
                // Text 实现了 Deref<Target=String>，所以可以用 **text = "...".to_string() 改文本内容
                Text::new("点击次数：0"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )]
        )],
    ));
}

// 按钮交互系统：检测 Interaction 变化并响应。
fn button_system(
    // 查询所有 Interaction 发生变化的按钮实体。
    // Changed<Interaction> 过滤器：只返回 Interaction 组件本帧发生变化的实体，
    // 避免每帧遍历所有按钮（性能优化，按钮多时尤其重要）。
    // 同时取出 BackgroundColor（改颜色）、Children（找文本子节点改文字）。
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        Changed<Interaction>,
    >,
    // 查询文本组件：用于修改按钮上的文字。
    // 这里单独用一个 Query 而不是和上面合并，是因为 Text 在子节点上，需要先用 Children 找到子实体再查。
    mut text_query: Query<&mut Text>,
    // 点击计数资源：ResMut 表示可写访问
    mut count: ResMut<ClickCount>,
) {
    // 遍历所有 Interaction 变化的按钮（本示例只有一个按钮）
    for (interaction, mut bg_color, children) in &mut interaction_query {
        match *interaction {
            // 按下状态：计数 +1，背景变绿，文字更新
            Interaction::Pressed => {
                count.0 += 1;
                *bg_color = PRESSED_BUTTON.into();
                info!("[UI] 按钮被按下，计数 = {}", count.0);
            }
            // 悬停状态：背景变浅灰（不计数，只是视觉反馈）
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON.into();
            }
            // 默认状态（鼠标移开 / 松开）：背景恢复深灰
            Interaction::None => {
                *bg_color = NORMAL_BUTTON.into();
            }
        }

        // 更新按钮上的文字：children[0] 是按钮的第一个子节点（即 Text 实体）。
        // text_query.get_mut(children[0]) 拿到 Text 组件的可写引用。
        // **text 解两次引用（QueryItem 是 Mut<Text>，Text Deref 到 String），然后赋新值。
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            **text = format!("点击次数：{}", count.0);
        }
    }
    // 注意：Interaction 由 Bevy 的 UI 系统自动更新（基于鼠标位置和点击状态），
    // 我们只需要读取并响应。不需要手动设置 Interaction。
}
