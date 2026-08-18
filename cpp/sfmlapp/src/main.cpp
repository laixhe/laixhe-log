#include <spdlog/cfg/env.h>
#include <spdlog/spdlog.h>

#include <SFML/Graphics.hpp>
#include <SFML/System/Angle.hpp>
#include <SFML/System/Clock.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Mouse.hpp>

#include <filesystem>
#include <string>

#include "core/audio.hpp"
#include "core/collision.hpp"
#include "core/mouse.hpp"
#include "core/particles.hpp"
#include "core/player.hpp"
#include "core/shapes.hpp"
#include "core/sprite.hpp"
#include "core/text.hpp"
#include "core/vertex_array.hpp"
#include "core/view.hpp"

int main()
{
    // 默认 info 级别, 按键/鼠标/滚轮等高频事件用 debug; 可用环境变量 SPDLOG_LEVEL=debug 开启
    spdlog::set_level(spdlog::level::info);
    spdlog::cfg::load_env_levels();

    // ---- 窗口创建 ----
    constexpr sf::Vector2u kWindowSize{800, 600};
    spdlog::info("正在创建窗口 ...");
    sf::RenderWindow window(sf::VideoMode(kWindowSize), "SFML Learning Sample");
    window.setFramerateLimit(60);
    spdlog::info("窗口创建完成: {}x{}", kWindowSize.x, kWindowSize.y);

    // ---- 文本与字体 ----
    TextRenderer text;
    if (!text.loadFont("C:/Windows/Fonts/simhei.ttf")) {
        spdlog::warn("无法加载字体 simhei.ttf, 界面文本将被禁用");
    }

    // ---- 音频(程序化生成 WAV, 无需外部音频文件) ----
    SoundManager sfx;
    const std::filesystem::path tmp = std::filesystem::temp_directory_path();
    const std::string beepPath = (tmp / "sfmlapp_beep.wav").string();
    const std::string musicPath = (tmp / "sfmlapp_music.wav").string();
    // 点击提示音: 880Hz 短促一声
    if (generateWav(beepPath, {{880.f, 0.12f}})) {
        if (sfx.loadSound(beepPath)) {
            spdlog::info("音效加载完成: {}", beepPath);
        } else {
            spdlog::warn("音效加载失败: {}", beepPath);
        }
    } else {
        spdlog::warn("程序化生成音效 WAV 失败");
    }
    // 背景音乐: C 大调琶音(523.25=E5 前的 C5... 见下行注释), 循环播放
    // C5(523.25) E5(659.25) G5(783.99) C6(1046.5)
    if (generateWav(musicPath, {{523.25f, 0.25f}, {659.25f, 0.25f}, {783.99f, 0.25f}, {1046.5f, 0.5f}})) {
        if (sfx.openMusic(musicPath, true)) {
            sfx.startMusic();
            spdlog::info("背景音乐开始循环播放: {}", musicPath);
        } else {
            spdlog::warn("背景音乐加载失败: {}", musicPath);
        }
    } else {
        spdlog::warn("程序化生成背景音乐 WAV 失败");
    }

    // ---- 纹理与精灵(程序化棋盘格纹理) ----
    const sf::Texture checker = makeCheckerTexture({128, 128}, 16, sf::Color::White, sf::Color(0, 120, 255));
    sf::Sprite sprite = makeSprite(checker, {700.f, 120.f}, 1.5f);
    sprite.setColor(sf::Color(255, 255, 255, 200));   // 半透明白色着色

    // ---- 目标矩形(来自 core/shapes) + 玩家(键盘控制) ----
    constexpr sf::Vector2f kTargetPos{600.f, 430.f};
    const sf::Vector2f kTargetHalf{50.f, 40.f};
    auto target = makeRectangle({kTargetHalf.x * 2.f, kTargetHalf.y * 2.f}, kTargetPos, sf::Color(0, 120, 255));
    Player player;

    // ---- VertexArray: 世界网格 + 闪烁星星场 + 旋转五角星 ----
    constexpr sf::Vector2f kWorldSize{1200.f, 900.f};
    const sf::VertexArray grid = makeGrid(kWorldSize, 50.f, sf::Color(60, 60, 60));
    TwinkleStarField stars(60, kWorldSize);
    const sf::Vector2f kDemoStarPos{120.f, 150.f};
    const sf::VertexArray demoStar = makeStar(kDemoStarPos, 60.f, 25.f, 5, sf::Color(255, 200, 60));

    // ---- 鼠标交互 ----
    MouseDemo mouse;

    // ---- 粒子系统(爆炸效果) ----
    ParticleSystem explosion;

    // ---- 视图/相机: 视口=窗口尺寸, 平滑跟随玩家 ----
    Camera camera({static_cast<float>(kWindowSize.x), static_cast<float>(kWindowSize.y)}, player.position);

    bool wasColliding = false;
    float spriteRotation = 0.f;
    float starRotation = 0.f;

    sf::Clock deltaClock;
    spdlog::info("进入主循环 (WASD/方向键移动玩家, 滚轮缩放视野, 左键点击/拖拽)");
    while (window.isOpen()) {
        // ---- 事件处理 ----
        while (const auto event = window.pollEvent()) {
            if (event->is<sf::Event::Closed>()) {
                spdlog::info("收到窗口关闭事件, 准备退出");
                window.close();
            } else if (event->is<sf::Event::KeyPressed>()) {
                const auto* key = event->getIf<sf::Event::KeyPressed>();
                spdlog::debug("按下按键: code={}", static_cast<int>(key->code));
            } else if (event->is<sf::Event::MouseButtonPressed>()) {
                const auto* btn = event->getIf<sf::Event::MouseButtonPressed>();
                mouse.handleEvent(*event, window, camera.view());
                if (btn->button == sf::Mouse::Button::Left) {
                    sfx.playSound();   // 点击提示音
                    // 粒子爆炸: 在点击的世界坐标处引爆
                    const sf::Vector2f world = camera.screenToWorld(window, btn->position);
                    explosion.explode(world, 80, 350.f, 1.2f, sf::Color(255, 160, 40));
                    spdlog::info("点击引爆粒子 x80 @ ({:.0f},{:.0f})", world.x, world.y);
                }
            } else if (event->is<sf::Event::MouseButtonReleased>()) {
                mouse.handleEvent(*event, window, camera.view());
                spdlog::debug("鼠标按键释放");
            } else if (event->is<sf::Event::MouseWheelScrolled>()) {
                const auto* wheel = event->getIf<sf::Event::MouseWheelScrolled>();
                camera.zoom(wheel->delta > 0.f ? 1.1f : 1.f / 1.1f);   // 滚轮缩放视野
                spdlog::debug("滚轮缩放: x{:.2f}", camera.zoomLevel());
            }
        }

        const float dt = deltaClock.restart().asSeconds();
        const int fps = dt > 0.f ? static_cast<int>(1.f / dt) : 0;

        // 鼠标悬停位置(每帧轮询, 世界坐标)
        mouse.updateHover(window, camera.view());

        // ---- 键盘控制移动(世界边界) ----
        player.handleInput(dt, kWorldSize);

        // ---- 碰撞检测(圆 vs 矩形) ----
        const bool hit = circleRectCollide(player.position, player.radius, kTargetPos, kTargetHalf);
        if (hit && !wasColliding) {
            spdlog::info("进入碰撞!");
            // 碰撞瞬间也引爆粒子(蓝色)
            explosion.explode(player.position, 120, 400.f, 1.5f, sf::Color(0, 200, 255));
        } else if (!hit && wasColliding) {
            spdlog::info("离开碰撞");
        }
        wasColliding = hit;
        player.collided = hit;

        // ---- 相机平滑跟随玩家(指数插值) ----
        camera.follow(player.position, 3.f, dt);

        // ---- 动画: 星星闪烁 / 精灵与五角星旋转 / 粒子爆炸 ----
        stars.update(dt);
        explosion.update(dt);
        spriteRotation += 60.f * dt;
        sprite.setRotation(sf::degrees(spriteRotation));
        starRotation += 30.f * dt;

        // ---- 世界空间绘制(相机视野) ----
        window.clear(sf::Color(25, 25, 25));
        camera.apply(window);
        window.draw(grid);          // VertexArray 网格
        stars.draw(window);         // VertexArray 闪烁星星场
        window.draw(target);        // 碰撞目标
        window.draw(sprite);        // 旋转棋盘格精灵
        // sf::Transform 整体旋转五角星(顶点数组不能像 Sprite 直接 setRotation)
        window.draw(demoStar, sf::Transform().rotate(sf::degrees(starRotation), kDemoStarPos));
        player.draw(window);        // 玩家(碰撞时变红)
        mouse.draw(window);         // 悬停圆环 + 点击标记 + 选区
        explosion.draw(window);     // 爆炸粒子(最上层世界物体)

        // ---- 屏幕空间绘制(UI 文字, 不随相机移动) ----
        window.setView(window.getDefaultView());
        text.draw(window, "WASD/方向键 移动 | 滚轮缩放视野 | 左键点击=爆炸 / 拖拽选区", {20.f, 20.f}, 20, sf::Color::White);
        text.draw(window, "蓝矩形: 碰撞目标 | 旋转方块: 精灵 | 金色星: VertexArray | 亮点: 闪烁星星", {20.f, 48.f}, 16, sf::Color(200, 200, 200));
        if (hit) {
            text.draw(window, "碰撞!", {20.f, 76.f}, 28, sf::Color::Red);
        }
        text.draw(window, "帧率: " + std::to_string(fps) + " FPS | 视野缩放: x" + std::to_string(camera.zoomLevel()),
                  {20.f, 570.f}, 16, sf::Color(200, 200, 200));
        window.display();
    }

    sfx.stopMusic();
    spdlog::info("程序退出");
}
