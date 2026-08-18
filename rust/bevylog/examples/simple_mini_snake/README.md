## 技术栈

| 依赖  | 版本         | 用途                      |
| ---- | ----------- | ------------------------- |
| bevy | 0.19        | 游戏引擎（ECS、UI、场景） |
| rand | 0.10        | 随机空位生成食物          |

## 架构设计

数据与渲染分离，ECS 组件驱动。

### 状态机

```
Loading → Menu → Playing ⇄ GameOver
                ↑          ↓
                └──────────┘
```

4 个页面通过 `GameState` 状态驱动切换：

- **Loading** — 短暂启动画面（计时器后进入菜单）
- **Menu** — 主菜单（标题 + 开始游戏按钮，BSN 场景）
- **Playing** — 游戏核心逻辑（移动管线、碰撞、食物、分数）
- **GameOver** — 结算界面（分数展示、重新开始/返回菜单）

### 核心组件

| 组件                 | 用途                                 |
| -------------------- | ------------------------------------ |
| `SnakeHead`          | 标记蛇头                             |
| `SnakeBody`          | 标记蛇身段                           |
| `Tail`               | 标记蛇尾（用于吃食物时增长定位）     |
| `Block { color }`    | 可渲染块 — 数据层，自动映射到 Sprite |
| `Position(i32, i32)` | 逻辑网格坐标                         |
| `PreviousPosition`   | 上一步位置（蛇身跟随链用）           |
| `Follow(Entity)`     | 跟随目标（蛇身跟随前一段的轨迹）     |
| `Direction`          | 移动方向枚举                         |
| `Speed(u32)`         | 每 tick 移动格数                     |
| `Food`               | 食物标记                             |
| `PlayerControl`      | 玩家控制的标记                       |

### 移动管线（按顺序 chain 执行）

```
input_direction       ← 键盘输入 → Direction
    ↓
snapshot_positions    ← 保存所有位置到 PreviousPosition
    ↓
move_system           ← Direction + Speed → 更新 Position
    ↓
snake_follow_system   ← Follow 链 → 蛇身跟随
    ↓
collision_system      ← 撞墙/撞自己 → GameOver
    ↓
eating_system         ← 蛇头 vs 食物 → 增长 + 加分
    ↓
spawn_food_system     ← 食物不足时随机生成
    ↓
sync_positions        ← Position → Transform.translation
    ↓
block_render_system   ← Block → Sprite 自动渲染
```

### 数据驱动渲染

`Block { color }` 组件的生命周期自动管理渲染：

- **Added** → 自动插入 `Sprite + Transform + Visibility`
- **Changed** → 自动更新 `Sprite.color`
- **Removed** → 自动移除渲染组件

无需手动维护渲染层，数据变，渲染自动跟着变。

### 蛇身跟随算法

```
Head (Position) → Follow(Head) → body1 (Position = Head.PreviousPosition)
                                   ↓
                                 Follow(body1) → body2 (Position = body1.PreviousPosition)
                                                   ↓
                                                 Follow(body2) → body3 (Tail)
```

每 tick 先 snapshot 所有位置，蛇身段取前一段的 PreviousPosition 作为自己的新位置。

### 吃食物增长

蛇尾标记了 `Tail`，食物命中时：

1. 在蛇尾的 PreviousPosition 生成新身体段
2. 新段拥有 `Tail` + `Follow(旧蛇尾)`
3. 旧蛇尾移除 `Tail` 标记

## 配置参数

`examples/simple_mini_snake/pages/game.rs` 顶部可调整：

| 常量           | 默认值 | 说明                 |
| -------------- | ------ | -------------------- |
| `CELL_SIZE`    | 25.0   | 每格像素大小         |
| `GRID_SIZE`    | 2      | 网格线宽             |
| `ARENA_WIDTH`  | 30     | 游戏区域宽（格数）   |
| `ARENA_HEIGHT` | 20     | 游戏区域高（格数）   |
| `MoveTimer`    | 0.15s  | 移动间隔（越小越快） |

## 项目结构

```
examples/simple_mini_snake/
├── main.rs              # 应用入口，注册所有系统和资源
└── pages/
    ├── mod.rs           # 模块声明
    ├── router.rs        # GameState 状态枚举
    ├── loading.rs       # 加载页面
    ├── menu.rs          # 主菜单（BSN 场景）
    ├── game.rs          # 核心游戏逻辑
    └── gameover.rs      # 游戏结束页面（BSN 场景）

中文字体：项目根目录 assets/fonts/Yozai-Regular.ttf（与其它示例共用）
```