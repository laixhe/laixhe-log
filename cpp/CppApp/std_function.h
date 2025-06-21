#ifndef STD_FUNCTION_H
#define STD_FUNCTION_H

#include <iostream>
#include <functional>
#include <initializer_list>

extern inline void func_inline();
extern void func_lambda();
extern void func_lambda_recursion();
extern void func_initializer_list(std::initializer_list<int> list);

#endif // STD_FUNCTION_H

