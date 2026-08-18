#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/VertexArray.hpp>
#include <SFML/System/Vector2.hpp>

#include <vector>

// ---------------------------------------------------------------------------
// 学习点: sf::VertexArray 顶点数组
//   Shape 是现成的图形; VertexArray 则直接指定顶点 + 图元类型,
//   可以自由绘制网格、星形、曲线、粒子等, 是更底层更灵活的方式
// 图元类型:
//   Points         散点
//   Lines          线段(每 2 个顶点 1 条)
//   LineStrip      折线(顶点依次相连)
//   Triangles      三角形(每 3 个顶点 1 个)
//   TriangleStrip  三角形带(相邻共享边, 省内存)
//   TriangleFan    三角形扇(第 1 个顶点为公共点)
// 注意: SFML 3 移除了 sf::PrimitiveType::Quads, 四边形改用 2 个三角形拼
// ---------------------------------------------------------------------------

// 网格线: 在 (0,0)-(size) 范围内画 cell 间距的横竖线(Lines 图元)
sf::VertexArray makeGrid(sf::Vector2f size, float cell, sf::Color color);

// 空心星形(如五角星): 中心 center, 外接半径 outerRadius, 内接半径 innerRadius,
// points 个角(TriangleFan 图元)
sf::VertexArray makeStar(sf::Vector2f center, float outerRadius, float innerRadius,
                         unsigned points, sf::Color color);

// 闪烁星星场: 每颗星由 2 个三角形拼成菱形, 把所有顶点放进同一个
// VertexArray 一次性提交 GPU, 每帧用正弦波调制颜色 alpha 实现"闪烁"
class TwinkleStarField {
public:
    // count 星星数量, areaSize 分布区域(世界坐标)
    explicit TwinkleStarField(unsigned count, sf::Vector2f areaSize);

    void update(float dt);                       // 累加时间, 更新每帧的 alpha
    void draw(sf::RenderTarget& target) const;   // 提交整个顶点数组

private:
    struct Star {
        sf::Vector2f pos;
        float size = 0.f;    // 菱形半边长(像素)
        float phase = 0.f;   // 闪烁相位(0~2π), 让星星不同步
        float speed = 0.f;   // 闪烁速度
    };

    std::vector<Star> m_stars;
    sf::VertexArray m_vertices;   // 每颗星 6 个顶点(2 个三角形拼菱形)
    float m_time = 0.f;
};
