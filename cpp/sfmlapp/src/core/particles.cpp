#include "particles.hpp"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <random>

namespace {

// 全局随机数引擎(静态局部变量): 每次爆炸生成的粒子方向/速度都不同
std::mt19937& rng()
{
    static std::mt19937 engine(std::random_device{}());
    return engine;
}

} // namespace

void ParticleSystem::explode(sf::Vector2f position, std::size_t count, float speed,
                             float maxLifetime, sf::Color color)
{
    constexpr float kTwoPi = 6.283185307f;
    std::uniform_real_distribution<float> angleDist(0.f, kTwoPi);   // 全方向 0~2π
    std::uniform_real_distribution<float> speedDist(0.4f, 1.f);     // 初速度 = speed 的 40%~100%
    std::uniform_real_distribution<float> lifeDist(0.6f, 1.f);      // 寿命 = maxLifetime 的 60%~100%
    std::uniform_real_distribution<float> sizeDist(2.f, 5.f);       // 粒子大小
    std::uniform_real_distribution<float> brightDist(0.8f, 1.2f);   // 颜色亮度扰动

    for (std::size_t i = 0; i < count && m_particles.size() < kMaxParticles; ++i) {
        const float angle = angleDist(rng());
        const float v = speed * speedDist(rng());

        Particle p;
        p.position = position;
        p.velocity = {std::cos(angle) * v, std::sin(angle) * v};   // 方向向量 * 初速度
        p.maxLifetime = maxLifetime * lifeDist(rng());
        p.lifetime = 0.f;
        p.size = sizeDist(rng());

        // 颜色亮度随机扰动(让爆炸更有层次), 并截断到 0~255
        const float b = brightDist(rng());
        p.color.r = static_cast<std::uint8_t>(std::clamp(color.r * b, 0.f, 255.f));
        p.color.g = static_cast<std::uint8_t>(std::clamp(color.g * b, 0.f, 255.f));
        p.color.b = static_cast<std::uint8_t>(std::clamp(color.b * b, 0.f, 255.f));

        m_particles.push_back(p);
    }
}

void ParticleSystem::update(float dt)
{
    constexpr float kGravity = 600.f;   // 像素/秒², 让粒子向下坠落
    constexpr float kDrag = 1.2f;       // 空气阻力系数(越大衰减越快)

    for (std::size_t i = 0; i < m_particles.size();) {
        Particle& p = m_particles[i];
        p.lifetime += dt;
        if (p.lifetime >= p.maxLifetime) {
            // 生命结束: 与末尾粒子交换后弹出, 避免 erase 造成整体搬移(重尾操作)
            m_particles[i] = m_particles.back();
            m_particles.pop_back();
            continue;
        }
        // 欧拉积分: 速度受重力/阻力影响, 位置按速度移动
        // 阻力用 exp(-kDrag * dt), 保证与帧率无关(不会因掉帧而衰减变慢)
        p.velocity += sf::Vector2f{0.f, kGravity} * dt;
        p.velocity *= std::exp(-kDrag * dt);
        p.position += p.velocity * dt;
        ++i;
    }

    rebuildVertices();
}

void ParticleSystem::rebuildVertices()
{
    m_vertices.clear();
    m_vertices.setPrimitiveType(sf::PrimitiveType::Triangles);

    for (const Particle& p : m_particles) {
        // 存活比例 progress = 1 刚爆炸 -> 0 即将死亡; alpha 随之线性淡出
        const float progress = p.lifetime / p.maxLifetime;
        sf::Color c = p.color;
        c.a = static_cast<std::uint8_t>((1.f - progress) * 255.f);

        // 每个粒子: 以 position 为中心的一个小方块, 2 个三角形 = 6 个顶点
        const sf::Vector2f half{p.size, p.size};
        const sf::Vector2f tl = p.position - half;
        const sf::Vector2f br = p.position + half;
        const sf::Vector2f tr{br.x, tl.y};
        const sf::Vector2f bl{tl.x, br.y};

        m_vertices.append(sf::Vertex(tl, c));
        m_vertices.append(sf::Vertex(br, c));
        m_vertices.append(sf::Vertex(tr, c));
        m_vertices.append(sf::Vertex(tl, c));
        m_vertices.append(sf::Vertex(bl, c));
        m_vertices.append(sf::Vertex(br, c));
    }
}

void ParticleSystem::draw(sf::RenderTarget& target) const
{
    if (!m_particles.empty()) {
        target.draw(m_vertices);
    }
}
