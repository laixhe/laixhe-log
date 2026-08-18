#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/Graphics/VertexArray.hpp>
#include <SFML/System/Vector2.hpp>

#include <cstddef>
#include <vector>

// ---------------------------------------------------------------------------
// 学习点: 粒子系统与爆炸效果
//   粒子(Particle): 一个"小点", 拥有 位置/速度/寿命/大小 等属性
//   系统更新(每帧): 欧拉积分更新位置与速度, 计时, 剔除已死亡的粒子
//   爆炸 = 在一点批量发射粒子, 每个粒子随机方向 + 随机初速度,
//          受重力(向下)与空气阻力(衰减)影响, 颜色随寿命淡出
//   性能: 所有粒子放进同一个 VertexArray(每个粒子 2 个三角形拼成小方块),
//          一次 draw 提交 GPU, 比逐个 draw sf::CircleShape 高效得多
// ---------------------------------------------------------------------------

struct Particle {
    sf::Vector2f position;    // 当前位置(世界坐标)
    sf::Vector2f velocity;    // 当前速度(像素/秒)
    float lifetime = 0.f;     // 已存活时间(秒)
    float maxLifetime = 0.f;  // 总寿命(秒), 到时死亡
    float size = 0.f;         // 粒子半径(像素)
    sf::Color color;          // 粒子颜色
};

class ParticleSystem {
public:
    // 在 position 处引爆: 发射 count 个全方向粒子,
    // speed 平均初速度(实际 40%~100% 随机), maxLifetime 平均寿命, color 基础色
    void explode(sf::Vector2f position, std::size_t count, float speed,
                 float maxLifetime, sf::Color color);

    // 每帧调用: 更新所有粒子(重力/阻力/移动/计时), 重建顶点数组
    void update(float dt);

    // 提交整个顶点数组到 GPU
    void draw(sf::RenderTarget& target) const;

    // 是否还有存活的粒子
    bool alive() const { return !m_particles.empty(); }

private:
    static constexpr std::size_t kMaxParticles = 500;   // 粒子池上限, 防止无限增长

    void rebuildVertices();

    std::vector<Particle> m_particles;
    sf::VertexArray m_vertices{sf::PrimitiveType::Triangles};
};
