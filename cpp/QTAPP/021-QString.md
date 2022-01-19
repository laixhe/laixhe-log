> QString 也是封装了字符串，但是内部的编码为 utf8

#### 构造函数
```
// 构造一个空字符串对象
QString::QString();
// 将 char* 字符串 转换为 QString 类型
QString::QString(const char *str);
// 将 QByteArray 转换为 QString 类型
QString::QString(const QByteArray &ba);
```

#### 数据操作
```
// 尾部追加数据
// 其他重载的同名函数可参考Qt帮助文档, 此处略
QString &QString::append(const QString &str);
QString &QString::append(const char *str);
QString &QString::append(const QByteArray &ba);
void QString::push_back(const QString &other);

// 头部添加数据
// 其他重载的同名函数可参考Qt帮助文档, 此处略
QString &QString::prepend(const QString &str);
QString &QString::prepend(const char *str);
QString &QString::prepend(const QByteArray &ba);
void QString::push_front(const QString &other);

// 插入数据, 将 str 插入到字符串第 position 个字符的位置(从0开始)
// 其他重载的同名函数可参考Qt帮助文档, 此处略
QString &QString::insert(int position, const QString &str);
QString &QString::insert(int position, const char *str);
QString &QString::insert(int position, const QByteArray &str);

// 删除数据
// 从大字符串中删除len个字符, 从第pos个字符的位置开始删除
QString &QString::remove(int position, int n);

// 从字符串的尾部删除 n 个字符
void QString::chop(int n);
// 从字节串的 position 位置将字符串截断 (前边部分留下, 后边部分被删除)
void QString::truncate(int position);
// 将对象中的数据清空, 使其为null
void QString::clear();

// 字符串替换
// 将字节数组中的 子字符串 before 替换为 after
// 参数 cs 为是否区分大小写, 默认区分大小写
// 其他重载的同名函数可参考Qt帮助文档, 此处略
QString &QString::replace(const QString &before, const QString &after, Qt::CaseSensitivity cs = Qt::CaseSensitive);
```

#### 子字符串查找和判断
```
// 参数 cs 为是否区分大小写, 默认区分大小写
// 其他重载的同名函数可参考Qt帮助文档, 此处略

// 判断字符串中是否包含子字符串 str, 包含返回true, 否则返回false
bool QString::contains(const QString &str, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

// 判断字符串是否以字符串 ba 开始, 是返回true, 不是返回false
bool QString::startsWith(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;

// 判断字符串是否以字符串 ba 结尾, 是返回true, 不是返回false
bool QString::endsWith(const QString &s, Qt::CaseSensitivity cs = Qt::CaseSensitive) const;
```

#### 遍历
```
// 使用迭代器
iterator QString::begin();
iterator QString::end();

// 使用数组的方式进行遍历
// i的取值范围 0 <= position < size()
const QChar QString::at(int position) const
const QChar QString::operator[](int position) const;
```

#### 类型转换
```
// 将int, short, long, float, double 转换为 QString 类型
// 其他重载的同名函数可参考Qt帮助文档, 此处略
QString &QString::setNum(int n, int base = 10);
QString &QString::setNum(short n, int base = 10);
QString &QString::setNum(long n, int base = 10);
QString &QString::setNum(float n, char format = 'g', int precision = 6);
QString &QString::setNum(double n, char format = 'g', int precision = 6);
[static] QString QString::number(long n, int base = 10);
[static] QString QString::number(int n, int base = 10);
[static] QString QString::number(double n, char format = 'g', int precision = 6);

// 将 QString 转换为 int, short, long, float, double 类型
int QString::toInt(bool *ok = Q_NULLPTR, int base = 10) const;
short QString::toShort(bool *ok = Q_NULLPTR, int base = 10) const;
long QString::toLong(bool *ok = Q_NULLPTR, int base = 10) const
float QString::toFloat(bool *ok = Q_NULLPTR) const;
double QString::toDouble(bool *ok = Q_NULLPTR) const;

// 将标准C++中的 std::string 类型 转换为 QString 类型
[static] QString QString::fromStdString(const std::string &str);
// 将 QString 转换为 标准C++中的 std::string 类型
std::string QString::toStdString() const;

// QString -> QByteArray
// 转换为本地编码, 跟随操作系统
QByteArray QString::toLocal8Bit() const;
// 转换为 Latin-1 编码的字符串 不支持中文
QByteArray QString::toLatin1() const;
// 转换为 utf8 编码格式的字符串 (常用)
QByteArray QString::toUtf8() const;

// 所有字符转换为大写
QString QString::toUpper() const;
// 所有字符转换为小写
QString QString::toLower() const;
```

#### 字符串格式
```
QString QString::arg(const QString &a, 
          int fieldWidth = 0, 
          QChar fillChar = QLatin1Char( ' ' )) const;
QString QString::arg(int a, int fieldWidth = 0, 
          int base = 10, 
          QChar fillChar = QLatin1Char( ' ' )) const;

// 示例程序
int i;                // 假设该变量表示当前文件的编号
int total;            // 假设该变量表示文件的总个数
QString fileName;     // 假设该变量表示当前文件的名字
// 使用以上三个变量拼接一个动态字符串
QString status = QString("Processing file %1 of %2: %3")
                  .arg(i).arg(total).arg(fileName);

```
