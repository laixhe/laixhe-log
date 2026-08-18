#include "shapes.hpp"

#include <SFML/System/Angle.hpp>

#include <cmath>

namespace {
constexpr float kPi = 3.14159265f;
}

sf::CircleShape makeCircle(float radius, sf::Vector2f position, sf::Color fillColor)
{
    sf::CircleShape shape(radius);
    shape.setOrigin({radius, radius});   // 让圆心与 position 对齐
    shape.setPosition(position);
    shape.setFillColor(fillColor);
    return shape;
}

sf::RectangleShape makeRectangle(sf::Vector2f size, sf::Vector2f position, sf::Color fillColor)
{
    sf::RectangleShape shape(size);
    shape.setOrigin({size.x / 2.f, size.y / 2.f});   // 让矩形中心与 position 对齐
    shape.setPosition(position);
    shape.setFillColor(fillColor);
    return shape;
}

// 学习点: 正多边形顶点均匀分布在圆周上(以原点为圆心), 之后整体平移到 position
sf::ConvexShape makeRegularPolygon(std::size_t sides, float radius, sf::Vector2f position, sf::Color fillColor)
{
    sf::ConvexShape shape(sides);
    for (std::size_t i = 0; i < sides; ++i) {
        const float angle = static_cast<float>(i) * 2.f * kPi / static_cast<float>(sides);
        shape.setPoint(i, {std::cos(angle) * radius, std::sin(angle) * radius});
    }
    shape.setPosition(position);
    shape.setFillColor(fillColor);
    return shape;
}
