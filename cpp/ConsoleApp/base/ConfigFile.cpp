#include "ConfigFile.h"
#include "Strings.h"

// 要在 C++ 中进行文件处理，必须在 C++ 源代码文件中包含头文件 <iostream> 和 <fstream>
// 头文件 <fstream> 包含的多个文件流类
//
// 1. ifstream (Input file stream class) 该数据类型表示输入文件流，用于从文件读取信息
//    ifstream infile;
//    infile.open("afile.dat");  // 以读模式打开文件
//
// 2. ofstream (Output file stream class) 该数据类型表示输出文件流，用于创建文件并向文件写入信息
//     ofstream outfile;
//     outfile.open("afile.dat"); // 以写模式打开文件
//
// 3. fstream (Input/output file stream class) 该数据类型通常表示文件流，且同时具有 ofstream 和 ifstream 两种功能，这意味着它可以创建文件，向文件写入信息，从文件读取信息
//
// 4. filebuf (File stream buffe class)
//
// 文件模式
// ios::app   追加到文件末尾
// ios::in    打开文件用于读取 (文件数据输入到内存)
// ios::out   打开文件用于写入 (内存数据输出到文件)
// ios::trunc 如果该文件已经存在，其内容将在打开文件之前被截断，即把文件长度设为 0

ConfigFile::ConfigFile(std::string fileName) : m_fileName(fileName){
    loadFile();
}

ConfigFile::~ConfigFile(){
    for (auto a : m_config_map) {
    std::cout << "~ConfigFile()：" << a.first << "="  << a.second << std::endl;
    }
}

// 读取文件并解析内容
void ConfigFile::loadFile() {

    // 以读模式打开文件
    std::ifstream infile;
    infile.open(m_fileName);

    if (!infile.is_open()){
        std::cout << "打开配置文件失败："+ m_fileName << std::endl;
        return;
    }

    std::string value;
    // 逐行读取
    std::string line;
    while(std::getline(infile, line)){
        if (line.length() > 2) {
            // 忽略带有 # 的那行
            if(line.find('#') != -1){
                continue;
            }
            // 忽略没有 = 的那行
            if(line.find('=') == -1){
                continue;
            }

            // 清除所有空格
            line.erase(remove_if(line.begin(), line.end(), isspace), line.end());

            //line.erase(0, line.find_first_not_of(" "));
            //line.erase(line.find_last_not_of(" ") + 1);

            std::vector<std::string> strs;
            split(line, strs, '=');
            if(strs.size() == 1){
                m_config_map.insert(std::make_pair(strs[0], value));
            }
            if(strs.size() >= 2){
                m_config_map.insert(std::make_pair(strs[0], strs[1]));
            }
        }
    }
    


    // 关闭打开的文件
    infile.close();
}

std::string ConfigFile::get(std::string name){
    std::string value;

    if(name.empty()){
        return value;
    }
    auto it = m_config_map.find(name);
    if(it != m_config_map.end()){
        return it->second;
    }
    return value;
}