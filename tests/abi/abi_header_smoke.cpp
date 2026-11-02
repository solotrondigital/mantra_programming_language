#include "../../include/mantra_abi.h"

int main() {
    MantraHeader header{};
    return static_cast<int>(header.flags);
}
