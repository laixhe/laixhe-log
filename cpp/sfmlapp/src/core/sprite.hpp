#pragma once

#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <SFML/System/Vector2.hpp>

// ---------------------------------------------------------------------------
// 学习点: 纹理与精灵
//   sf::Texture   GPU 纹理(可从 Image/文件/内存加载, 支持 setSmooth 平滑缩放)
//   sf::Sprite    引用纹理的显示对象, 支持位置/旋转/缩放/着色(setColor)
// 这里演示"程序化生成纹理" - 用 sf::Image 逐像素绘制棋盘格, 无需外部图片文件
// ---------------------------------------------------------------------------

// 生成棋盘格纹理: size 纹理尺寸, cellPixels 每格边长(像素), a/b 两种格子颜色
sf::Texture makeCheckerTexture(sf::Vector2u size, unsigned cellPixels, sf::Color a, sf::Color b);

// 创建精灵: 使用 texture, 中心对齐到 position, 缩放 scale 倍
sf::Sprite makeSprite(const sf::Texture& texture, sf::Vector2f position, float scale = 1.f);
