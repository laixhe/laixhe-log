#pragma once

#include <SFML/Audio/Music.hpp>
#include <SFML/Audio/Sound.hpp>
#include <SFML/Audio/SoundBuffer.hpp>

#include <memory>
#include <string>
#include <vector>

// ---------------------------------------------------------------------------
// 学习点: 音频播放 + 程序化生成 WAV
//   sf::SoundBuffer  音频数据(驻留内存, 可重复播放)
//   sf::Sound        播放 SoundBuffer 里的短音效
//   sf::Music        流式播放, 不一次性载入内存, 适合背景音乐(可循环)
//   WAV 格式: RIFF 头 + fmt 块 + data 块; 这里用正弦波直接合成 PCM 采样,
//             无需任何外部音频文件, 顺便理解音频数据的本质
// 注意: CMake 里需开启 SFML_BUILD_AUDIO 并链接 SFML::Audio
// ---------------------------------------------------------------------------

// 一个音符: 频率(Hz) + 时长(秒), 如 {440.f, 0.5f} 即 440Hz 的 A4 持续半秒
struct Note {
    float freq;
    float seconds;
};

// 把一串音符合成 16bit 单声道 WAV 文件(带淡入淡出包络, 避免爆音)
// 返回是否生成成功
bool generateWav(const std::string& path, const std::vector<Note>& notes, float sampleRate = 44100.f);

class SoundManager {
public:
    // 加载音效文件到 SoundBuffer, 并创建播放器(SFML 3 的 Sound 无默认构造)
    bool loadSound(const std::string& wavPath);
    // 播放音效(会从头重新播放); 未加载成功时无操作
    void playSound()
    {
        if (m_sound) {
            m_sound->play();
        }
    }

    // 打开背景音乐(流式), loop 是否循环播放
    bool openMusic(const std::string& wavPath, bool loop = true);
    void startMusic() { m_music.play(); }
    void stopMusic() { m_music.stop(); }

private:
    sf::SoundBuffer m_soundBuffer;
    std::unique_ptr<sf::Sound> m_sound;
    sf::Music m_music;
};
