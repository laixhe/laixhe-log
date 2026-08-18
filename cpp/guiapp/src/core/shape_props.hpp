#pragma once

// 学习点: 由 ImGui 控件驱动的 SFML 图形属性(纯数据结构, 不依赖任何库)
struct ShapeProps {
    float radius = 60.f;
    float x = 450.f, y = 300.f;
    float rotationSpeed = 30.f;   // 单位: 度/秒
    int shapeType = 0;            // 0=圆形 1=矩形 2=三角形
    bool filled = true;
    float color[4] = {0.2f, 0.9f, 0.3f, 1.f};   // ImGui 的 RGBA(0~1)
    bool showGrid = true;
    bool showText = true;
    bool showParticles = true;
    char text[32] = "SFML 文本";
};
