use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

/**
1. Sprite 是 bevy 提供的自定义结构体，实现 Component 特征，实体持有该组件会默认显示在当前默认的 2d 摄像头中
2. 可以通过 bevy 默认提供的资源 AssetServer 来加载文件，加载文件的启始路径是 project/assets
3. 2d 对象会在默认的 2d 摄像头中显示，不指定位置默认将中心点放置在摄像头原点位置
4. color 字段是会将指定的颜色乘到每一个像素上
5. custom_size 字段指定 Sprite 大小，不指定则大小为图片默认大小
6. rect 则用于裁剪图片的部分区域，这时候大小变为裁剪区域的大小
*/
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands
        .spawn(Sprite {
            image: asset_server.load("bevy_bird_dark.png"),
            color: Color::Srgba(Srgba::RED),
            ..Default::default()
        })
        .insert(Transform::from_xyz(0., 50., 0.));
}
