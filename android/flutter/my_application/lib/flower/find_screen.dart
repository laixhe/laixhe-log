import 'package:flutter/material.dart';

// 发现
class FindScreen extends StatefulWidget {
  const FindScreen({Key? key}) : super(key: key);

  @override
  _FindScreenState createState() => _FindScreenState();
}

class _FindScreenState extends State<FindScreen> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('发现...AppBar'),
      ),
      body: Center(
        child: Text(
          '发现',
          style: TextStyle(
            fontSize: 30
          ),
        ),
      ),
    );
  }

}
