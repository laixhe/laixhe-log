import 'package:flutter/material.dart';

import 'package:flutter_swiper_null_safety/flutter_swiper_null_safety.dart';

const double _kAppBarHeight = 256;

class IdentifyCompanyDetailScreen extends StatefulWidget {
  const IdentifyCompanyDetailScreen({Key? key}) : super(key: key);

  @override
  _IdentifyCompanyDetailScreenState createState() => _IdentifyCompanyDetailScreenState();
}

class _IdentifyCompanyDetailScreenState extends State<IdentifyCompanyDetailScreen> with TickerProviderStateMixin {

  List<Image> imgs = [
    //建立了一个图片数组
    Image.network(
      "https://images.unsplash.com/photo-1477346611705-65d1883cee1e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=500&q=60",
      fit: BoxFit.cover,
    ),
    Image.network(
      "https://images.unsplash.com/photo-1498550744921-75f79806b8a7?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=1050&q=80",
      fit: BoxFit.cover,
    ),
    Image.network(
      "https://images.unsplash.com/photo-1451187580459-43490279c0fa?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=500&q=60",
      fit: BoxFit.cover,
    ),
  ];

  // late 关键字实现延迟初始化
  late TabController _tabController;
  List<Tab> _tabs = [
    new Tab(text: "公司概况"),
    new Tab(text: "热招职位")
  ];
  int _tabIndex = 0;
  List<Widget> _tabIndexContent = [
    Text('公司概况...'),
    Text('热招职位...')
  ];

  @override
  void initState() {
    super.initState();

    _tabController = new TabController(length: _tabs.length, vsync: this);
    // 注册侦听事件
    _tabController.addListener(() {
      setState(() {
        _tabIndex = _tabController.index;
      });
    });

  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: <Widget>[
          SingleChildScrollView( // 用于固定布局的数据的滚动视图
            child: Column(
              children: <Widget>[
                // 轮播图
                SizedBox.fromSize(
                  size: Size.fromHeight(_kAppBarHeight),
                  child: Swiper(
                    itemBuilder: (BuildContext context, int index) {
                      // 条目构建函数传入了index,根据index索引到特定图片
                      return imgs[index];
                    },
                    itemCount: imgs.length,
                    autoplay: true,
                    pagination: SwiperPagination(), // 这些都是控件默认写好的,直接用
                    //control: SwiperControl(),
                  ),
                ),

                // 公司简介 tab
                Container(
                  color: Colors.white,
                  child: Column(
                    children: <Widget>[
                      TabBar(
                        tabs: _tabs,
                        controller: _tabController,
                        indicatorWeight: 3.0,
                        labelStyle: new TextStyle(fontSize:16.0),
                        labelColor: Colors.black,
                      )
                    ],
                  ),
                ),

                // 公司简介 tab 内容
                _tabIndexContent[_tabIndex]

              ],
            ),
          ),

          // 返回按钮
          Positioned(
            top: 25,
            left: 10,
            child: BackButton(color: Colors.white)
          )

        ],
      ),
    );
  }

}
