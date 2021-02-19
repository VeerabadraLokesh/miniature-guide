
#include <string>
#include <iostream>
#include <vector>
#include <atomic>
#include <fstream>

using namespace std;

int main() {

    std::vector<std::atomic_int32_t> testvector(1);


    // testvector.resize(10);

    for(int32_t i: testvector) {
        cout << i << endl;
    }

    std::vector<int32_t> pointIndexNodeIndexMap;

    int vectorSize = 10000;

    pointIndexNodeIndexMap.resize(vectorSize);

    for (int i = 0; i < pointIndexNodeIndexMap.size(); i++) {
        pointIndexNodeIndexMap[i] = i;
    }

    string targetDir = "/tmp/output";

    string pointIndexNodeIndexMapPath = targetDir + "/pointIndexMap.bin";
	
	ofstream fout(pointIndexNodeIndexMapPath, ios::out | ios::binary);
	fout.write((char*)&pointIndexNodeIndexMap[0], vectorSize * sizeof(int32_t));
	fout.close();

    return 0;
}