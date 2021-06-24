
#ifndef __C_TEST_H
#define __C_TEST_H


// 只有在 CPP 环境才有这个宏 __cplusplus

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void c_test_sum(int v1, int v2);

#ifdef __cplusplus
}
#endif // __cplusplus


#endif // __C_TEST_H
