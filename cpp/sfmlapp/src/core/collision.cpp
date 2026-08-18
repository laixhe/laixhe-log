#include "collision.hpp"

#include <algorithm>
#include <cmath>

bool circlesCollide(sf::Vector2f a, float ra, sf::Vector2f b, float rb)
{
    const float dx = a.x - b.x;
    const float dy = a.y - b.y;
    const float radiusSum = ra + rb;
    return dx * dx + dy * dy <= radiusSum * radiusSum;
}

bool circleRectCollide(sf::Vector2f circleCenter, float circleRadius,
                       sf::Vector2f rectCenter, sf::Vector2f rectHalfSize)
{
    // 圆心在矩形轴对齐下的最近点
    const float closestX = std::clamp(circleCenter.x, rectCenter.x - rectHalfSize.x, rectCenter.x + rectHalfSize.x);
    const float closestY = std::clamp(circleCenter.y, rectCenter.y - rectHalfSize.y, rectCenter.y + rectHalfSize.y);
    const float dx = circleCenter.x - closestX;
    const float dy = circleCenter.y - closestY;
    return dx * dx + dy * dy <= circleRadius * circleRadius;
}
