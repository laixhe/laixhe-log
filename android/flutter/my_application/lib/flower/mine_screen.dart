import 'package:flutter/material.dart';

// 我的
class MineScreen extends StatefulWidget {
  const MineScreen({Key? key}) : super(key: key);

  @override
  _MineScreenState createState() => _MineScreenState();
}

class _MineScreenState extends State<MineScreen> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('我的...AppBar'),
      ),
      body: Center(
        child: Text(
          '我的',
          style: TextStyle(
              fontSize: 30
          ),
        ),
      ),
    );
  }

}
