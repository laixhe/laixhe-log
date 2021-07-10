#include <iostream>
#include <memory>

void shared(){
    std::shared_ptr<int> a(new int(1));
}