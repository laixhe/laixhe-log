#include "audio.hpp"

#include <cmath>
#include <cstdint>
#include <fstream>

bool generateWav(const std::string& path, const std::vector<Note>& notes, float sampleRate)
{
    if (notes.empty()) {
        return false;
    }

    // ---- 1. 合成 PCM 采样(16bit 单声道) ----
    constexpr float kPi = 3.14159265358979f;
    constexpr float kAmplitude = 0.3f;   // 音量峰值(0~1)

    std::vector<std::int16_t> samples;
    std::size_t total = 0;
    for (const Note& n : notes) {
        total += static_cast<std::size_t>(n.seconds * sampleRate);
    }
    samples.reserve(total);

    for (const Note& n : notes) {
        const std::size_t count = static_cast<std::size_t>(n.seconds * sampleRate);
        const float fade = 0.02f;   // 前/后 2% 时长做淡入淡出, 消除"啪"的爆音
        for (std::size_t i = 0; i < count; ++i) {
            const float t = static_cast<float>(i) / sampleRate;
            // 包络: 起始段渐入, 结束段渐出
            const float p = static_cast<float>(i) / static_cast<float>(count);   // 0~1
            const float env = std::min(std::min(p / fade, (1.f - p) / fade), 1.f);
            const float value = kAmplitude * env * std::sin(2.f * kPi * n.freq * t);
            samples.push_back(static_cast<std::int16_t>(value * 32767.f));
        }
    }

    // ---- 2. 写 WAV 文件(RIFF 容器) ----
    std::ofstream out(path, std::ios::binary);
    if (!out) {
        return false;
    }

    const std::uint32_t dataSize = static_cast<std::uint32_t>(samples.size()) * 2u;   // 字节数
    const std::uint32_t sampleRateU32 = static_cast<std::uint32_t>(sampleRate);
    const std::uint32_t chunkSize = dataSize + 36;   // 整个文件去掉 "RIFF"+长度 的 8 字节

    // RIFF 块
    out.write("RIFF", 4);
    out.write(reinterpret_cast<const char*>(&chunkSize), 4);
    out.write("WAVE", 4);

    // fmt 块: 说明编码格式
    out.write("fmt ", 4);
    const std::uint32_t fmtSize = 16;
    out.write(reinterpret_cast<const char*>(&fmtSize), 4);
    const std::uint16_t audioFormat = 1;      // 1 = PCM(未压缩)
    const std::uint16_t numChannels = 1;      // 单声道
    const std::uint16_t bitsPerSample = 16;
    const std::uint32_t byteRate = sampleRateU32 * numChannels * bitsPerSample / 8;
    const std::uint16_t blockAlign = numChannels * bitsPerSample / 8;
    out.write(reinterpret_cast<const char*>(&audioFormat), 2);
    out.write(reinterpret_cast<const char*>(&numChannels), 2);
    out.write(reinterpret_cast<const char*>(&sampleRateU32), 4);
    out.write(reinterpret_cast<const char*>(&byteRate), 4);
    out.write(reinterpret_cast<const char*>(&blockAlign), 2);
    out.write(reinterpret_cast<const char*>(&bitsPerSample), 2);

    // data 块: 真正的采样数据
    out.write("data", 4);
    out.write(reinterpret_cast<const char*>(&dataSize), 4);
    out.write(reinterpret_cast<const char*>(samples.data()), static_cast<std::streamsize>(dataSize));

    out.close();
    return true;
}

bool SoundManager::loadSound(const std::string& wavPath)
{
    if (!m_soundBuffer.loadFromFile(wavPath)) {
        return false;
    }
    m_sound = std::make_unique<sf::Sound>(m_soundBuffer);
    return true;
}

bool SoundManager::openMusic(const std::string& wavPath, bool loop)
{
    if (!m_music.openFromFile(wavPath)) {
        return false;
    }
    m_music.setLooping(loop);
    return true;
}
