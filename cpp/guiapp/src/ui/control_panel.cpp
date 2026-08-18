#include "control_panel.hpp"

#include <spdlog/spdlog.h>

#include "imgui.h"

#include "core/config.hpp"
#include "core/shape_props.hpp"
#include "ui/plots.hpp"

#include <cmath>

void showControlPanel(ShapeProps& props, float rotation, sf::Vector2u canvasSize)
{
    ImGui::SetNextWindowPos({20.f, 20.f}, ImGuiCond_FirstUseEver);
    ImGui::SetNextWindowSize({400.f, 540.f}, ImGuiCond_FirstUseEver);
    ImGui::Begin("控制面板");
    {
        // 学习点: 全局快捷键 - 不依赖当前激活的 tab, 随时可保存/加载配置
        if (ImGui::IsKeyPressed(ImGuiKey_S) && ImGui::GetIO().KeyCtrl) {
            saveConfig(props, "config.json");
        }
        if (ImGui::IsKeyPressed(ImGuiKey_L) && ImGui::GetIO().KeyCtrl) {
            loadConfig(props, "config.json");
        }

        ImGui::Text("帧率: %.1f FPS", ImGui::GetIO().Framerate);
        ImGui::Separator();

        if (ImGui::BeginTabBar("Tabs")) {
            // 学习点: 常用控件速查 + 实时控制画布中的 SFML 图形
            if (ImGui::BeginTabItem("控件速查")) {
                ImGui::SliderFloat("半径", &props.radius, 10.f, 200.f, "%.1f");
                ImGui::SliderFloat("位置 X", &props.x, 0.f, static_cast<float>(canvasSize.x));
                ImGui::SliderFloat("位置 Y", &props.y, 0.f, static_cast<float>(canvasSize.y));
                ImGui::SliderFloat("旋转速度", &props.rotationSpeed, -360.f, 360.f, "%.0f 度/秒");
                ImGui::Combo("形状类型", &props.shapeType, "圆形\0矩形\0三角形\0");
                ImGui::ColorEdit4("颜色", props.color);
                ImGui::Checkbox("填充", &props.filled);
                ImGui::Checkbox("显示网格", &props.showGrid);
                ImGui::Checkbox("显示文本", &props.showText);
                ImGui::Checkbox("显示粒子", &props.showParticles);
                ImGui::InputText("画布文本", props.text, sizeof(props.text));
                ImGui::ProgressBar(std::fmod(rotation + 360.f, 360.f) / 360.f, {0.f, 0.f});

                if (ImGui::Button("重置")) {
                    props = {};
                }
                ImGui::SameLine();
                if (ImGui::Button("打个招呼")) {
                    spdlog::info("按钮被点击: 打个招呼");
                }
                ImGui::EndTabItem();
            }

            if (ImGui::BeginTabItem("使用说明")) {
                ImGui::BulletText("画布由 sf::RenderTexture 离屏渲染, 再通过 ImGui::SFML::Image 嵌入 ImGui 窗口显示。");
                ImGui::BulletText("画布中的图形属性全部由 ImGui 控件驱动, 可实时调节。");
                ImGui::BulletText("在画布上按住鼠标左键可拖拽形状。");
                ImGui::BulletText("运行 SPDLOG_LEVEL=debug 可查看按键/鼠标事件日志。");
                ImGui::EndTabItem();
            }

            // 学习点: 配置保存/加载 - 用 nlohmann/json 序列化 ShapeProps 到 config.json
            // (快捷键 Ctrl+S / Ctrl+L 为全局, 见 showControlPanel 开头)
            if (ImGui::BeginTabItem("配置")) {
                ImGui::TextWrapped("将图形属性保存到 / 从 config.json(工作目录) 加载。");
                if (ImGui::Button("保存 (Ctrl+S)")) {
                    saveConfig(props, "config.json");
                }
                ImGui::SameLine();
                if (ImGui::Button("加载 (Ctrl+L)")) {
                    loadConfig(props, "config.json");
                }
                ImGui::EndTabItem();
            }

            // 学习点: 数据可视化 - 帧耗时折线图与直方图
            showPlotsTab();
        }
        ImGui::EndTabBar();
    }
    ImGui::End();
}
