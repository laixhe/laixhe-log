import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:fluttertoast/fluttertoast.dart';

//void main() {
//  runApp(GetMaterialApp(
//    home: MyHome(title: 'MyApp'),
//  ));
//}

// 定义动态数据
class Controller extends GetxController {
  var count = 0.obs;

  increment() => count++;
}

class MyHome extends StatelessWidget {
  const MyHome({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {

    final Controller c = Get.put(Controller());

    return Scaffold(
      appBar: AppBar(
        title: Text("首页..."),
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          ElevatedButton(
            child: Text('Toast'),
            onPressed: (){
              Fluttertoast.showToast(msg: 'msg....');
            },
          ),
          ElevatedButton(
            child: Text('bottomSheet'),
            onPressed: (){
              Get.bottomSheet(Container(
                height: 200,
                color: Colors.white,
                child: ListView(
                  children: [
                    ListTile(
                      leading: Icon(Icons.account_box, size: 35, color: Colors.red),
                      title: Text('重启'),
                      trailing: Icon(Icons.keyboard_arrow_right, color: Colors.black),
                    ),
                    ListTile(
                      leading: Icon(Icons.account_box, size: 35, color: Colors.red),
                      title: Text('注销'),
                      trailing: Icon(Icons.keyboard_arrow_right, color: Colors.black),
                    ),
                    ListTile(
                      leading: Icon(Icons.account_box, size: 35, color: Colors.red),
                      title: Text('关机'),
                      trailing: Icon(Icons.keyboard_arrow_right, color: Colors.black),
                    ),
                  ],
                ),
              ));
            },
          ),
          ElevatedButton(
            child: Text('dialog'),
            onPressed: (){
              Get.defaultDialog(
                title: '',
                middleText: '内容....',
                textCancel: '取消',
                textConfirm: '确认'
              );
            },
          ),
          ElevatedButton(
            child: Text('snackbar'),
            onPressed: (){
              Get.snackbar('提示', '内容...', 
                colorText: Colors.white,
                backgroundColor: Colors.black54,
                duration: Duration(seconds: 1),
                snackPosition: SnackPosition.BOTTOM,
              );
            }
          ),
          GestureDetector(
            child: Container(
              height: 200,
              width: 200,
              color: Colors.red,
              child: Align(
                alignment: Alignment.bottomCenter,
                child: Obx(()=>Text(
                  c.count.toString(),
                  style: TextStyle(fontSize: 50, color: Colors.white),
                )),
              ),
            ),
            onTap: (){
              // 跳转到下一页面
              Get.to(Other());
            },
          )
        ],
      ),
      floatingActionButton: FloatingActionButton(
        child: Icon(Icons.add),
        onPressed: (){
          c.increment();
        },
      ),
    );
  }

}

class Other extends StatelessWidget {
  // 你可以让Get找到一个正在被其他页面使用的Controller，并将它返回给你。
  final Controller c = Get.find();

  @override
  Widget build(context){
    // 访问更新后的计数变量
    return Scaffold(
      appBar: AppBar(
        title: Text('Other'),
        leading: GestureDetector(
          child: Icon(Icons.arrow_back),
          onTap: (){
            // 返回上一页面
            Get.back();
          },
        ),
      ),
      body: Center(
        child: Text("${c.count}"), // 访问更新后的计数变量
      )
    );
  }
}
