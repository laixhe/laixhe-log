## QDialog
> 对话框窗口类
.
> 模态(show)和非模态(exec)两种显示方式
>
> 不能内嵌
>
> 是 QWidget 类的子类, 处理继承自父类的属性之外, 还有一些自己所特有的属性
>

```
// 构造函数
QDialog::QDialog(QWidget *parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());

// 模态显示窗口
[virtual slot] int QDialog::exec();
// 隐藏模态窗口, 并且解除模态窗口的阻塞, 将 exec() 的返回值设置为 QDialog::Accepted
[virtual slot] void QDialog::accept();
// 隐藏模态窗口, 并且解除模态窗口的阻塞, 将 exec() 的返回值设置为 QDialog::Rejected
[virtual slot] void QDialog::reject();
// 关闭对话框并将其结果代码设置为r。finished()信号将发出r;
// 如果r是QDialog::Accepted 或 QDialog::Rejected，则还将分别发出accept()或Rejected()信号。
[virtual slot] void QDialog::done(int r);

[signal] void QDialog::accepted();
[signal] void QDialog::rejected();
[signal] void QDialog::finished(int result);
```

#### QMessageBox (开打提示对话框)
> 是 QDialog 类的子类，通过这个类可以显示一些简单的提示框，用于展示警告、错误、问题等信息

```
# 通用参数:
- parent: 对话框窗口的父窗口
- title: 对话框窗口的标题
- text: 对话框窗口中显示的提示信息
- buttons: 对话框窗口中显示的按钮(一个或多个)

QMessageBox::about(this, "提示", "提示信息...");
QMessageBox::information(this, "通知", "通知消息...");
QMessageBox::critical(this, "错误", "错误信息...");
QMessageBox::warning(this, "警告", "是否关闭？");
QMessageBox::question(this, "询问", "询问提示信息...");

# 如：
int ret = QMessageBox::question(this, "保存", "你是否要保存信息...", QMessageBox::Save|QMessageBox::Cancel, QMessageBox::Cancel);
if(ret == QMessageBox::Save){
    QMessageBox::information(this, "通知", "保存成功...");
}
if(ret == QMessageBox::Cancel){
    QMessageBox::warning(this, "警告", "你放弃了保存...");
}
```

#### QFileDialog (开打文件对话框)
> 是 QDialog 类的子类, 通过这个类可以选择要打开/保存的文件或者目录

```
# 通用参数
- parent: 当前对话框窗口的父对象也就是父窗口
- caption: 当前对话框窗口的标题
- dir: 当前对话框窗口打开的默认目录
- options: 当前对话框窗口的一些可选项,枚举类型, 一般不需要进行设置, 使用默认值即可
- filter: 过滤器, 在对话框中只显示满足条件的文件, 可以指定多个过滤器, 使用 ;; 分隔
-     样式举例:
-         Images (*.png *.jpg)
-         Images (*.png *.jpg);;Text files (*.txt)
- selectedFilter: 如果指定了多个过滤器, 通过该参数指定默认使用哪一个, 不指定默认使用第一个过滤器

# 如：
// 打开目录，返回绝对路径
QString dirName = QFileDialog::getExistingDirectory(this, "打开目录", "home/laixhe/Desktop");
QMessageBox::information(this, "打开目录", "打开目录:" + dirName);

// 打开一个文件, 得到这个文件的绝对路径
QString fileName = QFileDialog::getOpenFileName(this, "打开文件", "home/laixhe/Desktop", "Text files (*.txt)");
QMessageBox::information(this, "打开文件", "打开文件:" + fileName);

// 打开多个文件, 得到这多个文件的绝对路径 QFileDialog::getOpenFileNames

// 打开一个目录, 使用这个目录来保存指定的文件
QString fileName = QFileDialog::getSaveFileName(this, "保存文件");
QMessageBox::information(this, "保存文件", "保存文件:" + fileName);
```

#### QFontDialog (选择字体对话框)
> 是 QDialog 的子类，通过这个类我们可以得到一个进行字体属性设置的对话框窗口

##### QFont 字体类

