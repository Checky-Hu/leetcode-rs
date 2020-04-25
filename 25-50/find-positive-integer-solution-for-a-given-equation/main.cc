#include <iostream>
#include <vector>

using namespace std;

class CustomFunction {
public:
    int f(int x, int y) {
        return x + y;
    }
};

class Solution {
public:
    vector<vector<int>> findSolution(CustomFunction& customfunction, int z) {
        vector<vector<int>> result;
        for (int x = 1; x <= 1000; x++) {
            for (int y = 1; y <= 1000; y++) {
                int t = customfunction.f(x, y);
                if (t == z) {
                    result.push_back({x, y});
                } else if (t > z) {
                    break;
                }
            }
        }
        return result;
    }
};

int main(int argc, char ** argv) {
    CustomFunction func;
    Solution* obj = new Solution();
    vector<vector<int>> result = obj->findSolution(func, 5);
    for (vector<vector<int>>::const_iterator iter = result.begin(); iter != result.end(); iter++) {
        cout << "[" << (*iter)[0] << ", " << (*iter)[1] << "]" << endl;
    }
    delete obj;
    return 0;
}
