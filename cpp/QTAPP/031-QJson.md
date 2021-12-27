> 从 Qt 5.0 开始提供了对 Json 的支持，我们可以直接使用 Qt 提供的 Json 类进行数据的组织和解析
>
> QJsonDocument 它封装了一个完整的 JSON 文档，并且可以从 UTF-8 编码的基于文本的表示以及 Qt 自己的二进制格式读取和写入该文档
>
> QJsonArray JSON 数组是一个值列表。可以通过从数组中插入和删除 QJsonValue 来操作该列表
>
> QJsonObject JSON 对象是键值对的列表，其中键是唯一的字符串，值由 QJsonValue 表示
>
> QJsonValue 该
> 类封装了 JSON 支持的数据类型
>

#### QJsonDocument
> 对 QJsonObject 和 QJsonArray 这两个对象中的数据是不能直接转换为字符串类型的，如果要进行数据传输或者数据的持久化，操作的都是字符串类型而不是 QJsonObject 或者 QJsonArray 类型，我们需要通过一个 QJsonDocument 文档类进行二者之间的转换

> QJsonObject 或者 QJsonArray `转为` 字符串

1. 创建 QJsonDocument 对象
```
QJsonDocument::QJsonDocument(const QJsonObject &object);
QJsonDocument::QJsonDocument(const QJsonArray &array);
```

2. 将文件对象中的数据进行序列化，通过调用 toxxx() 方法就可以得到文本格式或者二进制格式的 Json 字符串了
```
// 二进制格式的json字符串
QByteArray QJsonDocument::toBinaryData() const;
// 文本格式
QByteArray QJsonDocument::toJson(JsonFormat format = Indented) const;
```

> 字符串 `转为` QJsonObject 或者 QJsonArray

1. 将得到的 Json 格式字符串通过 QJsonDocument 类的静态函数转换为 QJsonDocument 类对象
```
[static] QJsonDocument QJsonDocument::fromBinaryData(const QByteArray &data, DataValidation validation = Validate);
// 参数文件格式的json字符串
[static] QJsonDocument QJsonDocument::fromJson(const QByteArray &json, QJsonParseError *error = Q_NULLPTR);
```

2. 将文档对象转换为 json 数组 / 对象
```
// 判断文档对象中存储的数据是不是数组
bool QJsonDocument::isArray() const;
// 判断文档对象中存储的数据是不是json对象
bool QJsonDocument::isObject() const
    
// 文档对象中的数据转换为json对象
QJsonObject QJsonDocument::object() const;
// 文档对象中的数据转换为json数组
QJsonArray QJsonDocument::array() const;
```

3. 通过调用 QJsonArray , QJsonObject 类提供的 API 读出存储在对象中的数据


#### QJsonValue
> 在 Qt 中 QJsonValue 可以封装的基础数据类型有六种（和 Json 支持的类型一致）
>
> QJsonValue::Bool 布尔类型
>
> QJsonValue::Double 浮点类型（包括整形
>
> QJsonValue::String 字符串类型
>
> QJsonValue::Array 数组类型
>
> QJsonValue::Object 对象类型
>
> QJsonValue::Null 空值类型
>

这个类型可以通过 QJsonValue 的构造函数被封装为一个类对象:
```
// Json对象
QJsonValue(const QJsonObject &o);
// Json数组
QJsonValue(const QJsonArray &a);
// 字符串
QJsonValue(const char *s);
QJsonValue(QLatin1String s);
QJsonValue(const QString &s);
// 整形 and 浮点型
QJsonValue(qint64 v);
QJsonValue(int v);
QJsonValue(double v);
// 布尔类型
QJsonValue(bool b);
// 空值类型
QJsonValue(QJsonValue::Type type = Null);
```
如果我们得到一个 QJsonValue 对象，如何判断内部封装的到底是什么类型的数据呢？这时候就需要调用相关的判断函数了，具体如下：
```
// 是否是Json数组
bool isArray() const;
// 是否是Json对象
bool isObject() const;
// 是否是布尔类型
bool isBool() const;
// 是否是浮点类型(整形也是通过该函数判断)
bool isDouble() const;
// 是否是空值类型
bool isNull() const;
// 是否是字符串类型
bool isString() const;
// 是否是未定义类型(无法识别的类型)
bool isUndefined() const;
```

通过判断函数得到对象内部数据的实际类型之后，如果有需求就可以再次将其转换为对应的基础数据类型，对应的 API 函数如下：
```
// 转换为Json数组
QJsonArray toArray(const QJsonArray &defaultValue) const;
QJsonArray toArray() const;
// 转换为布尔类型
bool toBool(bool defaultValue = false) const;
// 转换为浮点类型
double toDouble(double defaultValue = 0) const;
// 转换为整形
int toInt(int defaultValue = 0) const;
// 转换为Json对象
QJsonObject toObject(const QJsonObject &defaultValue) const;
QJsonObject toObject() const;
// 转换为字符串类型
QString toString() const;
QString toString(const QString &defaultValue) const;
```

#### QJsonObject
> 封装了 Json 中的对象，在里边可以存储多个键值对，为了方便操作，键值为字符串类型，值为 QJsonValue 类型

1. 如何创建空的 Json 对象
```
QJsonObject::QJsonObject();	// 构造空对象
```

2. 将键值对添加到空对象中
```
iterator QJsonObject::insert(const QString &key, const QJsonValue &value);
```

3. 通过 key 得到 value
```
QJsonValue QJsonObject::value(const QString &key) const;    // utf8
QJsonValue QJsonObject::value(QLatin1String key) const;	    // 字符串不支持中文
QJsonValue QJsonObject::operator[](const QString &key) const;
QJsonValue QJsonObject::operator[](QLatin1String key) const;
```

4. 删除键值对
```
void QJsonObject::remove(const QString &key);
QJsonValue QJsonObject::take(const QString &key);	// 返回key对应的value值
```

5. 通过 key 进行查找
```
iterator QJsonObject::find(const QString &key);
bool QJsonObject::contains(const QString &key) const;
```

#### QJsonArray
> 封装了 Json 中的数组，在里边可以存储多个元素，为了方便操作，所有的元素类统一为 QJsonValue 类型

1. 创建空的 Json 数组
```
QJsonArray::QJsonArray();
```

2. 添加数据
```
void QJsonArray::append(const QJsonValue &value);	// 在尾部追加
void QJsonArray::insert(int i, const QJsonValue &value); // 插入到 i 的位置之前
iterator QJsonArray::insert(iterator before, const QJsonValue &value);
void QJsonArray::prepend(const QJsonValue &value); // 添加到数组头部
void QJsonArray::push_back(const QJsonValue &value); // 添加到尾部
void QJsonArray::push_front(const QJsonValue &value); // 添加到头部
```

3. 从数组中取出某一个元素的值
```
QJsonValue QJsonArray::at(int i) const;
QJsonValue QJsonArray::first() const; // 头部元素
QJsonValue QJsonArray::last() const; // 尾部元素
QJsonValueRef QJsonArray::operator[](int i);
```

4. 删除数组中的某一个元素
```
iterator QJsonArray::erase(iterator it);    // 基于迭代器删除
void QJsonArray::pop_back();           // 删除尾部
void QJsonArray::pop_front();          // 删除头部
void QJsonArray::removeAt(int i);      // 删除i位置的元素
void QJsonArray::removeFirst();        // 删除头部
void QJsonArray::removeLast();         // 删除尾部
QJsonValue QJsonArray::takeAt(int i);  // 删除i位置的原始, 并返回删除的元素的值
```



