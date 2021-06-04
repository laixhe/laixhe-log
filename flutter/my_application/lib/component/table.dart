import 'package:flutter/material.dart';

class TableTask extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Table 的使用'),
      ),
      body: Center(
          child: Table(
          border: TableBorder.all(
            color:Colors.green,
            width: 2,
            style: BorderStyle.solid
          ),
          columnWidths: {
            0: FixedColumnWidth(100.0),
            1: FixedColumnWidth(200.0),
            2: FixedColumnWidth(50),
          },
          children: [
            TableRow(
              children: [
                Text('姓名'),
                Text('性别'),
                Text('年龄'),
              ]
            ),
            TableRow(
              children: [
                Text('张三'),
                Text('男'),
                Text('21'),
              ]
            ),
            TableRow(
              children: [
                Text('李四'),
                Text('女'),
                Text('19'),
              ]
            ),
          ],
        )
      )

    );
  }
}
