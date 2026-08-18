#include "sprite.hpp"

#include <SFML/Graphics/Image.hpp>

sf::Texture makeCheckerTexture(sf::Vector2u size, unsigned cellPixels, sf::Color a, sf::Color b)
{
    sf::Image image(size, sf::Color::Black);
    for (unsigned y = 0; y < size.y; ++y) {
        for (unsigned x = 0; x < size.x; ++x) {
            const bool even = ((x / cellPixels) + (y / cellPixels)) % 2 == 0;
            image.setPixel({x, y}, even ? a : b);
        }
    }

    sf::Texture texture(image);
    texture.setSmooth(true);   // 缩放时插值平滑
    return texture;
}

sf::Sprite makeSprite(const sf::Texture& texture, sf::Vector2f position, float scale)
{
    sf::Sprite sprite(texture);
    sprite.setOrigin({static_cast<float>(texture.getSize().x) / 2.f,
                      static_cast<float>(texture.getSize().y) / 2.f});
    sprite.setPosition(position);
    sprite.setScale({scale, scale});
    return sprite;
}
