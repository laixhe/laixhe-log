#include "plots.hpp"

#include "imgui.h"

#include <array>
#include <cmath>

namespace {

constexpr int kSamples = 200;                        // 帧耗时采样点数量
std::array<float, kSamples> g_frameTimes{};          // 环形缓冲
int g_index = 0;

constexpr int kHistBins = 10;
std::array<float, kHistBins> g_hist{};

}  // namespace

void recordFrameTime(float dtSeconds)
{
    g_frameTimes[g_index] = dtSeconds * 1000.f;      // 转为毫秒
    g_index = (g_index + 1) % kSamples;
}

void showPlotsTab()
{
    if (ImGui::BeginTabItem("图表")) {
        // 学习点: 折线图 - 实时帧耗时(最近 200 帧)
        ImGui::Text("帧耗时 (毫秒, 最近 %d 帧):", kSamples);
        ImGui::PlotLines("##frame_time", g_frameTimes.data(), kSamples, g_index, nullptr, 0.f, 33.3f,
                         {0.f, 120.f});

        // 学习点: 直方图 - 演示用的示例数据(正弦分布)
        for (int i = 0; i < kHistBins; ++i) {
            g_hist[i] = std::sin(i * 0.8f) + 1.f;
        }
        ImGui::Text("直方图 (示例数据):");
        ImGui::PlotHistogram("##hist", g_hist.data(), kHistBins, 0, nullptr, 0.f, 2.5f, {0.f, 120.f});

        ImGui::EndTabItem();
    }
}
