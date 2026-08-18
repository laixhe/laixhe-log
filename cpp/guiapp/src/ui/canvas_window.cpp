#include "canvas_window.hpp"

#include <algorithm>

#include "imgui.h"
#include "imgui-SFML.h"

#include "core/canvas.hpp"
#include "core/shape_props.hpp"

bool showCanvasWindow(Canvas& canvas, ShapeProps& props)
{
    static bool draggingShape = false;

    ImGui::SetNextWindowPos({440.f, 20.f}, ImGuiCond_FirstUseEver);
    ImGui::SetNextWindowSize({900.f, 650.f}, ImGuiCond_FirstUseEver);
    ImGui::Begin("SFML 画布");
    {
        // 学习点: 纹理嵌入 ImGui 窗口, 并按内容区自适应缩放保持宽高比
        // 注意: 必须传 RenderTexture 本体(而非 getTexture()),
        // imgui-sfml 的 RenderTexture 重载会处理 OpenGL 纹理的垂直翻转
        const sf::Vector2u canvasSize = canvas.getSize();
        const sf::Vector2f avail{ImGui::GetContentRegionAvail().x,
                                 ImGui::GetContentRegionAvail().y};
        const float aspect = static_cast<float>(canvasSize.x) / static_cast<float>(canvasSize.y);
        sf::Vector2f imgSize(avail.x, avail.x / aspect);
        if (imgSize.y > avail.y) {
            imgSize.y = avail.y;
            imgSize.x = avail.y * aspect;
        }
        ImGui::Image(canvas.getRenderTexture(), imgSize);

        // 学习点: 屏幕坐标 <-> 纹理坐标映射, 实现鼠标拖拽形状
        const ImVec2 itemMin = ImGui::GetItemRectMin();
        const ImVec2 itemMax = ImGui::GetItemRectMax();
        const ImVec2 mouse = ImGui::GetMousePos();

        auto toCanvasCoords = [&](float sx, float sy) {
            const float cx = (sx - itemMin.x) / (itemMax.x - itemMin.x) * canvasSize.x;
            const float cy = (sy - itemMin.y) / (itemMax.y - itemMin.y) * canvasSize.y;
            return sf::Vector2f{std::clamp(cx, 0.f, static_cast<float>(canvasSize.x)),
                                std::clamp(cy, 0.f, static_cast<float>(canvasSize.y))};
        };

        if (ImGui::IsItemHovered() && ImGui::IsMouseClicked(ImGuiMouseButton_Left)) {
            const sf::Vector2f p = toCanvasCoords(mouse.x, mouse.y);
            const float hitRadius = props.radius * (props.shapeType == 1 ? 1.6f : 1.2f);
            const float dx = p.x - props.x;
            const float dy = p.y - props.y;
            if (dx * dx + dy * dy <= hitRadius * hitRadius) {
                draggingShape = true;
            }
        }
        if (draggingShape) {
            const sf::Vector2f p = toCanvasCoords(mouse.x, mouse.y);
            props.x = p.x;
            props.y = p.y;
            if (!ImGui::IsMouseDown(ImGuiMouseButton_Left)) {
                draggingShape = false;
            }
        }
        if (draggingShape) {
            ImGui::SetMouseCursor(ImGuiMouseCursor_Hand);
        }
    }
    ImGui::End();

    return draggingShape;
}
