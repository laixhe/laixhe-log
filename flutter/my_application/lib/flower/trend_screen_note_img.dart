import 'package:flutter/material.dart';

class TrendNoteImgScreen extends StatelessWidget {
  const TrendNoteImgScreen({Key? key, required this.imgUrl, required this.index}) : super(key: key);

  final String imgUrl;
  final int index;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: InkWell(
        child: Container(
          constraints: BoxConstraints.expand(), // 全屏平铺
          child: Hero(
            tag: index,
            child: Image.network(imgUrl, fit: BoxFit.fitWidth),
          ),
        ),
        onTap: (){
          // 返回到上一个页面
          Navigator.of(context).pop();
        },
      ),
    );
  }

}
