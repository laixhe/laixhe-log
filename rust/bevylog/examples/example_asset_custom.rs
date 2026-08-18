//! Bevy 0.19 入门示例：演示自定义 Asset + AssetLoader。
//!
//! 学习重点：
//! - #[derive(Asset, TypePath)] 定义自定义资产类型
//! - 实现 AssetLoader trait：从文件字节解析出自定义资产
//! - init_asset + init_asset_loader 注册资产与加载器
//! - AssetServer::load 用自定义加载器加载 .level 文件
//!
//! 观察：从 assets/level1.level 加载关卡数据，日志打印关卡名和敌人数量。

use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bevy::reflect::TypePath;

// 自定义资产：一个简单的关卡配置
#[derive(Asset, TypePath, Debug)]
struct LevelData {
    name: String,
    enemies: u32,
}

// 自定义加载器
#[derive(Default, TypePath)]
struct LevelLoader;

impl AssetLoader for LevelLoader {
    type Asset = LevelData;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // 读取文件全部字节
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // 转成 UTF-8 文本
        let text = String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        // 文本格式：第一行关卡名，第二行敌人数量
        let mut lines = text.lines();
        let name = lines.next().unwrap_or("未知").trim().to_string();
        let enemies = lines.next().unwrap_or("0").trim().parse().unwrap_or(0);

        Ok(LevelData { name, enemies })
    }

    fn extensions(&self) -> &[&str] {
        &["level"]
    }
}

#[derive(Resource)]
struct Level(Handle<LevelData>);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_asset::<LevelData>()
        .init_asset_loader::<LevelLoader>()
        .add_systems(Startup, setup)
        .add_systems(Update, report)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut level: ResMut<Level>) {
    commands.spawn(Camera2d);
    // 用自定义加载器加载 .level 文件
    level.0 = asset_server.load("level1.level");
}

fn report(level: Res<Level>, levels: Res<Assets<LevelData>>, mut printed: Local<bool>) {
    if *printed {
        return;
    }
    // 从 Assets<LevelData> 容器取出已加载的资产
    if let Some(data) = levels.get(&level.0) {
        info!(
            "[自定义资产] 关卡名: {} 敌人数量: {}",
            data.name, data.enemies
        );
        *printed = true;
    }
}
