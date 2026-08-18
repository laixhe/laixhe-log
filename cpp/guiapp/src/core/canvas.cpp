#include "canvas.hpp"
#include "shape_props.hpp"

#include <spdlog/spdlog.h>

#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/System/Angle.hpp>
#include <SFML/System/String.hpp>

#include <cstdint>
#include <string>

namespace {
// 学习点: 把 ImGui 的 RGBA(0~1) 颜色转换为 SFML 颜色
sf::Color toFillColor(const ShapeProps& p)
{
    return sf::Color(static_cast<std::uint8_t>(p.color[0] * 255.f),
                     static_cast<std::uint8_t>(p.color[1] * 255.f),
                     static_cast<std::uint8_t>(p.color[2] * 255.f),
                     static_cast<std::uint8_t>(p.color[3] * 255.f));
}
}  // namespace

Canvas::Canvas(sf::Vector2u size) : m_texture(size)
{
    // 学习点: SFML 3.1 移除了内置默认字体, 这里加载系统字体(黑体 simhei.ttf 支持中文)
    // (注意: 不要用 msyh.ttc 这类 TTC 集合文件, 优先使用单文件 TTF)
    m_hasFont = m_font.openFromFile("C:/Windows/Fonts/simhei.ttf");
    if (!m_hasFont) {
        m_hasFont = m_font.openFromFile("C:/Windows/Fonts/arial.ttf");
    }
    if (!m_hasFont) {
        spdlog::warn("无法加载系统字体(simhei.ttf / arial.ttf), 画布文本将被禁用");
    }
}

void Canvas::draw(const ShapeProps& props, float rotation, float dtSeconds)
{
    m_texture.clear(sf::Color(30, 30, 30));
    if (props.showGrid) {
        drawGrid();
    }
    drawShape(props, rotation);
    if (props.showParticles) {
        drawParticles(props, dtSeconds);
    }
    if (props.showText && m_hasFont) {
        drawText(props);
    }
    m_texture.display();
}

// 学习点: SFML 基础绘图 - 网格背景
void Canvas::drawGrid()
{
    const sf::Vector2u size = m_texture.getSize();
    constexpr int divisions = 10;
    const sf::Color lineColor(80, 80, 80, 120);

    for (int i = 0; i <= divisions; ++i) {
        sf::RectangleShape h({static_cast<float>(size.x), 1.f});
        h.setFillColor(lineColor);
        h.setPosition({0.f, static_cast<float>(i) * size.y / divisions});
        m_texture.draw(h);
    }
    for (int i = 0; i <= divisions; ++i) {
        sf::RectangleShape v({1.f, static_cast<float>(size.y)});
        v.setFillColor(lineColor);
        v.setPosition({static_cast<float>(i) * size.x / divisions, 0.f});
        m_texture.draw(v);
    }
}

// 学习点: SFML 基础绘图 - 三种形状(圆形/矩形/三角形), 属性全部来自 ImGui 控件
void Canvas::drawShape(const ShapeProps& p, float rotation)
{
    const sf::Color fill = toFillColor(p);
    const sf::Vector2f center{p.x, p.y};
    const float r = p.radius;

    switch (p.shapeType) {
    case 0: {
        sf::CircleShape shape(r);
        shape.setOrigin({r, r});
        shape.setPosition(center);
        shape.setRotation(sf::degrees(rotation));
        shape.setFillColor(p.filled ? fill : sf::Color::Transparent);
        shape.setOutlineThickness(p.filled ? 0.f : 3.f);
        shape.setOutlineColor(fill);
        m_texture.draw(shape);
        break;
    }
    case 1: {
        sf::RectangleShape shape({2.f * r, r});
        shape.setOrigin({r, r / 2.f});
        shape.setPosition(center);
        shape.setRotation(sf::degrees(rotation));
        shape.setFillColor(p.filled ? fill : sf::Color::Transparent);
        shape.setOutlineThickness(p.filled ? 0.f : 3.f);
        shape.setOutlineColor(fill);
        m_texture.draw(shape);
        break;
    }
    case 2: {
        sf::ConvexShape shape(3);
        shape.setPoint(0, {0.f, -r});
        shape.setPoint(1, {r, r});
        shape.setPoint(2, {-r, r});
        shape.setPosition(center);
        shape.setRotation(sf::degrees(rotation));
        shape.setFillColor(p.filled ? fill : sf::Color::Transparent);
        shape.setOutlineThickness(p.filled ? 0.f : 3.f);
        shape.setOutlineColor(fill);
        m_texture.draw(shape);
        break;
    }
    }
}

// 学习点: 喷泉粒子动画 - 从形状中心发射, 受重力回落并淡出
void Canvas::drawParticles(const ShapeProps& p, float dtSeconds)
{
    m_particles.update(dtSeconds, {p.x, p.y}, toFillColor(p), 120.f);
    m_particles.draw(m_texture);
}

void Canvas::drawText(const ShapeProps& p)
{
    const sf::Vector2u size = m_texture.getSize();

    // 学习点: sf::String 从 std::string 构造时按系统本地代码页(GBK)解码,
    // 必须用 fromUtf8() 显式指定 UTF-8, 否则中文会乱码
    const std::string textStr(p.text);
    sf::Text text(m_font, sf::String::fromUtf8(textStr.begin(), textStr.end()), 24);
    text.setFillColor(sf::Color::White);
    text.setPosition({20.f, static_cast<float>(size.y) - 60.f});
    m_texture.draw(text);

    const std::string hintStr = "按住鼠标左键拖拽形状";
    sf::Text hint(m_font, sf::String::fromUtf8(hintStr.begin(), hintStr.end()), 16);
    hint.setFillColor(sf::Color(200, 200, 200));
    hint.setPosition({20.f, 20.f});
    m_texture.draw(hint);
}

const sf::Texture& Canvas::getTexture() const
{
    return m_texture.getTexture();
}

const sf::RenderTexture& Canvas::getRenderTexture() const
{
    return m_texture;
}

sf::Vector2u Canvas::getSize() const
{
    return m_texture.getSize();
}