```
# QFont 通用参数
- family: 本地字库中的字体名, 通过 office 等文件软件可以查看
- pointSize: 字体的字号
- weight: 字体的粗细, 有效范围为 0 ~ 99
- italic: 字体是否倾斜显示, 默认不倾斜
QFont::QFont(const QString &family, int pointSize = -1, int weight = -1, bool italic = false);
  
// 设置字体
void QFont::setFamily(const QString &family);
// 根据字号设置字体大小
void QFont::setPointSize(int pointSize);
// 根据像素设置字体大小
void QFont::setPixelSize(int pixelSize);
// 设置字体的粗细程度, 有效范围: 0 ~ 99
void QFont::setWeight(int weight);
// 设置字体是否加粗显示
void QFont::setBold(bool enable);
// 设置字体是否要倾斜显示
void QFont::setItalic(bool enable);

# QFontDialog 通用参数
- ok: 传出参数, 用于判断是否获得了有效字体信息, 指定一个布尔类型变量地址
- initial: 字体对话框中默认选中并显示该字体信息, 用于对话框的初始化
- parent: 字体对话框窗口的父对象
- title: 字体对话框的窗口标题
- options: 字体对话框选项, 使用默认属性即可, 一般不设置
[static] QFont QFontDialog::getFont(
		bool *ok, const QFont &initial, 
		QWidget *parent = nullptr, const QString &title = QString(), 
		QFontDialog::FontDialogOptions options = FontDialogOptions());
  
[static] QFont QFontDialog::getFont(bool *ok, QWidget *parent = nullptr);

# 如：
bool ok;
QFont ft = QFontDialog::getFont(&ok, QFont("微软雅黑", 12, QFont::Bold), this, "选择字体");
if(ok) {
    ui->fontDialogEdit->setFont(ft);
}
```

#### QColorDialog (选择颜色的对话框)
> 是 QDialog 的子类，通过这个类我们可以得到一个选择颜色的对话框窗口

##### QColor 颜色类
> 各种颜色都是基于红 , 绿 , 蓝这三种颜色调配而成的

```
// 构造函数
QColor::QColor(Qt::GlobalColor color);
QColor::QColor(int r, int g, int b, int a = ...);
QColor::QColor();

// 参数设置 red, green, blue, alpha, 取值范围都是 0-255
void QColor::setRed(int red);		// 红色
void QColor::setGreen(int green);	// 绿色
void QColor::setBlue(int blue);	// 蓝色
void QColor::setAlpha(int alpha);	// 透明度, 默认不透明(255)
void QColor::setRgb(int r, int g, int b, int a = 255);

int QColor::red() const;
int QColor::green() const;
int QColor::blue() const;
int QColor::alpha() const;
void QColor::getRgb(int *r, int *g, int *b, int *a = nullptr) const;

# QFontDialog 通用参数
- initial: 对话框中默认选中的颜色, 用于窗口初始化
- parent: 给对话框窗口指定父对象
- title: 对话框窗口的标题
- options: 颜色对话框窗口选项, 使用默认属性即可, 一般不需要设置
[static] QColor QColorDialog::getColor(
		const QColor &initial = Qt::white, 
		QWidget *parent = nullptr, const QString &title = QString(), 
		QColorDialog::ColorDialogOptions options = ColorDialogOptions());

```

#### QInputDialog (输入对话框)
> 是 QDialog 的子类，通过这个类我们可以得到一个输入对话框窗口，根据实际需求我们可以在这个输入窗口中输入整形 , 浮点型 , 字符串类型的数据，并且还可以显示下拉菜单供使用者选择

