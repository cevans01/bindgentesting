#include "foo.hpp"

int foo(float x) {
    // Pretend we actually need C++ here...
    int * var = new int();
    delete var;

    return (int)x;
}
