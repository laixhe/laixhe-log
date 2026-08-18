//! Bevy 0.19 入门示例：演示自定义关系组件（Relationship / RelationshipTarget）。
//!
//! Bevy 0.19 把「关系」抽象成了可复用的机制，除了内置的 ChildOf / Children（父子），
//! 你还能定义自己的关系。本例定义一个「跟随」关系：
//!   - `Follows(Entity)`：关系组件，挂在跟随者上、指向被跟随者（一对一）
//!   - `Followers(Vec<Entity>)`：关系目标组件，Bevy 自动维护，存在被跟随者上（一对多）
//!
//! 学习重点：
//! - #[relationship(relationship_target = ...)] 定义关系（字段需 pub）
//! - #[relationship_target(relationship = ...)] 定义关系目标（字段自动维护）
//! - spawn(Follows(leader)) 时，Bevy 自动给 leader 挂上 Followers 列表
//! - 自定义关系 vs 内置 ChildOf/Children：机制相同，语义自定

use bevy::prelude::*;

// 自定义关系：跟随者 → 被跟随者
#[derive(Component)]
#[relationship(relationship_target = Followers)]
struct Follows(pub Entity);

// 关系目标：被跟随者上自动维护的跟随者列表
#[derive(Component)]
#[relationship_target(relationship = Follows)]
struct Followers(Vec<Entity>);

// 被跟随者标记
#[derive(Component)]
struct Leader;

// 名字组件：用于日志打印
#[derive(Component)]
struct Named(&'static str);

// 保存被跟随者实体
#[derive(Resource)]
struct LeaderEntity(Entity);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        .add_systems(Update, report_followers)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // 被跟随者（队长）
    let leader = commands.spawn((Leader, Named("队长"))).id();

    // 三个跟随者，通过 Follows 关系指向队长。
    // spawn(Follows(leader)) 会自动给 leader 挂上 Followers([a, b, c])。
    commands.spawn((Named("跟随者A"), Follows(leader)));
    commands.spawn((Named("跟随者B"), Follows(leader)));
    commands.spawn((Named("跟随者C"), Follows(leader)));

    commands.insert_resource(LeaderEntity(leader));
}

// 每秒遍历队长的 Followers 列表，验证关系自动维护正确
fn report_followers(
    leader: Res<LeaderEntity>,
    followers: Query<&Followers>,
    names: Query<&Named>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    if time.elapsed_secs() - *last < 1.0 {
        return;
    }
    *last = time.elapsed_secs();

    if let Ok(list) = followers.get(leader.0) {
        let mut buf = String::new();
        for (i, e) in list.0.iter().enumerate() {
            if i > 0 {
                buf.push('、');
            }
            buf.push_str(names.get(*e).map(|n| n.0).unwrap_or("?"));
        }
        info!("[关系] 队长的跟随者列表: {}", buf);
    }
}
