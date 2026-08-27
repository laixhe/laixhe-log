//! Bevy 0.19 入门示例：演示 moonshine 系列 —— behavior（实体状态机）+ kind（类型化实体）+ util（工具）。
//!
//! 三个 crate 的分工：
//! - moonshine-behavior：给实体做「状态机」。状态就是普通 Component（常用 enum），
//!   切换不立即生效，而是由每帧的 transition::<T> 系统统一应用。
//! - moonshine-kind：给实体加「类型」（Kind）。Instance<T> 是带类型的 Entity，
//!   查询 / 命令都带类型信息，避免把实体引用传错。
//! - moonshine-util：零散小工具。Expect<T> 保证组件必然存在（缺失直接 panic），
//!   WithChild 在插入时自动生成一个子实体。
//!
//! 学习重点：
//! - #[derive(Component, Debug, Reflect)] enum + impl Behavior for T + match_next!
//! - BehaviorPlugin::<T>::default() + 每帧 transition::<T>()（AI 系统要在它之前）
//! - BehaviorMut<T>：current() 读状态、start(...) 请求切换（含 filter_next 校验）
//! - 事件观察者：OnStart / OnStop（event.instance 是 Instance<T>）
//! - Query<InstanceRef<T>> 类型化查询；commands.instance(...) 类型化命令
//! - Expect<&mut Transform> 与 WithChild
//!
//! 操作：观察小甲虫的行为循环——漫游（随机转向）→ 睡觉 → 漫游；靠近食物 → 进食 → 漫游。

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::text::FontSourceTemplate;
use moonshine_behavior::prelude::*;
// kind 的完整 prelude：InstanceRef / GetInstanceCommands 等需要单独引入
use moonshine_kind::prelude::*;
use moonshine_util::prelude::*;

const FONT_PATH: &str = "fonts/Yozai-Regular.ttf";
const SPEED: f32 = 120.0; // 漫游速度（像素/秒）
const EAT_RANGE: f32 = 110.0; // 发现食物的距离（像素）
const EAT_DISTANCE: f32 = 24.0; // 吃到食物的距离（像素）

// ---- moonshine-behavior：行为状态（enum 是状态机的最佳载体）----
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
enum BugBehavior {
    Wandering, // 漫游
    Eating,    // 进食
    Sleeping,  // 睡觉
}

impl Behavior for BugBehavior {
    // 定义「允许的状态切换表」：不匹配的组合默认不允许
    fn filter_next(&self, next: &Self) -> bool {
        use BugBehavior::*;
        match_next! {
            self => next,
            Wandering => Eating | Sleeping,
            Eating => Wandering,
            Sleeping => Wandering,
        }
    }
}

// ---- moonshine-kind：类型化实体 ----
// 任意 Component 自动实现 Kind；Bug / Food 就是两种「实体类型」
#[derive(Component)]
struct Bug;

// 食物：自身存逻辑坐标（不读 Transform，避免和移动系统的 &mut Transform 查询冲突）
#[derive(Component)]
struct Food {
    pos: Vec2,
}

// ---- 普通组件 ----
// 漫游状态：当前方向 + 多久转向一次
#[derive(Component)]
struct Wander {
    direction: Vec2,
    timer: Timer,
}

