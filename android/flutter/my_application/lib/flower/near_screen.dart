import 'package:flutter/material.dart';
import 'near_screen_navigation_task.dart';

// 附近
class NearScreen extends StatefulWidget {
  const NearScreen({Key? key}) : super(key: key);

  @override
  _NearScreenState createState() => _NearScreenState();
}

class _NearScreenState extends State<NearScreen> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        elevation: 0, // 去掉阴影
        title: Text('附近'),
      ),
      body: Center(
        child: Column(
          children: [
            ElevatedButton(
              child: Text('导航...'),
              onPressed: (){

                // Navigator.push(context, MaterialPageRoute(
                //   builder: (context) => NearNavigationTaskScreen()
                // )).then((value) {
                //   // 接收回传数据
                //   print('value=$value');
                // });

                Navigator.of(context).push(
                  PageRouteBuilder(
                    //transitionDuration: const Duration(milliseconds: 500), // 设置动画时长500毫秒
                    pageBuilder: (BuildContext context, Animation<double> animation, Animation<double> secondaryAnimation){
                      return NearNavigationTaskScreen();
                    },
                    transitionsBuilder: (BuildContext context, Animation<double> animation, Animation<double> secondaryAnimation, Widget child){

                      //渐变过渡
                      return FadeTransition(//渐变过渡 0.0-1.0
                        opacity: Tween(begin: 0.0, end: 1.0).animate(CurvedAnimation(
                          parent: animation, //动画样式
                          curve: Curves.fastOutSlowIn, //动画曲线
                        )),
                        child: child,
                      );

                      // 翻转缩放
                      // return RotationTransition(
                      //   turns: Tween(begin: 0.0, end: 1.0).
                      //   animate(
                      //     CurvedAnimation(
                      //       parent: animation,
                      //       curve: Curves.fastOutSlowIn,
                      //     )
                      //   ),
                      //   child: ScaleTransition(
                      //     scale: Tween(begin: 0.0, end: 1.0).
                      //     animate(CurvedAnimation(parent: animation, curve: Curves.fastOutSlowIn)),
                      //     child: child,
                      //   ),
                      // );

                     // 左右滑动
                     //  return SlideTransition(
                     //    position: Tween<Offset>(
                     //      begin: Offset(-1.0, 0.0), // Offset(0.0, 1.0) 上下滑动 Offset(-1.0, 0.0) 左右滑动
                     //      end: Offset.zero
                     //    ).animate(animation),
                     //    child: child,
                     //  );

                    }
                  )
                );

              }
            )
          ],
        ),
      ),
    );
  }

}
