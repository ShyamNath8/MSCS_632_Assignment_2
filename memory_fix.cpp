#include <iostream>
#include <vector>

int main() {
    for (int i = 0; i < 3; ++i) {
        std::vector<int> arr(1000000); // RAII-managed; freed at loop end
    }
    std::cout << "Finished loop with RAII\n";
    return 0;
}
