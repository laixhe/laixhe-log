import 'dart:convert';

class IdentifyCompany {
  final String logo;
  final String name;
  final String location;
  final String type;
  final String size;
  final String employee;
  final String hot;
  final String count;
  final String inc;

  IdentifyCompany({
    required this.logo,
    required this.name,
    required this.location,
    required this.type,
    required this.size,
    required this.employee,
    required this.hot,
    required this.count,
    required this.inc,
  });

  static List<IdentifyCompany> fromJson(String json){
    List<IdentifyCompany> listMode = [];
    List list = jsonDecode(json)['list'];
    list.forEach((v) {
      var model = IdentifyCompany.fromMap(v);
      listMode.add(model);
    });
    return listMode;
  }

  static IdentifyCompany fromMap(Map map){
    return IdentifyCompany(
        logo: map['logo'],
        name: map['name'],
        location: map['location'],
        type: map['type'],
        size: map['size'],
        employee: map['employee'],
        hot: map['hot'],
        count: map['count'],
        inc: map['inc']
    );
  }

}