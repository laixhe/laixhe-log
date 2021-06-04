import 'package:flutter/material.dart';

import 'trend_screen_note.dart';
import 'trend_screen_id.dart';
import 'trend_screen_article.dart';

// 动态
class TrendScreen extends StatefulWidget {
  const TrendScreen({Key? key}) : super(key: key);

  @override
  _TrendScreenState createState() => _TrendScreenState();
}

class _TrendScreenState extends State<TrendScreen> {

  // 获取输入框上下文
  final _textFieldController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        elevation: 0, // 去掉阴影
        //title: Text('动态'),
        title: _getSearchBar(),
        actions: <Widget>[
          IconButton(
            color: Colors.grey,
            icon: Icon(Icons.add),
            iconSize: 28,
            onPressed: (){
            }
          )
        ],
      ),
      body: DefaultTabController(
        length: 3,
        child: Column(// 列向(竖)

          children: <Widget>[
            // 上面导航
            Container(
              color: Colors.white,
              child: TabBar(
                labelStyle: TextStyle(fontSize: 14),
                labelColor: Colors.black,     // 文本颜色
                indicatorColor: Colors.black, // 线条颜色
                indicatorWeight: 1,           // 线条大小
                indicatorSize: TabBarIndicatorSize.label, // 线条长度跟随文本长度
                tabs: <Widget>[
                  Tab(text: '花记'),
                  Tab(text: '鉴定'),
                  Tab(text: '文章'),
                ],
              ),
            ),

            // 下面导航内容
            Expanded(
              flex: 1, // 填充整个父控件剩余部分
              child: TabBarView(
                children: <Widget>[
                  TrendNoteScreen(),
                  TrendIDScreen(),
                  TrendArticleScreen()
                ]
              )
            )

          ],
        )
      ),
    );
  }

  // 头部搜索框
  Widget _getSearchBar(){
    return Container(
      height: 48,
      // 设置边框
      decoration: BoxDecoration(
        // 圆角
        borderRadius: BorderRadius.circular(24),
        // 边框的颜色和宽度
        border: Border.all(color: Colors.grey, width: 2)
      ),
      child: Row(// 横向(行)

        crossAxisAlignment: CrossAxisAlignment.center,
        children: <Widget>[
          SizedBox(width: 8),
          // 放大镜
          Icon(Icons.search, color: Colors.grey),

          // 输入框
          Expanded(// 填充整个父控件剩余部分
            child: Container(
              alignment: Alignment.center,
              child: TextField(// 输入框
                controller: _textFieldController,
                decoration: InputDecoration(
                  hintText: '搜索花记/百科/鉴定/文章',
                  border: InputBorder.none, // 除掉边框
                ),
              ),
            )
          ),

          // 清除按钮
          IconButton(
            icon: Icon(
              Icons.cancel,
              color: Colors.grey,
            ),
            onPressed: (){
              // 清空输入框内容
              _textFieldController.clear();
            }
          )

        ],
      ),
    );
  }

}
