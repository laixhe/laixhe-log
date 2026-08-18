#pragma once

#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/System/Vector2.hpp>

#include <cstddef>

// ---------------------------------------------------------------------------
// 学习点: SFML 基础形状
//   sf::CircleShape     圆形 / 正多边形(通过 setPointCount 调整边数)
//   sf::RectangleShape  矩形(也支持圆角半径 setCornerPointCount)
//   sf::ConvexShape     任意凸多边形(用 setPoint 指定顶点)
// 形状默认以左上角为原点, setOrigin 让中心与 position 对齐, 旋转/缩放都围绕中心
// ---------------------------------------------------------------------------

// 创建圆形: 半径为 radius, 中心在 position, 填充 fillColor
sf::CircleShape makeCircle(float radius, sf::Vector2f position, sf::Color fillColor);

// 创建矩形: 尺寸为 size, 中心在 position, 填充 fillColor
sf::RectangleShape makeRectangle(sf::Vector2f size, sf::Vector2f position, sf::Color fillColor);

// 创建正多边形: sides 条边(3=三角形, 5=五边形), 外接圆半径 radius, 中心在 position
sf::ConvexShape makeRegularPolygon(std::size_t sides, float radius, sf::Vector2f position, sf::Color fillColor);
