

#include <iostream>


using namespace std;

int main() {

    int64_t x = 1, y = 1, z = 1;

    for (int64_t j = 0; j < 8; j++) {
        int64_t ox = (j & 0b100) >> 2;
        int64_t oy = (j & 0b010) >> 1;
        int64_t oz = (j & 0b001) >> 0;

        int64_t nx = 2 * x + ox;
        int64_t ny = 2 * y + oy;
        int64_t nz = 2 * z + oz;

        // cout << ox << oy << oz << endl;
        cout << nx << ny << nz << endl;
    }

    int k = 1'000;
    cout << k << endl;

    return 0;
}