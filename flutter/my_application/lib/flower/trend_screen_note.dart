import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';

import 'friend.dart';
import 'trend_screen_note_img.dart';

// 花记
class TrendNoteScreen extends StatefulWidget {
  const TrendNoteScreen({Key? key}) : super(key: key);

  @override
  _TrendNoteScreenState createState() => _TrendNoteScreenState();
}

class _TrendNoteScreenState extends State<TrendNoteScreen> {

  List<Friend> _friends = [];
  var url = Uri.parse("https://randomuser.me/api/?results=30");

  @override
  void initState() {
    super.initState();
    _loadFriendsData();
  }

  _loadFriendsData() async {
    HttpClient httpClient = HttpClient();
    HttpClientRequest request = await httpClient.getUrl(url);
    HttpClientResponse response = await request.close();
    var jsonString = await response.transform(utf8.decoder).join();

    setState(() {
      _friends = Friend.resolveDataFromResponse(jsonString);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: GridView.builder(
        itemCount: _friends.length,
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: 3,
          mainAxisSpacing: 10,
          crossAxisSpacing: 10
        ),
        itemBuilder: (BuildContext context, int index){
          var friend = _friends[index];
          // 列表的每一行
          return InkWell(
            child: Card(
              child: Hero(
                tag: index,
                child: Image.network(friend.avatar),
              ),
            ),
            onTap: (){

              // 跳转到下一页面
              Navigator.push(context, MaterialPageRoute(
                  builder: (context) => TrendNoteImgScreen(imgUrl:friend.avatar, index: index)
              ));

            },
          );
        }
      ),
    );
  }

}
