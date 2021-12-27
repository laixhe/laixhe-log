import 'dart:async';

import 'package:flutter/material.dart';

import 'package:shared_preferences/shared_preferences.dart';

import '../flower/flower_app.dart';

class WelcomePage extends StatefulWidget {
  const WelcomePage({Key? key}) : super(key: key);

  @override
  _WelcomePageState createState() => _WelcomePageState();
}

class _WelcomePageState extends State<WelcomePage> {

  // late 关键字实现延迟初始化
  // 定时器
  late Timer _timer;

  // 页面启动时调用
  @override
  void initState() {
    super.initState();

    // 定时器
    _timer = Timer(Duration(seconds: 2), (){

      // 跳转面页并移除当前页面
      Navigator.of(context).pushAndRemoveUntil(
        MaterialPageRoute(builder: (BuildContext context){
          return FlowerApp();
        }), (route) => route == null);

    });
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text('启动页'),
    );
  }

  // 页面销毁时调用
  @override
  void dispose() {
    // 关闭定时器
    _timer.cancel();
    super.dispose();
  }

  // 简单持久化数据
  void saveSomething() async {
    final preference = await SharedPreferences.getInstance();
    preference.setString('localKey', 'LocalData');
  }

  // 简单持久化数据
  void getSomething() async {
    final preference = await SharedPreferences.getInstance();
    var result = preference.getString('localKey');
    print('T--------- $result');
  }
}
