#include "particles.hpp"

#include <SFML/System/Angle.hpp>

#include <cmath>
#include <cstdint>
#include <random>

namespace {
constexpr float kPi = 3.14159265f;

std::mt19937& rng()
{
    static std::mt19937 gen{std::random_device{}()};
    return gen;
}
}  // namespace

ParticleSystem::ParticleSystem(std::size_t maxCount) : m_maxCount(maxCount), m_particles(maxCount)
{
    // 学习点: SFML 3 移除了 Quads 图元, 每个粒子用 2 个三角形(6 顶点)组成小方块
    m_vertices.setPrimitiveType(sf::PrimitiveType::Triangles);
    m_vertices.resize(maxCount * 6);
}

void ParticleSystem::update(float dt, const sf::Vector2f& origin, const sf::Color& color, float spawnRate)
{
    m_spawnAccum += spawnRate * dt;
    while (m_spawnAccum >= 1.f) {
        m_spawnAccum -= 1.f;
        spawnParticle(origin, color);
    }

    // 物理模拟: 重力 + 空气阻力(阻尼)
    constexpr float gravity = 300.f;
    constexpr float drag = 0.98f;
    for (auto& p : m_particles) {
        if (p.life <= 0.f) {
            continue;
        }
        p.life -= dt;
        p.velocity.y += gravity * dt;
        p.velocity *= drag;
        p.position += p.velocity * dt;
    }
    rebuildVertices(color);
}

// 喷泉效果: 从原点向上随机方向发射, 受重力回落
void ParticleSystem::spawnParticle(const sf::Vector2f& origin, const sf::Color& /*color*/)
{
    Particle& p = m_particles[m_next];
    m_next = (m_next + 1) % m_maxCount;

    std::uniform_real_distribution<float> angleDist(0.f, 2.f * kPi);
    std::uniform_real_distribution<float> speedDist(100.f, 320.f);
    const float angle = angleDist(rng());
    const float speed = speedDist(rng());

    p.position = origin;
    p.velocity = {std::cos(angle) * speed, -std::abs(std::sin(angle)) * speed};
    p.maxLife = 1.2f;
    p.life = p.maxLife;
}

// 学习点: 顶点批量绘制 - 根据粒子寿命重新设置每个顶点的位置与颜色(淡出)
void ParticleSystem::rebuildVertices(const sf::Color& color)
{
    constexpr float size = 4.f;   // 粒子边长(像素)
    std::size_t i = 0;
    for (const auto& p : m_particles) {
        sf::Vertex* tri = &m_vertices[i * 6];
        if (p.life <= 0.f) {
            for (int k = 0; k < 6; ++k) {
                tri[k].color = sf::Color::Transparent;
            }
        } else {
            const float t = p.life / p.maxLife;
            const auto alpha = static_cast<std::uint8_t>(255.f * t);
            const sf::Color c(color.r, color.g, color.b, alpha);
            const sf::Vector2f& pos = p.position;
            // 两个三角形拼成小方块(逆时针)
            tri[0] = {pos, c};
            tri[1] = {{pos.x + size, pos.y}, c};
            tri[2] = {{pos.x + size, pos.y + size}, c};
            tri[3] = {pos, c};
            tri[4] = {{pos.x + size, pos.y + size}, c};
            tri[5] = {{pos.x, pos.y + size}, c};
        }
        ++i;
    }
}

void ParticleSystem::draw(sf::RenderTarget& target)
{
    target.draw(m_vertices);
}
