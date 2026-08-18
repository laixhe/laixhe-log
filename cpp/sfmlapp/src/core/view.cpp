#include "view.hpp"

#include <algorithm>
#include <cmath>

Camera::Camera(sf::Vector2f viewportSize, sf::Vector2f center)
    : m_baseSize(viewportSize), m_center(center)
{
}

void Camera::zoom(float factor)
{
    m_zoomLevel *= factor;
    m_zoomLevel = std::clamp(m_zoomLevel, 0.3f, 5.f);
}

void Camera::follow(sf::Vector2f target, float smoothing, float dt)
{
    // 指数插值: 目标越远移动越快, 越近越慢, 产生"平滑跟随"效果
    // 用 1 - exp(-smoothing * dt) 作插值系数, 结果与帧率无关(不会因掉帧而变慢)
    const float t = 1.f - std::exp(-smoothing * dt);
    m_center = m_center + (target - m_center) * t;
}

sf::View Camera::view() const
{
    // 视野大小 = 基础大小 / 缩放倍数: 缩放越大, 视野越小, 物体显得越大
    return sf::View(m_center, m_baseSize / m_zoomLevel);
}

sf::Vector2f Camera::screenToWorld(const sf::RenderWindow& window, sf::Vector2i screen) const
{
    return window.mapPixelToCoords(screen, view());
}

sf::Vector2i Camera::worldToScreen(const sf::RenderWindow& window, sf::Vector2f world) const
{
    return window.mapCoordsToPixel(world, view());
}
