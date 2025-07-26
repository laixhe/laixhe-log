#include <iostream>

#include "std_chrono.h"

int main(){

    auto stdChrono = new StdChrono();
    stdChrono->Timestamp();
    stdChrono->Format();
    stdChrono->Compute();
    delete stdChrono;

    return 0;
}
