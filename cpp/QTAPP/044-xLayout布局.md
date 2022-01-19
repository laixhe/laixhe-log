> 在 QT 中，布局也有对应的类，布局类之间的关系

```
       QObject       QLayoutltem
          ^               ^
          |___ QLayout____|
                ^    ^
            ____|    |____
            |            |
        QBoxLayout  QGridLayout
          ^    ^
        __|    |__
        |        |
  QHBoxLayout QVBoxLayout

# 水平布局      QHBoxLayout
# 垂直布局      QVBoxLayout
# 网格(栅格)布局 QGridLayout       
```

#### QLayout
```
// 在布局最后面添加一个窗口
void QLayout::addWidget(QWidget *w);
// 将某个窗口对象从布局中移除, 窗口对象如果不再使用需要自己析构
void QLayout::removeWidget(QWidget *widget);
// 设置布局的四个边界大小, 即: 左、上、右和下的边距。
void QLayout::setContentsMargins(int left, int top, int right, int bottom);
// 设置布局中各个窗口之间的间隙大小
void setSpacing(int);
```

#### QGridLayout
```
# 参数
- widget: 添加到布局中的窗口对象
- row: 添加到布局中的窗口对象位于第几行 (从0开始)
- column: 添加到布局中的窗口对象位于第几列 (从0开始)
- alignment: 窗口在布局中的对齐方式, 没有特殊需求使用默认值即可

void QGridLayout::addWidget(
	QWidget *widget, int row, int column, 
Qt::Alignment alignment = Qt::Alignment());

# 参数
- widget: 添加到布局中的窗口对象
- fromRow: 添加到布局中的窗口对象位于第几行 (从0开始)
- fromColumn: 添加到布局中的窗口对象位于第几列 (从0开始)
- rowSpan: 添加的窗口从 fromRow 行开始跨越的行数
- columnSpan: 添加的窗口从 fromColumn 列开始跨越的列数
- alignment: 窗口在布局中的对齐方式, 没有特殊需求使用默认值即可

void QGridLayout::addWidget(
QWidget *widget, int fromRow, int fromColumn, 
int rowSpan, int columnSpan, 
Qt::Alignment alignment = Qt::Alignment());

// 设置 column 对应的列的最新宽度, 单位: 像素
void QGridLayout::setColumnMinimumWidth(int column, int minSize);

// 设置布局中水平方向窗口之间间隔的宽度
void QGridLayout::setHorizontalSpacing(int spacing);

// 设置布局中垂直方向窗口之间间隔的宽度
void QGridLayout::setVerticalSpacing(int spacing);

```
