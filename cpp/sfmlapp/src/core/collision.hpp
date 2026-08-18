#pragma once

#include <SFML/System/Vector2.hpp>

// ---------------------------------------------------------------------------
// 学习点: 碰撞检测
//   圆-圆: 两圆心距离 <= 半径之和
//   圆-矩形(AABB): 圆心到矩形最近点的距离 <= 半径
// ---------------------------------------------------------------------------

// 圆与圆碰撞: 圆心 a(半径 ra) 与圆心 b(半径 rb)
bool circlesCollide(sf::Vector2f a, float ra, sf::Vector2f b, float rb);

// 圆与矩形(轴对齐 AABB)碰撞: 矩形用"中心 + 半宽高"表示(便于旋转无关)
bool circleRectCollide(sf::Vector2f circleCenter, float circleRadius,
                       sf::Vector2f rectCenter, sf::Vector2f rectHalfSize);
