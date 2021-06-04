import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';

import 'friend.dart';

class FriendListPage extends StatefulWidget {
  final String title;

  FriendListPage({Key? key, required this.title}) : super(key: key);

  @override
  _FriendListPageState createState() => _FriendListPageState();
}

class _FriendListPageState extends State<FriendListPage> {

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
    return Scaffold(
        appBar: AppBar(
          title: Text(widget.title),
        ),
        body: _buildBody()
    );
  }

  Widget _buildBody(){
    var content;

    if (_friends.isEmpty) {
      content = Center(
        child: CircularProgressIndicator(),
      );
    } else {
      content = ListView.builder(
          itemCount: _friends.length,
          itemBuilder: _buildItem
      );
    }

    return content;
  }

  Widget _buildItem(BuildContext context, int index){
    var friend = _friends[index];

    return ListTile(
      leading: CircleAvatar(
        backgroundImage: NetworkImage(friend.avatar),
      ),
      title: Text(friend.name),
      subtitle: Text(friend.email),
    );
  }
}
