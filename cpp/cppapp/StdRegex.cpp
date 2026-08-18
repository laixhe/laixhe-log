#include "StdRegex.h"

#include <format>     // std::format [C++20]
#include <iostream>
#include <regex>      // std::regex [C++11]
#include <string>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl

StdRegex::StdRegex()
{
    // ===== 1. 正则语法速览：字符类 / 量词 / 锚点 / 词边界 =====
    std::cout << "--- 语法速览 ---" << std::endl;

    // 字符类：\d 数字、\w 单词字符、\s 空白（等价 [0-9] [A-Za-z0-9_] [ \t\n]）
    const std::string str5 = "5";
    const std::string word = "abc_1";
    PRINT("\\d 匹配单个数字: {}", std::regex_match(str5, std::regex(R"(\d)")));     // true
    // 注意：regex_match 要求整体匹配，\w 只匹配单个字符，单词串要用 \w+
    PRINT("\\w+ 匹配单词串: {}", std::regex_match(word, std::regex(R"(\w+)")));     // true

    // 量词：* 0 次或多次、+ 1 次或多次、? 0 次或 1 次、{2,4} 2 到 4 次
    const std::string s12345 = "12345", s123 = "123";
    PRINT("\\d{{2,4}} 匹配 5 位数字: {}", std::regex_match(s12345, std::regex(R"(\d{2,4})"))); // false（超长）
    PRINT("\\d{{2,4}} 匹配 3 位数字: {}", std::regex_match(s123, std::regex(R"(\d{2,4})")));   // true

    // 词边界 \b：匹配完整的词，而不是更长词的一部分
    const std::string catalog = "catalog", phrase = "a cat eats";
    PRINT("\\bcat\\b 不匹配 catalog: {}", std::regex_search(catalog, std::regex(R"(\bcat\b)"))); // false
    PRINT("\\bcat\\b 匹配 a cat eats: {}", std::regex_search(phrase, std::regex(R"(\bcat\b)")));  // true

    // ===== 2. 匹配判断：regex_match（整个字符串匹配，对应 Go MatchString）=====
    std::cout << "--- 匹配 ---" << std::endl;

    // 手机号：1 开头，11 位数字（^ 与 $ 表示整体匹配，regex_match 默认就要整体）
    std::regex phone(R"(^1[3-9]\d{9}$)");
    const std::string okPhone = "13800138000", badPhone = "12345";
    PRINT("手机号匹配: {}", std::regex_match(okPhone, phone)); // true
    PRINT("非手机号: {}", std::regex_match(badPhone, phone));  // false

    // ===== 3. 修饰标志：忽略大小写等（对应 Go (?i) / Rust (?i)）=====
    std::cout << "--- 修饰标志 ---" << std::endl;

    const std::string hello = "Hello World";
    PRINT("默认区分大小写: {}", std::regex_search(hello, std::regex("hello")));           // false
    PRINT("icase 忽略大小写: {}", std::regex_search(hello, std::regex("hello", std::regex::icase))); // true

    // ===== 4. 搜索：regex_search（查找子串，对应 Python re.search）=====
    std::cout << "--- 搜索 ---" << std::endl;

    std::regex digit(R"(\d+)");
    std::smatch match;
    const std::string searchText = "价格 100 元";
    if (std::regex_search(searchText, match, digit)) {
        PRINT("搜索到数字: {}", match.str()); // 100
        // prefix() / suffix()：匹配位置前/后的剩余文本（对应 Go FindStringIndex 前后）
        PRINT("前缀: {} 后缀: {}", match.prefix().str(), match.suffix().str()); // 价格  元
    }

    // ===== 5. 查找所有：sregex_iterator（对应 Python re.findall / Go FindAll）=====
    std::cout << "--- 查找所有 ---" << std::endl;

    const std::string text = "a1 b22 c333";
    auto begin = std::sregex_iterator(text.begin(), text.end(), digit);
    auto end = std::sregex_iterator();
    std::cout << "所有数字: ";
    for (auto it = begin; it != end; ++it) {
        std::cout << (*it).str() << " ";
    }
    std::cout << std::endl; // 1 22 333

    // ===== 6. 捕获分组（对应 Python 分组 / Rust 捕获组）=====
    std::cout << "--- 捕获分组 ---" << std::endl;

    std::regex date(R"((\d{4})-(\d{2})-(\d{2}))");
    std::smatch dm;
    const std::string dateText = "日期：2026-03-04";
    if (std::regex_search(dateText, dm, date)) {
        // dm[0] 完整匹配，dm[1..3] 为三个捕获组
        PRINT("完整: {}，年: {}，月: {}，日: {}", dm[0].str(), dm[1].str(), dm[2].str(), dm[3].str());
    }

    // ===== 7. 替换：regex_replace（对应 Python re.sub / Go ReplaceAll）=====
    std::cout << "--- 替换 ---" << std::endl;

    // 替换所有数字为 [数字]
    const std::string score = "年龄 18 岁，得分 88";
    PRINT("替换: {}", std::regex_replace(score, digit, "[数字]"));
    // 年龄 [数字] 岁，得分 [数字]

    // 分组引用替换：$1 $2 $3（对应 Python re.sub 的 \1）
    const std::string dateStr = "2026-03-04";
    PRINT("分组引用: {}", std::regex_replace(dateStr, date, "$2/$3/$1"));
    // 03/04/2026

    // ===== 8. 综合应用：邮箱格式验证（整合 字符类+量词+锚点）=====
    std::cout << "--- 邮箱验证 ---" << std::endl;

    // 用户名(字母数字._%+-) + @ + 域名(字母数字.-) + . + 顶级域(2 个以上字母)
    std::regex email(R"(^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$)");
    const std::string okEmail = "laixhe@example.com";
    const std::string badEmail = "laixhe@@example.com";
    PRINT("合法邮箱: {}", std::regex_match(okEmail, email));   // true
    PRINT("非法邮箱: {}", std::regex_match(badEmail, email));  // false
}
