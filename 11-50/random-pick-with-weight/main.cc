#include <math.h>

#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(vector<int>& w) {
        sum_ = w;
        len_ = w.size();
        for (int i = 1; i < len_; i++) {
            sum_[i] = sum_[i - 1] + w[i];
        }
    }

    int pickIndex() {
        int target = rand() % sum_.back();
        int left = 0;
        int right = len_ - 1;
        while (left < right) {
            int mid = (right - left) / 2 + left;
            if (sum_[mid] <= target) left = mid + 1;
            else right = mid;
        }
	return right;
    }

private:
    vector<int> sum_;
    int len_;
};

int main(int argc, char ** argv)
{
    vector<int> w = {1, 3, 2};
    Solution* solution = new Solution(w);
    int result = solution->pickIndex();
    cout << "pick: " << result << endl;
    result = solution->pickIndex();
    cout << "pick: " << result << endl;
    result = solution->pickIndex();
    cout << "pick: " << result << endl;
    result = solution->pickIndex();
    cout << "pick: " << result << endl;
    result = solution->pickIndex();
    cout << "pick: " << result << endl;
    result = solution->pickIndex();
    cout << "pick: " << result << endl;
    delete solution;
    return 0;
}
