import 'dart:convert';

class Friend {
  final String avatar;
  final String name;
  final String email;

  Friend({required this.avatar, required this.name, required this.email});

  static List<Friend> resolveDataFromResponse(String responseData){
    var jsonData = jsonDecode(responseData);
    var results = jsonData['results'];
    var result = results
        .map((obj)=> Friend.fromMap(obj))
        .toList()
        .cast<Friend>();

    return result;
  }

  static Friend fromMap(Map map){
    var name = map['name'];
    return Friend(
      avatar: map['picture']['large'],
      name: '${name['first']} ${name['last']}',
      email: map['email']
    );
  }
}