```
# 得到一个可以输入浮点数的对话框窗口, 返回对话框窗口中输入的浮点数
# 通用参数
- parent: 对话框窗口的父窗口
- title: 对话框窗口显示的标题信息
- label: 对话框窗口中显示的文本信息(用于描述对话框的功能)
- value: 对话框窗口中显示的浮点值, 默认为 0
- min: 对话框窗口支持显示的最小数值
- max: 对话框窗口支持显示的最大数值
- decimals: 浮点数的精度, 默认保留小数点以后1位
- ok: 传出参数, 用于判断是否得到了有效数据, 一般不会使用该参数
- flags: 对话框窗口的窗口属性, 使用默认值即可
[static] double QInputDialog::getDouble(
    		QWidget *parent, const QString &title, 
    		const QString &label, double value = 0, 
    		double min = -2147483647, double max = 2147483647, 
    		int decimals = 1, bool *ok = nullptr, 
    		Qt::WindowFlags flags = Qt::WindowFlags());

# 得到一个可以输入整形数的对话框窗口, 返回对话框窗口中输入的整形数
# 通用参数
- parent: 对话框窗口的父窗口
- title: 对话框窗口显示的标题信息
- label: 对话框窗口中显示的文本信息(用于描述对话框的功能)
- value: 对话框窗口中显示的整形值, 默认为 0
- min: 对话框窗口支持显示的最小数值
- max: 对话框窗口支持显示的最大数值
- step: 步长, 通过对话框提供的按钮调节数值每次增长/递减的量
- ok: 传出参数, 用于判断是否得到了有效数据, 一般不会使用该参数
- flags: 对话框窗口的窗口属性, 使用默认值即可
[static] int QInputDialog::getInt(
    		QWidget *parent, const QString &title, 
    		const QString &label, int value = 0, 
    		int min = -2147483647, int max = 2147483647, 
    		int step = 1, bool *ok = nullptr, 
    		Qt::WindowFlags flags = Qt::WindowFlags());

# 得到一个带下来菜单的对话框窗口, 返回选择的菜单项上边的文本信息
# 通用参数
- parent: 对话框窗口的父窗口
- title: 对话框窗口显示的标题信息
- label: 对话框窗口中显示的文本信息(用于描述对话框的功能)
- items: 字符串列表, 用于初始化窗口中的下拉菜单, 每个字符串对应一个菜单项
- current: 通过菜单项的索引指定显示下拉菜单中的哪个菜单项, 默认显示第一个(编号为0)
- editable: 设置菜单项上的文本信息是否可以进行编辑, 默认为true, 即可以编辑
- ok: 传出参数, 用于判断是否得到了有效数据, 一般不会使用该参数
- flags: 对话框窗口的窗口属性, 使用默认值即可
- inputMethodHints: 设置显示模式, 默认没有指定任何特殊显示格式, 显示普通文本字符串
- 如果有特殊需求, 可以参数帮助文档进行相关设置
[static] QString QInputDialog::getItem(
    		QWidget *parent, const QString &title, 
    		const QString &label, const QStringList &items, 
    		int current = 0, bool editable = true, bool *ok = nullptr, 
    		Qt::WindowFlags flags = Qt::WindowFlags(), 
    		Qt::InputMethodHints inputMethodHints = Qt::ImhNone);

# 得到一个可以输入多行数据的对话框窗口, 返回用户在窗口中输入的文本信息
# 通用参数
- parent: 对话框窗口的父窗口
- title: 对话框窗口显示的标题信息
- label: 对话框窗口中显示的文本信息(用于描述对话框的功能)
- text: 指定显示到多行输入框中的文本信息, 默认是空字符串
- ok: 传出参数, 用于判断是否得到了有效数据, 一般不会使用该参数
- flags: 对话框窗口的窗口属性, 使用默认值即可
- inputMethodHints: 设置显示模式, 默认没有指定任何特殊显示格式, 显示普通文本字符串
- 如果有特殊需求, 可以参数帮助文档进行相关设置
[static] QString QInputDialog::getMultiLineText(
    		QWidget *parent, const QString &title, const QString &label, 
    		const QString &text = QString(), bool *ok = nullptr, 
    		Qt::WindowFlags flags = Qt::WindowFlags(), 
    		Qt::InputMethodHints inputMethodHints = Qt::ImhNone);

# 得到一个可以输入单行信息的对话框窗口, 返回用户在窗口中输入的文本信息
# 通用参数
- parent: 对话框窗口的父窗口 
- title: 对话框窗口显示的标题信息
- label: 对话框窗口中显示的文本信息(用于描述对话框的功能)
- mode: 指定单行编辑框中数据的反馈模式, 是一个 QLineEdit::EchoMode 类型的枚举值
- QLineEdit::Normal: 显示输入的字符。这是默认值
- QLineEdit::NoEcho: 不要展示任何东西。这可能适用于连密码长度都应该保密的密码。
- QLineEdit::Password: 显示与平台相关的密码掩码字符，而不是实际输入的字符。
- QLineEdit::PasswordEchoOnEdit: 在编辑时按输入显示字符，否则按密码显示字符。
- text: 指定显示到单行输入框中的文本信息, 默认是空字符串
- ok: 传出参数, 用于判断是否得到了有效数据, 一般不会使用该参数
- flags: 对话框窗口的窗口属性, 使用默认值即可
- inputMethodHints: 设置显示模式, 默认没有指定任何特殊显示格式, 显示普通文本字符串
- 如果有特殊需求, 可以参数帮助文档进行相关设置
[static] QString QInputDialog::getText(
    		QWidget *parent, const QString &title, const QString &label,
    		QLineEdit::EchoMode mode = QLineEdit::Normal, 
    		const QString &text = QString(), bool *ok = nullptr, 
    		Qt::WindowFlags flags = Qt::WindowFlags(), 
    		Qt::InputMethodHints inputMethodHints = Qt::ImhNone);

```

