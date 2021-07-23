
> 注意：在并发的线程数量很多，并每个线程都是执行一个时间很短的任务就结束了，这样频繁创建线程就会大大降低系统的效率，因为频繁创建线程和销毁线程需要时间。

#### QRunnable
在 Qt 中使用线程池需要先创建任务，添加到线程池中的每一个任务都需要是一个 QRunnable 类型，因此在程序中需要创建子类继承 QRunnable 这个类，然后重写 run() 方法，在这个函数中编写要在线程池中执行的任务，并将这个子类对象传递给线程池，这样任务就可以被线程池中的某个工作的线程处理掉了

```
// 在子类中必须要重写的函数, 里边是任务的处理流程
[pure virtual] void QRunnable::run();

// 参数设置为 true: 这个任务对象在线程池中的线程中处理完毕, 这个任务对象就会自动销毁
// 参数设置为 false: 这个任务对象在线程池中的线程中处理完毕, 对象需要程序猿手动销毁
void QRunnable::setAutoDelete(bool autoDelete);
// 获取当前任务对象的析构方式,返回true->自动析构, 返回false->手动析构
bool QRunnable::autoDelete() const;
```

创建一个要添加到线程池中的任务类，处理方式如下：

```
class MyWork : public QObject, public QRunnable {
    Q_OBJECT
public:
    explicit MyWork(QObject *parent = nullptr){
        // 任务执行完毕,该对象自动销毁
        setAutoDelete(true);
    }
    ~MyWork();

    void run() override{}
}
```

在上面的示例中 MyWork 类是一个多重继承，如果需要在这个任务中使用 Qt 的信号槽机制进行数据的传递就必须继承 QObject 这个类，如果不使用信号槽传递数据就可以不继承了，只继承 QRunnable 即可。

```
class MyWork :public QRunnable {
    Q_OBJECT
public:
    explicit MyWork() {
        // 任务执行完毕,该对象自动销毁
        setAutoDelete(true);
    }
    ~MyWork();

    void run() override{}
}
```

#### QThreadPool
Qt 中的 QThreadPool 类管理了一组 QThreads, 里边还维护了一个任务队列。QThreadPool 管理和回收各个 QThread 对象，以帮助减少使用线程的程序中的线程创建成本。每个Qt应用程序都有一个全局 QThreadPool 对象，可以通过调用 globalInstance() 来访问它。也可以单独创建一个 QThreadPool 对象使用。

```
// 获取和设置线程中的最大线程个数
int maxThreadCount() const;
void setMaxThreadCount(int maxThreadCount);

// 给线程池添加任务, 任务是一个 QRunnable 类型的对象
// 如果线程池中没有空闲的线程了, 任务会放到任务队列中, 等待线程处理
void QThreadPool::start(QRunnable * runnable, int priority = 0);
// 如果线程池中没有空闲的线程了, 直接返回值, 任务添加失败, 任务不会添加到任务队列中
bool QThreadPool::tryStart(QRunnable * runnable);

// 线程池中被激活的线程的个数(正在工作的线程个数)
int QThreadPool::activeThreadCount() const;

// 尝试性的将某一个任务从线程池的任务队列中删除, 如果任务已经开始执行就无法删除了
bool QThreadPool::tryTake(QRunnable *runnable);
// 将线程池中的任务队列里边没有开始处理的所有任务删除, 如果已经开始处理了就无法通过该函数删除了
void QThreadPool::clear();

// 在每个Qt应用程序中都有一个全局的线程池对象, 通过这个函数直接访问这个对象
static QThreadPool * QThreadPool::globalInstance();
```

一般情况下，我们不需要在 Qt 程序中创建线程池对象，直接使用 Qt 为每个应用程序提供的线程池全局对象即可。得到线程池对象之后，调用 start() 方法就可以将一个任务添加到线程池中，这个任务就可以被线程池内部的线程池处理掉了，使用线程池比自己创建线程的这种多种多线程方式更加简单和易于维护。

具体的使用方式如下：

```
// mywork.h
class MyWork :public QRunnable{
    Q_OBJECT
public:
    explicit MyWork();
    ~MyWork();

    void run() override;
}

// mywork.cpp
MyWork::MyWork() : QRunnable(){
    // 任务执行完毕,该对象自动销毁
    setAutoDelete(true);
}
void MyWork::run(){
    // 业务处理代码
    ......
}

// mainwindow.cpp
MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow){
    ui->setupUi(this);

    // 线程池初始化，设置最大线程池数
    QThreadPool::globalInstance()->setMaxThreadCount(4);
    // 添加任务
    MyWork* task = new MyWork;
    QThreadPool::globalInstance()->start(task);    
}
```








