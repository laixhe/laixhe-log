#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/VertexArray.hpp>
#include <SFML/System/Vector2.hpp>

#include <cstddef>
#include <vector>

// 学习点: 用 sf::VertexArray(Triangles) 批量绘制粒子, 演示动画、重力与顶点颜色渐变
class ParticleSystem {
public:
    explicit ParticleSystem(std::size_t maxCount);

    void update(float dt, const sf::Vector2f& origin, const sf::Color& color, float spawnRate);
    void draw(sf::RenderTarget& target);

private:
    struct Particle {
        sf::Vector2f position;
        sf::Vector2f velocity;
        float life = 0.f;
        float maxLife = 1.f;
    };

    void spawnParticle(const sf::Vector2f& origin, const sf::Color& color);
    void rebuildVertices(const sf::Color& color);

    std::vector<Particle> m_particles;
    sf::VertexArray m_vertices;   // sf::Triangles, 每个粒子 6 个顶点(2 个三角形)
    std::size_t m_maxCount;
    std::size_t m_next = 0;
    float m_spawnAccum = 0.f;
};
