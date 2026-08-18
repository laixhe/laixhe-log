#include "mouse.hpp"

#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Window/Mouse.hpp>

#include <algorithm>
#include <cmath>

void MouseDemo::handleEvent(const sf::Event& event, const sf::RenderWindow& window, const sf::View& view)
{
    if (const auto* press = event.getIf<sf::Event::MouseButtonPressed>()) {
        if (press->button == sf::Mouse::Button::Left) {
            const sf::Vector2f world = window.mapPixelToCoords(press->position, view);
            dragging = true;
            dragStart = world;
            // 记录点击标记(限制数量, 防止无限增长)
            clickMarks.push_back(world);
            if (clickMarks.size() > 30) {
                clickMarks.erase(clickMarks.begin());
            }
        }
    } else if (const auto* release = event.getIf<sf::Event::MouseButtonReleased>()) {
        if (release->button == sf::Mouse::Button::Left) {
            const sf::Vector2f world = window.mapPixelToCoords(release->position, view);
            dragging = false;
            // 拖拽距离足够大才算选区, 否则视为普通点击
            if (std::abs(world.x - dragStart.x) > 4.f || std::abs(world.y - dragStart.y) > 4.f) {
                const sf::Vector2f leftTop(std::min(dragStart.x, world.x), std::min(dragStart.y, world.y));
                const sf::Vector2f size(std::abs(world.x - dragStart.x), std::abs(world.y - dragStart.y));
                selections.emplace_back(leftTop, size);
                if (selections.size() > 20) {
                    selections.erase(selections.begin());
                }
            }
        }
    }
}

void MouseDemo::updateHover(const sf::RenderWindow& window, const sf::View& view)
{
    // sf::Mouse::getPosition 返回窗口内像素坐标(屏幕坐标)
    const sf::Vector2i screen = sf::Mouse::getPosition(window);
    hover = window.mapPixelToCoords(screen, view);
}

void MouseDemo::draw(sf::RenderTarget& target) const
{
    // 光标悬停圆环(描边圆圈, 填充透明)
    sf::CircleShape hoverRing(12.f, 32);
    hoverRing.setFillColor(sf::Color::Transparent);
    hoverRing.setOutlineThickness(2.f);
    hoverRing.setOutlineColor(sf::Color(255, 255, 255, 180));
    hoverRing.setOrigin({12.f, 12.f});
    hoverRing.setPosition(hover);
    target.draw(hoverRing);

    // 点击标记点(红色小圆)
    for (const sf::Vector2f& p : clickMarks) {
        sf::CircleShape dot(3.f);
        dot.setFillColor(sf::Color(255, 100, 100));
        dot.setOrigin({3.f, 3.f});
        dot.setPosition(p);
        target.draw(dot);
    }

    // 正在拖拽的选区(半透明青色)
    if (dragging) {
        const sf::Vector2f leftTop(std::min(dragStart.x, hover.x), std::min(dragStart.y, hover.y));
        const sf::Vector2f size(std::abs(hover.x - dragStart.x), std::abs(hover.y - dragStart.y));
        sf::RectangleShape rect(size);
        rect.setPosition(leftTop);
        rect.setFillColor(sf::Color(0, 200, 255, 40));
        rect.setOutlineThickness(1.f);
        rect.setOutlineColor(sf::Color(0, 200, 255));
        target.draw(rect);
    }

    // 已完成的选区(半透明绿色描边)
    for (const sf::FloatRect& s : selections) {
        sf::RectangleShape rect(s.size);
        rect.setPosition(s.position);
        rect.setFillColor(sf::Color(0, 255, 120, 20));
        rect.setOutlineThickness(1.5f);
        rect.setOutlineColor(sf::Color(0, 255, 120));
        target.draw(rect);
    }
}
