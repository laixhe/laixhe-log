import 'package:flutter/material.dart';

// 导航使用
class NearNavigationTaskScreen extends StatefulWidget {
  const NearNavigationTaskScreen({Key? key}) : super(key: key);

  @override
  _NearNavigationTaskScreenState createState() => _NearNavigationTaskScreenState();
}

class _NearNavigationTaskScreenState extends State<NearNavigationTaskScreen> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        elevation: 0, // 去掉阴影
        title: Text('导航使用'),
      ),
      body: Center(
        child: ElevatedButton(
          child: Text('返回'),
          onPressed: (){
            Navigator.of(context).pop('导航使用-返回-数据');
          }
        ),
      ),
    );
  }

}
