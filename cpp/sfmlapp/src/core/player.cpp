#include "player.hpp"

#include <algorithm>
#include <cmath>

void Player::handleInput(float dt, sf::Vector2f bounds)
{
    sf::Vector2f move{0.f, 0.f};
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Key::W) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::Up)) {
        move.y -= 1.f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Key::S) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::Down)) {
        move.y += 1.f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Key::A) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::Left)) {
        move.x -= 1.f;
    }
    if (sf::Keyboard::isKeyPressed(sf::Keyboard::Key::D) || sf::Keyboard::isKeyPressed(sf::Keyboard::Key::Right)) {
        move.x += 1.f;
    }

    // 归一化, 避免斜向移动比直线移动更快
    const float length = std::hypot(move.x, move.y);
    if (length > 0.f) {
        move /= length;
    }
    position += move * speed * dt;

    // 限制在窗口内(考虑半径, 不超出边界)
    position.x = std::clamp(position.x, radius, bounds.x - radius);
    position.y = std::clamp(position.y, radius, bounds.y - radius);
}

void Player::draw(sf::RenderTarget& target) const
{
    sf::CircleShape shape(radius);
    shape.setOrigin({radius, radius});
    shape.setPosition(position);
    shape.setFillColor(collided ? sf::Color::Red : normalColor);
    target.draw(shape);
}
