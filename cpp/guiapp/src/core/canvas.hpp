#pragma once

#include "particles.hpp"

#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/RenderTexture.hpp>
#include <SFML/System/Vector2.hpp>

struct ShapeProps;

// 学习点: 封装 RenderTexture 离屏渲染, 负责绘制网格/形状/粒子/文本
class Canvas {
public:
    explicit Canvas(sf::Vector2u size);

    void draw(const ShapeProps& props, float rotation, float dtSeconds);   // 清屏 + 绘制全部内容
    const sf::Texture& getTexture() const;
    // 学习点: RenderTexture 的纹理在 OpenGL 中是垂直翻转存储的,
    // 直接传 getTexture() 给 ImGui::SFML::Image 会显示颠倒;
    // 应传 RenderTexture 本身, 走 imgui-sfml 的 RenderTexture 重载(自动翻转 UV)
    const sf::RenderTexture& getRenderTexture() const;
    sf::Vector2u getSize() const;

private:
    void drawGrid();
    void drawShape(const ShapeProps& props, float rotation);
    void drawParticles(const ShapeProps& props, float dtSeconds);
    void drawText(const ShapeProps& props);

    sf::RenderTexture m_texture;
    sf::Font m_font;
    bool m_hasFont = false;
    ParticleSystem m_particles{200};
};
