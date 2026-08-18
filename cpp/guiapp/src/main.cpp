#include <nlohmann/json.hpp>
#include <spdlog/spdlog.h>
#include <spdlog/cfg/env.h>

#include "imgui.h"
#include "imgui-SFML.h"

#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Clock.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Event.hpp>

#include "core/canvas.hpp"
#include "core/shape_props.hpp"
#include "ui/canvas_window.hpp"
#include "ui/control_panel.hpp"
#include "ui/log_window.hpp"
#include "ui/plots.hpp"

int main()
{
    // 默认输出级别为 info, 按键/鼠标等高频事件使用 debug 级别; 可用环境变量 SPDLOG_LEVEL=debug 开启
    spdlog::set_level(spdlog::level::info);
    spdlog::cfg::load_env_levels();

    // ---- 窗口创建 ----
    spdlog::info("正在创建窗口 ...");
    sf::RenderWindow window(sf::VideoMode({1280, 800}), "ImGui + SFML learning sample");
    window.setFramerateLimit(60);
    spdlog::info("窗口创建完成: 1280x800, 帧率上限 60fps");

    spdlog::info("正在初始化 ImGui-SFML ...");
    if (!ImGui::SFML::Init(window)) {
        spdlog::error("ImGui-SFML 初始化失败");
        return 1;
    }
    spdlog::info("ImGui-SFML 初始化完成");

    // 学习点: 加载系统中文字体(黑体 simhei.ttf), 使 ImGui 能显示中文
    // 只加载这一个字体: 它的字符范围同时包含 ASCII 与常用简体中文
    // 关键: 必须先 Clear() 清空 ImGui::SFML::Init 时已构建的字体 atlas,
    // 否则 GetTexDataAsRGBA32 看到旧数据不会重新光栅化, GPU 纹理仍是默认字体 -> 渲染乱码
    ImGuiIO& io = ImGui::GetIO();
    io.Fonts->Clear();
    if (io.Fonts->AddFontFromFileTTF("C:/Windows/Fonts/simhei.ttf", 16.0f, nullptr,
                                     io.Fonts->GetGlyphRangesChineseSimplifiedCommon())) {
        if (!ImGui::SFML::UpdateFontTexture()) {
            spdlog::warn("中文字体纹理上传失败");
        } else {
            spdlog::info("已加载中文字体: C:/Windows/Fonts/simhei.ttf");
        }
    } else {
        io.Fonts->AddFontDefault();
        if (!ImGui::SFML::UpdateFontTexture()) {
            spdlog::warn("默认字体纹理上传失败");
        }
        spdlog::warn("无法加载中文字体 C:/Windows/Fonts/simhei.ttf, 回退到默认字体, 中文将显示为方块");
    }

    initImGuiLogger();   // 把 spdlog 日志接入 ImGui 日志窗口

    Canvas canvas({900, 600});

    ShapeProps props;
    float rotation = 0.f;

    sf::Clock deltaClock;
    spdlog::info("进入主循环");
    while (window.isOpen()) {
        // ---- 事件处理 ----
        while (const auto event = window.pollEvent()) {
            ImGui::SFML::ProcessEvent(window, *event);

            if (event->is<sf::Event::Closed>()) {
                spdlog::info("收到窗口关闭事件, 准备退出");
                window.close();
            } else if (event->is<sf::Event::Resized>()) {
                const auto* resized = event->getIf<sf::Event::Resized>();
                spdlog::info("窗口尺寸变化: {} x {}", resized->size.x, resized->size.y);
            } else if (event->is<sf::Event::KeyPressed>()) {
                const auto* key = event->getIf<sf::Event::KeyPressed>();
                spdlog::debug("按下按键: code={}", static_cast<int>(key->code));
            } else if (event->is<sf::Event::MouseButtonPressed>()) {
                const auto* mouse = event->getIf<sf::Event::MouseButtonPressed>();
                spdlog::debug("鼠标按下: button={} at ({}, {})", static_cast<int>(mouse->button),
                              mouse->position.x, mouse->position.y);
            }
        }

        const sf::Time dt = deltaClock.restart();
        ImGui::SFML::Update(window, dt);
        rotation += props.rotationSpeed * dt.asSeconds();

        recordFrameTime(dt.asSeconds());   // 供 Plots tab 绘制帧耗时

        // 1. SFML 渲染画布
        canvas.draw(props, rotation, dt.asSeconds());

        // 2. ImGui 界面(控制面板 + 画布窗口 + 日志窗口)
        showControlPanel(props, rotation, canvas.getSize());
        showCanvasWindow(canvas, props);
        showLogWindow();

        // 3. 合成到窗口
        window.clear();
        ImGui::SFML::Render(window);
        window.display();
    }

    ImGui::SFML::Shutdown();
    spdlog::info("主循环结束, ImGui-SFML 已释放");
}
