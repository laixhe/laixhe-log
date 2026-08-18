#include "text.hpp"

#include <SFML/Graphics/RenderTarget.hpp>
#include <SFML/System/String.hpp>

bool TextRenderer::loadFont(const std::string& path)
{
    m_loaded = m_font.openFromFile(path);
    return m_loaded;
}

void TextRenderer::draw(sf::RenderTarget& target, const std::string& utf8Text, sf::Vector2f position,
                        unsigned int characterSize, sf::Color color) const
{
    if (!m_loaded) {
        return;
    }
    sf::Text text(m_font, sf::String::fromUtf8(utf8Text.begin(), utf8Text.end()), characterSize);
    text.setFillColor(color);
    text.setPosition(position);
    target.draw(text);
}
