import 'package:flutter/material.dart';

// 文章
class TrendArticleScreen extends StatefulWidget {
  const TrendArticleScreen({Key? key}) : super(key: key);

  @override
  _TrendArticleScreenState createState() => _TrendArticleScreenState();
}

class _TrendArticleScreenState extends State<TrendArticleScreen> {

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Text(
        '文章',
        style: TextStyle(
            fontSize: 30
        ),
      ),
    );
  }

}
