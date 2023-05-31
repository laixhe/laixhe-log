#include <iostream>

#include "class_singleton.h"

int main(){

    Singleton& mySingleton = getSingletonInstance();
    mySingleton.doSomething();

    return 0;
}
