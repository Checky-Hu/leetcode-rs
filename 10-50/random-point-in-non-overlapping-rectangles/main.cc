#include <math.h>

#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(vector<vector<int>>& rects) {
        sum_ = 0;
        rects_.assign(rects.begin(), rects.end());
        vector<vector<int>>::const_iterator iter = rects.begin();
        for (; iter != rects.end(); iter++) {
            int t = (iter->at(2) - iter->at(0) + 1) * (iter->at(3) - iter->at(1) + 1);
            points_.push_back(t);
            sum_ += t;
        }
    }

    vector<int> pick() {
        int result = rand() % sum_;
        int i = 0;
        do {
            if (result >= points_[i]) {
                result -= points_[i];
                i += 1;
            } else {
                break;
            }
        } while (true);
        int x = result % (rects_[i][2] - rects_[i][0] + 1) + rects_[i][0];
        int y = result / (rects_[i][2] - rects_[i][0] + 1) + rects_[i][1];
        return {x, y};
    }

private:
    vector<vector<int>> rects_;
    vector<int> points_;
    int sum_;
};

int main(int argc, char ** argv)
{
    //vector<vector<int>> rects({{-2, -2, -1, -1}, {1, 0, 3, 0}});
    vector<vector<int>> rects({{1, 1, 5, 5}});
    Solution* solution = new Solution(rects);
    vector<int> result = solution->pick();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    result = solution->pick();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    result = solution->pick();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    result = solution->pick();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    result = solution->pick();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    delete solution;
    return 0;
}