#### QProgressDialog (进度条的对话框)
> 是 QDialog 的子类，通过这个类我们可以得到一个带进度条的对话框窗口，这种类型的对话框窗口一般常用于文件拷贝、数据传输等实时交互的场景中

```
# 通用参数
- labelText: 对话框中显示的提示信息
- cancelButtonText: 取消按钮上显示的文本信息
- minimum: 进度条最小值
- maximum: 进度条最大值
- parent: 当前窗口的父对象
- f: 当前进度窗口的flag属性, 使用默认属性即可, 无需设置

QProgressDialog::QProgressDialog(
	QWidget *parent = nullptr, 
	Qt::WindowFlags f = Qt::WindowFlags());

QProgressDialog::QProgressDialog(
	const QString &labelText, const QString &cancelButtonText, 
	int minimum, int maximum, QWidget *parent = nullptr,
	Qt::WindowFlags f = Qt::WindowFlags());


// 设置取消按钮显示的文本信息
[slot] void QProgressDialog::setCancelButtonText(const QString &cancelButtonText);

// 公共成员函数和槽函数
QString QProgressDialog::labelText() const;
void QProgressDialog::setLabelText(const QString &text);

// 得到进度条最小值
int QProgressDialog::minimum() const;
// 设置进度条最小值
void QProgressDialog::setMinimum(int minimum);

// 得到进度条最大值
int QProgressDialog::maximum() const;
// 设置进度条最大值
void QProgressDialog::setMaximum(int maximum);

// 设置进度条范围(最大和最小值)
[slot] void QProgressDialog::setRange(int minimum, int maximum);

// 得到进度条当前的值
int QProgressDialog::value() const;
// 设置进度条当前的值
void QProgressDialog::setValue(int progress);

bool QProgressDialog::autoReset() const;
// 当value() = maximum()时，进程对话框是否调用reset()，此属性默认为true。
void QProgressDialog::setAutoReset(bool reset);


bool QProgressDialog::autoClose() const;
// 当value() = maximum()时，进程对话框是否调用reset()并且隐藏，此属性默认为true。
void QProgressDialog::setAutoClose(bool close);

// 判断用户是否按下了取消键, 按下了返回true, 否则返回false
bool wasCanceled() const;

// 重置进度条
// 重置进度对话框。wascancelled()变为true，直到进程对话框被重置。进度对话框被隐藏。
[slot] void QProgressDialog::cancel();
// 重置进度对话框。如果autoClose()为真，进程对话框将隐藏。
[slot] void QProgressDialog::reset();   

// 信号
// 当单击cancel按钮时，将发出此信号。默认情况下，它连接到cancel()槽。
[signal] void QProgressDialog::canceled();

// 设置窗口的显示状态(模态, 非模态)
/*
参数:
	Qt::NonModal  -> 非模态
	Qt::WindowModal	-> 模态, 阻塞父窗口
	Qt::ApplicationModal -> 模态, 阻塞应用程序中的所有窗口
*/
void QWidget::setWindowModality(Qt::WindowModality windowModality);
```

