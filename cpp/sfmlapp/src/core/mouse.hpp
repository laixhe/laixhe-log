#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Rect.hpp>
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/View.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Event.hpp>

#include <vector>

// ---------------------------------------------------------------------------
// 学习点: 鼠标交互
//   事件驱动: event->is<sf::Event::MouseButtonPressed>() 等, 只在发生瞬间触发
//   实时轮询: sf::Mouse::getPosition(window) 每帧获取光标位置(窗口内坐标)
//   坐标转换: 相机/视图下鼠标是"屏幕坐标", 用 window.mapPixelToCoords()
//             转成"世界坐标"才能与场景元素命中检测
// 演示: 光标悬停显示白色圆环, 左键点击留红色标记, 按住拖拽画选区
// ---------------------------------------------------------------------------

struct MouseDemo {
    sf::Vector2f hover{0.f, 0.f};           // 光标的世界坐标(每帧更新)
    bool dragging = false;                  // 左键拖拽中
    sf::Vector2f dragStart{0.f, 0.f};       // 拖拽起点(世界坐标)
    std::vector<sf::Vector2f> clickMarks;   // 点击产生的标记点
    std::vector<sf::FloatRect> selections;  // 拖拽完成的选区

    // 处理鼠标按键事件(按下/抬起): 记录拖拽起点, 生成选区
    void handleEvent(const sf::Event& event, const sf::RenderWindow& window, const sf::View& view);
    // 每帧轮询光标位置并换算成世界坐标(不依赖 MouseMoved 事件)
    void updateHover(const sf::RenderWindow& window, const sf::View& view);

    void draw(sf::RenderTarget& target) const;
};
