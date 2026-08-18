#pragma once

#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/View.hpp>
#include <SFML/System/Vector2.hpp>

// ---------------------------------------------------------------------------
// 学习点: 视图 sf::View (相机)
//   View 决定"窗口能看到世界中的哪一块": 由 center(中心) + size(视野大小) 组成
//   window.setView(view) 之后绘制的是世界坐标; setView(默认视图) 回到屏幕坐标
//   mapPixelToCoords: 屏幕坐标 -> 世界坐标 (点击命中检测必需)
//   mapCoordsToPixel: 世界坐标 -> 屏幕坐标 (小地图/血条跟随等)
//   缩放: 视野 size 越小, 看到的世界越小, 物体显得越大(=放大)
// 演示: 平滑跟随目标(指数插值, 与帧率无关)、滚轮缩放
// ---------------------------------------------------------------------------

class Camera {
public:
    // viewportSize 窗口/视口尺寸, center 初始中心(世界坐标)
    explicit Camera(sf::Vector2f viewportSize, sf::Vector2f center = {0.f, 0.f});

    void setCenter(sf::Vector2f c) { m_center = c; }
    sf::Vector2f center() const { return m_center; }

    void move(sf::Vector2f delta) { m_center += delta; }

    // 缩放: factor > 1 放大, < 1 缩小; 内部累积并限制在 0.3x ~ 5x
    void zoom(float factor);
    float zoomLevel() const { return m_zoomLevel; }

    // 平滑跟随目标: smoothing 越大跟随越快(指数插值, 与帧率无关)
    void follow(sf::Vector2f target, float smoothing, float dt);

    // 根据当前中心/缩放生成 View
    sf::View view() const;
    void apply(sf::RenderWindow& window) const { window.setView(view()); }

    // 坐标互转(依赖 window 的实际尺寸做映射)
    sf::Vector2f screenToWorld(const sf::RenderWindow& window, sf::Vector2i screen) const;
    sf::Vector2i worldToScreen(const sf::RenderWindow& window, sf::Vector2f world) const;

private:
    sf::Vector2f m_baseSize;   // 未缩放时的视野大小
    sf::Vector2f m_center;     // 视野中心(世界坐标)
    float m_zoomLevel = 1.f;
};
