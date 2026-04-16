#include <iostream>

#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>
#include <SFML/Graphics.hpp>

int main()
{
    spdlog::info("Hello from spdlog!");

    nlohmann::json j;
    j["name"] = "John";
    j["age"] = 30;

    std::cout << j.dump(4) << std::endl;

    // VideoMode 表示的是窗口的大小
    // title 窗口的标题
    // style 窗口的样式 [Titlebar 标题栏][Resize 调整大小，并有一个最大化按钮][Close 关闭按钮]
    // state 定义是普通窗口还是全屏的
    sf::RenderWindow window(sf::VideoMode(sf::Vector2u{600, 800}), "SFML Window");
    // 窗口的标题
    // window.setTitle("测试");
    // 窗口的大小
    // window.setSize(sf::Vector2u{300, 300});
    // 移动窗口位置
    // window.setPosition(sf::Vector2i{100, 100});

    // 获取窗口大小
    // const auto size = window.getSize();
    // std::cout << size.x << ' ' << size.y << std::endl;

    // 帧率限制
    // window.setFramerateLimit(60);
    // 启用垂直同步（同步显示器的刷新率）
    window.setVerticalSyncEnabled(true);

    // 圆形
    sf::CircleShape shape(100.f);
    shape.setFillColor(sf::Color::Green);

    // 主循环
    while (window.isOpen())
    {
        // 检查自上次循环迭代以来触发的所有窗口事件
        while (const std::optional event = window.pollEvent())
        {
            // 如果检测到 关闭窗口 事件，则关闭窗口
            if (event->is<sf::Event::Closed>())
            {
                window.close();
            }
        }
        // 清屏，默认会使用黑色填充 sf::Color::Black
        window.clear();
        // 绘制
        window.draw(shape);
        // 更新窗口
        window.display();
    }
}

/*
 * 事件系统
 *
// 类型安全的事件检查
if (const auto* keyEvent = event->getIf<sf::Event::KeyPressed>()) {
//处理按键事件
}
//基于回调的事件处理
window.handleEvents(
	[](const sf::Event::Closed&) { // 关闭事件 },
	[](const sf::Event::KeyPressed&) { // 按键事件 }
);

sf::Event::Closed              关闭事件
sf::Event::Resized             窗口调整大小事件
sf::Event::KeyPressed          键盘按下
sf::Event::KeyReleased         键盘松开
sf::Event::MouseButtonPressed  鼠标按下
sf::Event::MouseButtonReleased 鼠标松开

sf::Keyboard::Scan::Escape
sf::Mouse::Button::Right
 */

/*
 * 角度系统
 *
// 设置旋转角度
shape.setRotation(sf::degrees(90));
// 获取角度值
float degrees = shape.getRotation().asDegrees();
 */

/*
 * 资源管理
 *
// 直接构造并加载
sf::Texture texture("image.png");

// 传统方式仍然可用
sf::Texture texture;
if (!texture.loadFromFile("image.png")) {
	// 错误处理
}

// 加载纹理
const sf::Texture texture("image.jpg");
// 创建精灵
sf::Sprite sprite(texture);

// 加载字体
const sf::Font font("arial.ttf");
// 创建文本
sf::Text text(font, "Hello SFML", 50);

// 加载音乐
sf::Music music("nice_music.ogg");
// 播放音乐
music.play();
 */

/*
 *
sf::CircleShape    圆形
sf::RectangleShape 矩形
sf::CircleShape    正多边形
 */
