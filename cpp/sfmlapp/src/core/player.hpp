#pragma once

#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Keyboard.hpp>

// ---------------------------------------------------------------------------
// 学习点: 键盘实时输入(sf::Keyboard::isKeyPressed)驱动移动
// 与"事件驱动"的区别: isKeyPressed 每帧轮询, 适合持续按住移动;
// 事件(KeyPressed)只在按下瞬间触发, 适合单击操作
// ---------------------------------------------------------------------------

struct Player {
    sf::Vector2f position{400.f, 300.f};
    float radius = 30.f;
    float speed = 300.f;   // 像素/秒
    sf::Color normalColor = sf::Color::Green;
    bool collided = false; // 由外部碰撞检测设置, 用于变色提示

    // 处理 WASD / 方向键输入, 并限制在窗口内(bounds = 窗口尺寸)
    void handleInput(float dt, sf::Vector2f bounds);

    void draw(sf::RenderTarget& target) const;
};
