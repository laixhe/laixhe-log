#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void func_var() {

	printf("int = %d\n", sizeof(int));
	printf("double = %d\n", sizeof(double));
	printf("float = %d\n", sizeof(float));
	printf("char = %d\n", sizeof(char));
	printf("long = %d\n", sizeof(long));
	printf("short = %d\n", sizeof(short));

	int i = 200;
	printf("i的数值是：%d\n", i);
	printf("i的内存地址是：%p\n", &i);
	printf("i的内存地址里面的数据：%d\n", *(&i));

	int* d = &i;
	printf("d指针对应的数值是：%d\n", *d);
	printf("d指针对应的地址是：%p\n", &d);
	printf("d指针存放的内容是：%p\n", d);

}

void malloc_free(){

	// 获取 int 要占用的内存大小
	int sizeInt = sizeof(int);
	// 创建一个 sizeInt 字节大小的堆空间
	int *p = (int *) malloc(sizeInt);
	// 初始化内存( 从 p 的首地址开始，连续 sizeInt 个字节的每个字节都为：0)
	memset(p, 0, sizeInt);
	// 释放内存
	free(p);

}
