#pragma once

// 学习点: 自定义 spdlog sink 把日志收集到内存, 并在 ImGui 窗口中显示
void initImGuiLogger();   // 向默认 spdlog logger 注册日志收集 sink, 应在初始化后调用一次
void showLogWindow();     // 渲染日志窗口(右下角, 可折叠/过滤/清空)
