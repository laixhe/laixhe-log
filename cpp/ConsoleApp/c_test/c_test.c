#include <stdio.h>

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