// 状态计时器：漫游多久后睡觉、睡多久
#[derive(Component)]
struct StateTimer(Timer);

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            // 静默 icu_provider 缺 CJK 字典数据的警告（保留默认的 wgpu/naga 过滤）
            filter: "wgpu=error,naga=warn,icu_provider=off".to_string(),
            ..default()
        }))
        // 注册行为插件（注册 Transition / Memory / TransitionQueue 的反射类型）
        .add_plugins(BehaviorPlugin::<BugBehavior>::default())
        .insert_resource(ClearColor(Color::srgb(0.08, 0.09, 0.12)))
        .add_systems(Startup, setup)
        // 顺序很重要：先跑 AI 逻辑请求状态切换，再统一应用切换
        .add_systems(Update, (bug_ai, transition::<BugBehavior>).chain())
        .add_observer(on_behavior_start)
        .add_observer(on_behavior_stop)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 生成小甲虫：行为 + 状态组件 + 视觉（moonshine-util 的 WithChild 自动生成子实体）
    let body_mesh = meshes.add(Circle::new(28.0));
    let body_mat = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.6, 0.2)));
    commands.spawn((
        Bug,
        BugBehavior::Wandering, // 初始行为
        Wander {
            direction: Vec2::X,
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        StateTimer(Timer::from_seconds(4.0, TimerMode::Once)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        // 父实体要有 Visibility，子实体（WithChild 生成）的 InheritedVisibility 才能正常继承
        Visibility::default(),
        // moonshine-util：组件被插入时自动把闭包返回的 Bundle 生成为子实体
        WithChild(move |_parent| {
            (
                Mesh2d(body_mesh),
                MeshMaterial2d(body_mat),
                Transform::from_xyz(0.0, 0.0, 1.0),
            )
        }),
    ));

    // 生成若干食物（moonshine-kind：Food 类型实体）
    let food_mesh = meshes.add(Rectangle::new(22.0, 22.0));
    let food_mat = materials.add(ColorMaterial::from(Color::srgb(0.3, 0.9, 0.4)));
    for pos in [
        Vec2::new(-250.0, 120.0),
        Vec2::new(250.0, -80.0),
        Vec2::new(-150.0, -180.0),
        Vec2::new(200.0, 150.0),
    ] {
        commands.spawn((
            Food { pos },
            Mesh2d(food_mesh.clone()),
            MeshMaterial2d(food_mat.clone()),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }

    // 底部提示文本
    commands.spawn_scene(bsn! {
        Text2d::new("状态机演示：漫游 / 进食 / 睡觉（观察终端日志）")
        TextColor(Color::WHITE)
        TextFont {
            font: FontSourceTemplate::Handle(FONT_PATH),
            font_size: FontSize::Px(22.0),
        }
        Transform::from_xyz(0.0, -280.0, 0.0)
    });
}

// AI 逻辑：按当前状态行动，并请求状态切换（transition 系统在下一帧应用）
fn bug_ai(
    time: Res<Time>,
    // 组合查询：行为 + 类型化实体 + 保证存在的 Transform + 两个状态组件
    bug: Single<(
        BehaviorMut<BugBehavior>,
        Expect<&mut Transform>,
        &mut Wander,
        &mut StateTimer,
    )>,
    // moonshine-kind：InstanceRef<Food> = (Instance<Food>, &Food)，位置从 Food.pos 读
    foods: Query<InstanceRef<Food>>,
    mut commands: Commands,
) {
    use BugBehavior::*;

    let (mut behavior, mut transform, mut wander, mut state_timer) = bug.into_inner();
    let delta = time.delta_secs();

    match behavior.current() {
        // ---- 漫游：沿方向移动，定时转向；到点睡觉；附近有食物就去吃 ----
        Wandering => {
            // 每 2 秒转一次方向（顺时针 60°），避免走出屏幕
            wander.timer.tick(time.delta());
            if wander.timer.is_finished() {
                wander.direction = rotate_60(wander.direction);
                wander.timer.reset();
            }
            transform.translation += wander.direction.extend(0.0) * SPEED * delta;

            // 到点睡觉
            state_timer.0.tick(time.delta());
            if state_timer.0.is_finished() {
                info!("[状态机] 漫游够了，开始睡觉");
                behavior.start(Sleeping);
                *state_timer = StateTimer(Timer::from_seconds(2.0, TimerMode::Once));
                return;
            }

            // 附近有食物 → 去进食
            let bug_pos = transform.translation.truncate();
            let food_nearby = foods.iter().any(|food| food.pos.distance(bug_pos) < EAT_RANGE);
            if food_nearby {
                info!("[状态机] 发现食物，去吃！");
                behavior.start(Eating);
            }
        }

        // ---- 进食：朝最近的食物移动，够近就吃掉 ----
        Eating => {
            let bug_pos = transform.translation.truncate();
            // 一次遍历找到最近的食物（保存 Instance<Food>，位置，距离）
            let mut nearest: Option<(Instance<Food>, Vec2, f32)> = None;
            for food in &foods {
                let pos = food.pos;
                let d = pos.distance(bug_pos);
                if nearest.is_none_or(|(_, _, nd)| d < nd) {
                    nearest = Some((food.instance(), pos, d));
                }
            }
            let Some((food, food_pos, distance)) = nearest else {
                // 没有食物了，回到漫游
                behavior.start(Wandering);
                return;
            };

            if distance <= EAT_DISTANCE {
                info!("[状态机] 吃到食物！");
                // moonshine-kind：commands.instance(...) 生成类型化命令，despawn 一个 Food 实例
                commands.instance(food).despawn();
                behavior.start(Wandering);
            } else {
                // 朝食物方向加速移动
                let toward = (food_pos - bug_pos).normalize_or_zero();
                transform.translation += toward.extend(0.0) * SPEED * 1.5 * delta;
            }
        }

        // ---- 睡觉：原地不动，睡够回到漫游 ----
        Sleeping => {
            state_timer.0.tick(time.delta());
            if state_timer.0.is_finished() {
                info!("[状态机] 睡醒了，继续漫游");
                behavior.start(Wandering);
                *state_timer = StateTimer(Timer::from_seconds(4.0, TimerMode::Once));
            }
        }
    }
}

// 把方向向量顺时针旋转 60°（cos60°=0.5，sin60°≈0.866）
fn rotate_60(v: Vec2) -> Vec2 {
    Vec2::new(v.x * 0.5 - v.y * 0.866, v.x * 0.866 + v.y * 0.5)
}

// 事件观察者：状态切换时打印日志（event.instance 是 moonshine-kind 的 Instance<T>）
fn on_behavior_start(
    event: OnStart<BugBehavior>,
    query: Query<BehaviorRef<BugBehavior>>,
) {
    let Ok(behavior) = query.get(event.instance.entity()) else {
        return;
    };
    info!(
        "[事件] {:?} 开始：{:?}",
        event.instance.entity(),
        &behavior[event.index]
    );
}

fn on_behavior_stop(event: OnStop<BugBehavior>) {
    info!(
        "[事件] {:?} 停止：{:?}",
        event.instance.entity(),
        event.behavior
    );
}
