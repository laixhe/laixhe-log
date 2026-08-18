#include "log_window.hpp"

#include <spdlog/sinks/base_sink.h>
#include <spdlog/spdlog.h>

#include "imgui.h"

#include <deque>
#include <memory>
#include <mutex>
#include <string>

namespace {

// 学习点: spdlog 自定义 sink - 继承 base_sink, 在 sink_it_ 中把格式化后的日志存入环形缓冲
class ImGuiLogSink final : public spdlog::sinks::base_sink<std::mutex> {
protected:
    void sink_it_(const spdlog::details::log_msg& msg) override
    {
        spdlog::memory_buf_t formatted;
        formatter_->format(msg, formatted);
        std::lock_guard lock(m_mutex);
        m_entries.push_back({msg.level, fmt::to_string(formatted)});
        if (m_entries.size() > kMaxEntries) {
            m_entries.pop_front();
        }
    }
    void flush_() override {}

public:
    struct Entry {
        spdlog::level::level_enum level;
        std::string message;
    };

    static constexpr std::size_t kMaxEntries = 200;

    std::mutex m_mutex;
    std::deque<Entry> m_entries;
};

std::shared_ptr<ImGuiLogSink> g_sink;

const char* levelLabel(spdlog::level::level_enum level)
{
    switch (level) {
    case spdlog::level::debug:    return "debug";
    case spdlog::level::info:     return "info";
    case spdlog::level::warn:     return "warn";
    case spdlog::level::err:      return "error";
    case spdlog::level::critical: return "critical";
    default:                      return "trace";
    }
}

ImVec4 levelColor(spdlog::level::level_enum level)
{
    switch (level) {
    case spdlog::level::debug:    return {0.6f, 0.6f, 0.6f, 1.f};
    case spdlog::level::info:     return {0.4f, 0.8f, 1.f, 1.f};
    case spdlog::level::warn:     return {1.f, 0.8f, 0.2f, 1.f};
    case spdlog::level::err:      return {1.f, 0.4f, 0.4f, 1.f};
    case spdlog::level::critical: return {1.f, 0.2f, 0.2f, 1.f};
    default:                      return {1.f, 1.f, 1.f, 1.f};
    }
}

// 过滤选项: 0=全部 1=info+ 2=warn+ 3=error+, 对应 spdlog 级别阈值
constexpr int kLevelThresholds[] = {0, 2, 3, 4};

}  // namespace

void initImGuiLogger()
{
    if (!g_sink) {
        g_sink = std::make_shared<ImGuiLogSink>();
        spdlog::default_logger()->sinks().push_back(g_sink);
    }
}

void showLogWindow()
{
    ImGui::SetNextWindowPos({1280.f - 460.f, 800.f - 260.f}, ImGuiCond_FirstUseEver);
    ImGui::SetNextWindowSize({440.f, 240.f}, ImGuiCond_FirstUseEver);
    ImGui::Begin("日志");
    {
        static int filterIdx = 0;
        ImGui::SetNextItemWidth(120.f);
        ImGui::Combo("过滤", &filterIdx, "全部\0信息+\0警告+\0错误+\0");
        ImGui::SameLine();
        if (ImGui::Button("清空")) {
            std::lock_guard lock(g_sink->m_mutex);
            g_sink->m_entries.clear();
        }
        ImGui::SameLine();
        ImGui::TextDisabled("(最新在底部)");
        ImGui::Separator();

        const int threshold = kLevelThresholds[filterIdx];
        std::lock_guard lock(g_sink->m_mutex);
        ImGui::BeginChild("log_scroll", {0.f, 0.f}, true);
        for (const auto& e : g_sink->m_entries) {
            if (static_cast<int>(e.level) < threshold) {
                continue;
            }
            ImGui::TextColored(levelColor(e.level), "[%s] %s", levelLabel(e.level), e.message.c_str());
        }
        // 自动滚动: 仅当用户已在底部时跟随最新日志
        if (ImGui::GetScrollY() >= ImGui::GetScrollMaxY()) {
            ImGui::SetScrollHereY(1.f);
        }
        ImGui::EndChild();
    }
    ImGui::End();
}
