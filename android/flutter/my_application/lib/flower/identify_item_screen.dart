import 'package:flutter/material.dart';

import 'identify_company.dart';

class IdentifyItemScreen extends StatelessWidget {

  final IdentifyCompany companyModel;

  IdentifyItemScreen(this.companyModel);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.symmetric(
        horizontal: 5, // 水平
        vertical: 3    // 垂直
      ),
      child: Card(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start, // 交叉轴对齐(横轴对齐\水平方向对齐)
          children: <Widget>[

            // 公司信息
            Row(
              children: <Widget>[
                // 公司 logo
                Padding(
                  padding: EdgeInsets.only(
                    top: 10,
                    left: 15,
                    right: 15,
                    bottom: 0
                  ),
                  child: Image.network(
                    companyModel.logo,
                    width: 50,
                    height: 50,
                  ),
                ),
                // 公司地址
                Padding(
                  padding: EdgeInsets.only(
                    top: 10,
                    left: 15,
                    right: 15,
                    bottom: 0
                  ),
                  child: Text(
                    // 截取
                    companyModel.location.substring(
                        0,
                        companyModel.location.length > 6 ? 6 : companyModel.location.length),
                    style: TextStyle(fontSize: 13, color: Colors.grey),
                    textAlign: TextAlign.center,
                  ),
                ),
                // 公司基本信息
                Padding(
                  padding: EdgeInsets.only(
                      top: 5,
                      left: 5,
                      right: 10,
                      bottom: 5
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.start,
                    children: <Widget>[
                      Text(
                        "|" + companyModel.type,
                        style: TextStyle(fontSize: 13, color: Colors.grey),
                        overflow: TextOverflow.ellipsis, // 文本溢出效果
                        textAlign: TextAlign.center,
                      ),
                      Text(
                        "|" + companyModel.size,
                        style: TextStyle(fontSize: 13, color: Colors.grey),
                        overflow: TextOverflow.ellipsis,
                        textAlign: TextAlign.center,
                      ),
                      Text(
                        "|" + companyModel.employee,
                        style: TextStyle(fontSize: 13, color: Colors.grey),
                        overflow: TextOverflow.ellipsis,
                        textAlign: TextAlign.center,
                      )
                    ],
                  ),
                )
              ],
            ),

            // 分隔线
            Divider(),

            // 热招 和 箭头
            Row(
              children: <Widget>[
                // 热招
                Padding(
                  padding: EdgeInsets.only(
                    top: 5,
                    left: 10,
                    right: 5,
                    bottom: 15
                  ),
                  child: Text(
                    "热招：${companyModel.hot} 等${companyModel.count}个职位",
                    style: TextStyle(fontSize: 13, color: Colors.grey)
                  ),
                ),
                // 箭头
                Expanded( // 大小填充整个父控件剩余部分
                  child: Column( // 主要作用是对齐
                    crossAxisAlignment: CrossAxisAlignment.end,
                    children: <Widget>[
                      Padding(
                        padding: EdgeInsets.only(bottom: 8),
                        child: Icon(Icons.keyboard_arrow_right, color: Colors.grey),
                      )
                    ],
                  )
                )
              ],
            )

          ],
        ),
      ),
    );
  }

}