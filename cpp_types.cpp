#include <iostream>
#include <functional>

int main() {
    // static typing: types fixed at compile time
    auto adder_int = [](int x, int y) { return x + y; };
    std::cout << adder_int(2, 3) << "\n";

    // lambda capturing by value
    int count = 0;
    auto inc_by_value = [count]() mutable { // captured by value; mutable allows modification of the copy
        count += 1;
        return count;
    };
    std::cout << inc_by_value() << "\n"; // 1
    std::cout << inc_by_value() << "\n"; // 2

    // capture by reference
    int ref_count = 0;
    auto inc_by_ref = [&ref_count]() {
        ref_count += 1;
        return ref_count;
    };
    std::cout << inc_by_ref() << "\n"; // 1
    std::cout << ref_count << "\n";    // 1 (external variable changed)
}
