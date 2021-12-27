#ifndef __CONFIG_FILE_H
#define __CONFIG_FILE_H

#include <iostream>
#include <fstream>
#include <string>
#include <algorithm>
#include <map>

class ConfigFile {
public:
    ConfigFile(std::string fileName);
    ~ConfigFile();
    
    std::string get(std::string name);

private:
    std::string m_fileName;
    std::map<std::string, std::string>  m_config_map;

    // 读取文件并解析内容
    void loadFile();
};

#endif //!__CONFIG_FILE_H