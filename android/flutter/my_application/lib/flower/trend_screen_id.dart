import 'package:flutter/material.dart';

// 鉴定
class TrendIDScreen extends StatefulWidget {
  const TrendIDScreen({Key? key}) : super(key: key);

  @override
  _TrendIDScreenState createState() => _TrendIDScreenState();
}

class _TrendIDScreenState extends State<TrendIDScreen> {

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text(
        '鉴定',
        style: TextStyle(
            fontSize: 30
        ),
      ),
    );
  }

}
