//! Bevy 0.19 入门示例：演示 UI 网格布局（CSS Grid）。
//!
//! Bevy 的 UI 除了 Flexbox，还支持 CSS Grid 布局：
//! 用 grid_template_columns / grid_template_rows 定义行列轨道，
//! 子节点会自动填充，也可以用 grid_row / grid_column 显式指定位置、跨行跨列。
//!
//! 学习重点：
//! - Display::Grid 开启网格布局
//! - grid_template_columns / grid_template_rows 定义行列轨道（fr 表示等分）
//! - RepeatedGridTrack::fr 重复 N 个等分轨道
//! - GridPlacement::start / start_span 显式放置、跨行跨列
//! - 自动放置：未指定位置的子节点按顺序填入空格子

use bevy::{prelude::*, text::FontSourceTemplate};

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.12)))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 根节点：一个 3 行 × 3 列的网格，占满全屏
    commands
        .spawn(Node {
            display: Display::Grid,
            width: percent(100),
            height: percent(100),
            // 3 列，每列 1fr（等分剩余空间）
            grid_template_columns: vec![RepeatedGridTrack::fr(3, 1.0)],
            // 3 行，每行 1fr
            grid_template_rows: vec![RepeatedGridTrack::fr(3, 1.0)],
            // 行列之间的间距
            column_gap: px(8),
            row_gap: px(8),
            padding: UiRect::all(px(8)),
            ..default()
        })
        .with_children(|parent| {
            // 标题格子：显式放置在第 1 行，横跨 3 列（演示 start_span 跨列）
            parent.spawn((
                Node {
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start_span(1, 3),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.25, 0.4)),
            ));

            // 6 个自动放置的彩色格子：未指定位置，自动按顺序填满第 2、3 行
            for i in 0..6 {
                parent.spawn((Node::default(), BackgroundColor(cell_color(i))));
            }
        });

    // 底部提示文本（世界坐标）
    commands.spawn_scene(bsn! {
        Text2d::new("UI Grid 网格布局：第 1 行标题跨 3 列，其余格子自动填充")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(24.0),
        }
        Transform::from_xyz(0.0, -260.0, 0.0)
    });
}

// 根据索引生成不同颜色（HSL 色相均匀分布，视觉上更好区分）
fn cell_color(i: usize) -> Color {
    let hue = (i as f32 * 40.0) % 360.0;
    Color::hsl(hue, 0.7, 0.6)
}
