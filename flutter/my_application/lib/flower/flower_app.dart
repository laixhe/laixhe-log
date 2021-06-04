import 'package:flutter/material.dart';

import 'find_screen.dart';
import 'identify_screen.dart';
import 'mine_screen.dart';
import 'near_screen.dart';
import 'trend_screen.dart';

class FlowerApp extends StatefulWidget {
  const FlowerApp({Key? key}) : super(key: key);

  @override
  _FlowerAppState createState() => _FlowerAppState();
}

class _FlowerAppState extends State<FlowerApp> {

  final List<Widget> _children = [
    TrendScreen(),
    NearScreen(),
    IdentifyScreen(),
    FindScreen(),
    MineScreen()
  ];

  int _currentIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _children[_currentIndex],
      bottomNavigationBar: BottomNavigationBar(
        selectedItemColor: Colors.black,  // 选中颜色
        unselectedItemColor: Colors.grey, // 未选中颜色
        type: BottomNavigationBarType.fixed, // 未选中时的导航-显示文本和图标
        //type: BottomNavigationBarType.shifting, // 未选中时的导航-只显示图标
        selectedFontSize: 12.0,   // 选中字体大小 (默认选中的比不选中大点)
        unselectedFontSize: 12.0, // 未选中字体大小

        currentIndex: _currentIndex, // 绑定当前选中的下标
        items: [
          BottomNavigationBarItem(
            icon: Icon(Icons.filter),
            label: '动态'
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.location_on),
            label: '附近'
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.indeterminate_check_box),
            label: '识花'
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.find_in_page),
            label: '发现'
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.person),
            label: '我的'
          ),
        ],
        onTap: (int selectIndex){
          // 点击事件
          setState(() {
            _currentIndex = selectIndex;
          });
        },
      ),
    );
  }

}
