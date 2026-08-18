import std;

int main() {
    std::println("{}", 42);
}

// g++ -std=c++23 -fmodules --compile-std-module main.cpp -o app.exe