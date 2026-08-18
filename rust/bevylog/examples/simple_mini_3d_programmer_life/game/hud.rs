//! HUD 界面：程序化纸张纹理、手账卡片（横幅 / 金钱 / 三条字章 / 技能药丸）、
//! toast 提示，以及场景管理（scene_manager：只在重置/读档时安排玩家落点）。
//! 注意：Bevy 0.19 的 `Single` 是「查询恰好匹配一个实体」的便捷写法，
//! 等价于 `Query<&mut Text, With<BannerText>>` 但保证唯一结果。

use bevy::image::Image;
use bevy::prelude::*;
use bevy::text::FontSource;

use super::components::*;
use super::resources::*;
use super::scenes;
use rand::RngExt;

// ==================== 程序化纸张纹理 ====================
// 启动时生成暖黄纸张纹理（噪点 + 纸屑斑点 + 边缘晕影），不依赖任何贴图文件。
pub fn generate_paper(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    use bevy::asset::RenderAssetUsages;
    use bevy::image::ImageSampler;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    let w = 256u32;
    let h = 256u32;
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    let mut rng = rand::rng();
    for y in 0..h {
        for x in 0..w {
            let nx = (x as f32 / (w - 1) as f32) * 2.0 - 1.0;
            let ny = (y as f32 / (h - 1) as f32) * 2.0 - 1.0;
            let dist = (nx * nx + ny * ny).sqrt();
            let vignette = 1.0 - 0.22 * (dist * 1.25).clamp(0.0, 1.0);
            let noise = (rng.random::<f32>() - 0.5) * 0.07;
            let speck = match rng.random::<f32>() {
                p if p < 0.006 => -0.16,
                p if p < 0.012 => 0.08,
                _ => 0.0,
            };
            let r = ((0.93 + noise + speck) * vignette).clamp(0.0, 1.0);
            let g = ((0.87 + noise + speck) * vignette).clamp(0.0, 1.0);
            let b = ((0.72 + noise + speck) * vignette).clamp(0.0, 1.0);
            data.extend_from_slice(&[(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, 255]);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::linear();
    commands.insert_resource(PaperTexture(images.add(image)));
}

// ==================== HUD 手账卡片 ====================
// 顶部红色牌匾横幅 + 金钱药丸 + 三枚圆形字章 + 五枚技能药丸 + 底部消息。
pub fn spawn_hud(commands: &mut Commands, assets: &Res<AssetServer>) {
    let font = FontSource::Handle(assets.load(FONT_PATH));
    let gold = Color::srgb(0.83, 0.62, 0.22);
    let red = Color::srgb(0.72, 0.15, 0.12);
    let paper_dark = Color::srgb(0.94, 0.88, 0.75);

    commands
        .spawn((
            GameRoot,
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(px(10), px(10), px(8), px(10)),
                row_gap: px(8),
                ..default()
            },
        ))
        .with_children(|root| {
            // ---- 顶部红色牌匾横幅 ----
            root.spawn((
                GameRoot,
                Node {
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|banner_row| {
                banner_row
                    .spawn((
                        GameRoot,
                        Node {
                            padding: UiRect::new(px(20), px(20), px(8), px(8)),
                            border: UiRect::all(px(2)),
                            border_radius: BorderRadius::all(px(8)),
                            ..default()
                        },
                        BorderColor::all(gold),
                        BackgroundColor(red),
                    ))
                    .with_children(|banner| {
                        banner.spawn((
                            GameRoot,
                            BannerText,
                            Text::new("第1章 大三暑期 · 第1周 · 周一 · 上午"),
                            TextColor(Color::srgb(1.0, 0.92, 0.8)),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(24.0),
                                ..default()
                            },
                        ));
                    });
            });

            // ---- 属性区：金钱药丸 + 三枚圆形字章 ----
            root.spawn((
                GameRoot,
                Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(18),
                    ..default()
                },
            ))
            .with_children(|stats_row| {
                // 金钱金锭药丸
                stats_row
                    .spawn((
                        GameRoot,
                        Node {
                            padding: UiRect::new(px(16), px(16), px(8), px(8)),
                            border_radius: BorderRadius::MAX,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.95, 0.78, 0.30)),
                    ))
                    .with_children(|pill| {
                        pill.spawn((
                            GameRoot,
                            MoneyText,
                            Text::new("¥1500"),
                            TextColor(Color::srgb(0.35, 0.22, 0.05)),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(22.0),
                                ..default()
                            },
                        ));
                    });

                // 三枚圆形字章
                let seals: [(SealKind, &str, Color); 3] = [
                    (SealKind::Energy, "精力", Color::srgb(0.92, 0.68, 0.20)),
                    (SealKind::Mentality, "心态", Color::srgb(0.42, 0.72, 0.42)),
                    (SealKind::Health, "健康", Color::srgb(0.38, 0.62, 0.88)),
                ];
                stats_row
                    .spawn((
                        GameRoot,
                        Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: px(16),
                            ..default()
                        },
                    ))
                    .with_children(|seals_row| {
                        for (which, name, fill_color) in seals {
                            seals_row
                                .spawn((
                                    GameRoot,
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        row_gap: px(2),
                                        ..default()
                                    },
                                ))
                                .with_children(|seal_col| {
                                    seal_col.spawn((
                                        GameRoot,
                                        Text::new(name),
                                        TextColor(Color::srgb(0.35, 0.28, 0.2)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FontSize::Px(15.0),
                                            ..default()
                                        },
                                    ));
                                    seal_col
                                        .spawn((
                                            GameRoot,
                                            Node {
                                                width: px(58),
                                                height: px(58),
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::FlexEnd,
                                                border: UiRect::all(px(3)),
                                                border_radius: BorderRadius::MAX,
                                                ..default()
                                            },
                                            BorderColor::all(gold),
                                            BackgroundColor(paper_dark),
                                        ))
                                        .with_children(|circle| {
                                            circle.spawn((
                                                GameRoot,
                                                SealFill { which },
                                                Node {
                                                    width: percent(100),
                                                    height: percent(90.0),
                                                    border_radius: BorderRadius::MAX,
                                                    ..default()
                                                },
                                                BackgroundColor(fill_color),
                                            ));
                                        });
                                    seal_col.spawn((
                                        GameRoot,
                                        SealValue { which },
                                        Text::new("80"),
                                        TextColor(Color::srgb(0.35, 0.28, 0.2)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: FontSize::Px(15.0),
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    });
            });

            // ---- 五枚技能药丸 ----
            root.spawn((
                GameRoot,
                Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    column_gap: px(10),
                    ..default()
                },
            ))
            .with_children(|skills_row| {
                for (idx, name) in SKILL_NAMES.iter().enumerate() {
                    skills_row
                        .spawn((
                            GameRoot,
                            Node {
                                padding: UiRect::new(px(14), px(14), px(5), px(5)),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.80, 0.88, 0.95)),
                        ))
                        .with_children(|pill| {
                            pill.spawn((
                                GameRoot,
                                SkillPill { idx },
                                Text::new(format!("{name} 18")),
                                TextColor(Color::srgb(0.2, 0.3, 0.42)),
                                TextFont {
                                    font: font.clone(),
                                    font_size: FontSize::Px(16.0),
                                    ..default()
                                },
                            ));
                        });
                }
            });

            // ---- 弹性占位，把消息推到最底部 ----
            root.spawn((
                GameRoot,
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
            ));

            // ---- 底部消息 + 操作提示 ----
            root.spawn((
                GameRoot,
                Node {
                    width: percent(100),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(4),
                    ..default()
                },
            ))
            .with_children(|toast_row| {
                toast_row
                    .spawn((
                        GameRoot,
                        Node {
                            padding: UiRect::new(px(16), px(16), px(8), px(8)),
                            border_radius: BorderRadius::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.22, 0.18, 0.12, 0.78)),
                    ))
                    .with_children(|box_| {
                        box_.spawn((
                            GameRoot,
                            ToastText,
                            Text::new(""),
                            TextColor(Color::srgb(0.98, 0.94, 0.86)),
                            TextFont {
                                font: font.clone(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                        ));
                    });
                toast_row.spawn((
                    GameRoot,
                    Text::new("点击地面移动 · 走近热点自动触发 · 晚上睡觉推进天数 · 周一交房租"),
                    TextColor(Color::srgb(0.55, 0.45, 0.32)),
                    TextFont {
                        font,
                        font_size: FontSize::Px(15.0),
                        ..default()
                    },
                ));
            });
        });
}

// ==================== HUD 每帧更新 ====================
// 注意：所有动态文本都先比较再写入，只有内容真正变化才更新。
// Bevy 0.19 的文本布局引擎对中文缺 ICU4X 分词数据，每次重写都会打警告日志，
// 无条件每帧重写（即使内容相同）会导致日志刷屏，因此必须做变更检查。

pub fn update_banner(clock: Res<GameClock>, mut text: Single<&mut Text, With<BannerText>>) {
    let new = format!(
        "第{}章 {} · 第{}周 · {} · {}",
        clock.chapter(),
        chapter_name(clock.chapter()),
        clock.week,
        day_label(clock.day),
        clock.phase.label()
    );
    if text.0 != new {
        text.0 = new;
    }
}

pub fn update_money(stats: Res<PlayerStats>, mut text: Single<&mut Text, With<MoneyText>>) {
    let new = format!("¥{:.0}", stats.money);
    if text.0 != new {
        text.0 = new;
    }
}

pub fn update_seals(
    stats: Res<PlayerStats>,
    mut fills: Query<(&mut Node, &SealFill)>,
    mut values: Query<(&mut Text, &mut TextColor, &SealValue)>,
) {
    let red = Color::srgb(0.85, 0.28, 0.26);
    let ink = Color::srgb(0.35, 0.28, 0.2);
    let (energy, mentality, health) = (stats.energy, stats.mentality, stats.health);
    for (mut node, fill) in &mut fills {
        let v = match fill.which {
            SealKind::Energy => energy,
            SealKind::Mentality => mentality,
            SealKind::Health => health,
        };
        let new = percent(v.max(0.0));
        if node.height != new {
            node.height = new;
        }
    }
    for (mut text, mut color, val) in &mut values {
        let v = match val.which {
            SealKind::Energy => energy,
            SealKind::Mentality => mentality,
            SealKind::Health => health,
        };
        let new = format!("{:.0}", v);
        if text.0 != new {
            text.0 = new;
        }
        let new_color = if v < 30.0 { red } else { ink };
        if color.0 != new_color {
            color.0 = new_color;
        }
    }
}

pub fn update_skills(stats: Res<PlayerStats>, mut pills: Query<(&mut Text, &SkillPill)>) {
    for (mut text, pill) in &mut pills {
        let new = format!("{} {:.0}", SKILL_NAMES[pill.idx], stats.skills[pill.idx]);
        if text.0 != new {
            text.0 = new;
        }
    }
}

pub fn update_toast(toast: Res<ToastLog>, mut text: Single<&mut Text, With<ToastText>>) {
    if toast.items.is_empty() {
        if !text.0.is_empty() {
            text.0.clear();
        }
        return;
    }
    let joined = toast
        .items
        .iter()
        .map(|(s, _)| s.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    if text.0 != joined {
        text.0 = joined;
    }
}

// Toast 计时
pub fn toast_tick(time: Res<Time>, mut toast: ResMut<ToastLog>) {
    let dt = time.delta_secs();
    for (_, ttl) in toast.items.iter_mut() {
        *ttl -= dt;
    }
    toast.items.retain(|(_, ttl)| *ttl > 0.0);
}

// ==================== 场景管理 ====================
// 城市地图常驻（进入游戏时构建一次）。这里只在「重置 / 读档」（force 变化）时
// 处理玩家位置：行驶中存档→回到地图原位置继续行驶；否则传送到区域出生点。
// 自由行走跨区域不会触发传送，区域由 update_location 按坐标实时判定。
pub fn scene_manager(
    location: Res<GameLocation>,
    force: Res<SceneForce>,
    mut transit: ResMut<TransitState>,
    mut resume: ResMut<SceneResume>,
    mut player: Single<&mut Transform, With<PlayerRoot>>,
    mut toast: ResMut<ToastLog>,
    mut local: Local<u32>,
) {
    if *local == force.0 {
        return;
    }
    *local = force.0;

    // 行驶中存档恢复：放回地图原位置，继续自动行驶（过程在地图上，不传送）
    if let Some((pos, t)) = resume.0.take() {
        player.translation = pos;
        player.rotation = default();
        *transit = t;
        info!(
            "[场景] 恢复行驶：位于 ({:.1}, {:.1})，继续驶向 {}",
            pos.x,
            pos.z,
            scenes::location_name(transit.target)
        );
        toast.push(format!(
            "🚉 继续行驶，前往{}…",
            scenes::location_name(transit.target)
        ));
        return;
    }

    // 普通读档 / 重置：传送到区域出生点
    let sp = scenes::spawn_pos(location.0);
    player.translation = sp;
    player.rotation = default();
    info!("[场景] 传送至 {}", scenes::location_name(location.0));
    toast.push(format!("📍 {}", scenes::location_name(location.0)));
}

// ==================== 区域判定 ====================
// 城市地图上没有"切换场景"：按主角世界坐标实时判定所在区域。
pub fn update_location(
    player: Single<&Transform, With<PlayerRoot>>,
    transit: Res<TransitState>,
    mut location: ResMut<GameLocation>,
) {
    if transit.active {
        return; // 乘车行驶中保持原区域，到站后再判定
    }
    let new = region_of(player.translation);
    if location.0 != new {
        info!("[场景] 进入 {}", scenes::location_name(new));
        location.0 = new;
    }
}
