#include "vertex_array.hpp"

#include <cmath>
#include <cstdint>
#include <random>

sf::VertexArray makeGrid(sf::Vector2f size, float cell, sf::Color color)
{
    sf::VertexArray grid(sf::PrimitiveType::Lines);
    for (float x = 0.f; x <= size.x; x += cell) {
        grid.append(sf::Vertex({x, 0.f}, color));
        grid.append(sf::Vertex({x, size.y}, color));
    }
    for (float y = 0.f; y <= size.y; y += cell) {
        grid.append(sf::Vertex({0.f, y}, color));
        grid.append(sf::Vertex({size.x, y}, color));
    }
    return grid;
}

sf::VertexArray makeStar(sf::Vector2f center, float outerRadius, float innerRadius,
                         unsigned points, sf::Color color)
{
    // TriangleFan: 第 1 个顶点是公共点(中心), 之后每 2 个顶点与中心构成一个三角形
    sf::VertexArray star(sf::PrimitiveType::TriangleFan);
    star.append(sf::Vertex(center, color));   // 公共点

    constexpr float kPi = 3.14159265358979f;
    const unsigned total = points * 2;        // 尖角 + 凹角 交替
    for (unsigned i = 0; i <= total; ++i) {
        const float angle = static_cast<float>(i) * kPi / static_cast<float>(points);   // 0~2π
        const bool outer = (i % 2 == 0);
        const float radius = outer ? outerRadius : innerRadius;
        const sf::Vector2f p = center + sf::Vector2f(std::cos(angle), std::sin(angle)) * radius;
        star.append(sf::Vertex(p, color));
    }
    return star;
}

TwinkleStarField::TwinkleStarField(unsigned count, sf::Vector2f areaSize)
    : m_vertices(sf::PrimitiveType::Triangles)
{
    std::mt19937 rng(20260817);   // 固定种子, 每次运行星星分布一致
    std::uniform_real_distribution<float> posX(0.f, areaSize.x);
    std::uniform_real_distribution<float> posY(0.f, areaSize.y);
    std::uniform_real_distribution<float> sizeDist(3.f, 8.f);
    std::uniform_real_distribution<float> phaseDist(0.f, 6.2831853f);
    std::uniform_real_distribution<float> speedDist(1.f, 3.f);

    for (unsigned i = 0; i < count; ++i) {
        Star s;
        s.pos = {posX(rng), posY(rng)};
        s.size = sizeDist(rng);
        s.phase = phaseDist(rng);
        s.speed = speedDist(rng);
        m_stars.push_back(s);

        // 每颗星占 6 个顶点: 两个三角形拼成菱形 (0,1,2) + (0,2,3)
        for (unsigned v = 0; v < 6; ++v) {
            m_vertices.append(sf::Vertex(s.pos, sf::Color::White));
        }
    }
}

void TwinkleStarField::update(float dt)
{
    m_time += dt;
    constexpr float kPi = 3.14159265358979f;

    std::size_t vi = 0;
    for (const Star& s : m_stars) {
        // alpha 在 60~255 之间按正弦波动, 实现"亮-暗-亮"闪烁
        const float wave = 0.5f + 0.5f * std::sin(m_time * s.speed + s.phase);
        const std::uint8_t alpha = static_cast<std::uint8_t>(60 + 195.f * wave);
        const sf::Color c(sf::Color::White.r, sf::Color::White.g, sf::Color::White.b, alpha);

        // 菱形 4 个顶点: 上/右/下/左 (顺序: 0上 1右 2下 3左)
        const sf::Vector2f up{s.pos.x, s.pos.y - s.size};
        const sf::Vector2f right{s.pos.x + s.size, s.pos.y};
        const sf::Vector2f down{s.pos.x, s.pos.y + s.size};
        const sf::Vector2f left{s.pos.x - s.size, s.pos.y};
        const sf::Vector2f verts[4] = {up, right, down, left};

        m_vertices[vi + 0].position = verts[0];   m_vertices[vi + 0].color = c;
        m_vertices[vi + 1].position = verts[1];   m_vertices[vi + 1].color = c;
        m_vertices[vi + 2].position = verts[2];   m_vertices[vi + 2].color = c;
        m_vertices[vi + 3].position = verts[0];   m_vertices[vi + 3].color = c;
        m_vertices[vi + 4].position = verts[2];   m_vertices[vi + 4].color = c;
        m_vertices[vi + 5].position = verts[3];   m_vertices[vi + 5].color = c;
        vi += 6;
    }
}

void TwinkleStarField::draw(sf::RenderTarget& target) const
{
    target.draw(m_vertices);
}
