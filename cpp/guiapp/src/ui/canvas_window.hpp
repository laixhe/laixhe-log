#pragma once

struct ShapeProps;
class Canvas;

// 学习点: 把 RenderTexture 以纹理形式嵌入 ImGui 窗口, 并处理鼠标拖拽交互
// 返回是否正在拖拽(供主循环判断是否需要特殊光标等)
bool showCanvasWindow(Canvas& canvas, ShapeProps& props);
