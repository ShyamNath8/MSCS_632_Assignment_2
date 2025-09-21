#include <iostream>

int main() {
    for (int i = 0; i < 3; ++i) {
        int* arr = new int[1000000]; // allocate
        // forgot to delete[] arr;  // memory leak
    }
    std::cout << "Finished loop with leak\n";
    return 0;
}
