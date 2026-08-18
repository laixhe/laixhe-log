#pragma once

#include <SFML/System/Vector2.hpp>

struct ShapeProps;

// 学习点: 常用 ImGui 控件速查, 控件直接读写 props 实时驱动画布中的 SFML 图形
void showControlPanel(ShapeProps& props, float rotation, sf::Vector2u canvasSize);
