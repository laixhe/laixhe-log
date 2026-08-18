#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Font.hpp>
#include <SFML/Graphics/Text.hpp>
#include <SFML/System/Vector2.hpp>

#include <string>

// ---------------------------------------------------------------------------
// 学习点: 文本与字体
//   sf::Font  字体(3.1 移除了内置默认字体, 需从文件加载)
//   sf::Text  文本(字体 + 字符串 + 字号 + 颜色)
// 注意: sf::String 从 std::string 构造时按系统本地代码页解码,
// 中文必须用 sf::String::fromUtf8() 显式指定 UTF-8, 否则会乱码
// ---------------------------------------------------------------------------

class TextRenderer {
public:
    // 加载字体文件, 返回是否成功(失败时 draw 无输出)
    bool loadFont(const std::string& path);

    // 绘制 UTF-8 文本
    void draw(sf::RenderTarget& target, const std::string& utf8Text, sf::Vector2f position,
              unsigned int characterSize, sf::Color color) const;

private:
    sf::Font m_font;
    bool m_loaded = false;
};
