#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(vector<int>& nums) {
        v = nums;
    }

    int pick(int target) {
        int result = -1;
        int count = 0;
        for (unsigned int i = 0; i < v.size(); i++) {
            if (v[i] != target)
                continue;
            count++;
            if (rand() % count == 0) {
                result = i;
            }
        }
        return result;
    }

private:
    vector<int> v;
};

int main(int argc, char ** argv)
{
    vector<int> nums{1, 2, 3, 3, 3};
    Solution* solution = new Solution(nums);
    cout << "Pick 3, index: " << solution->pick(3) << endl;
    cout << "Pick 1, index: " << solution->pick(1) << endl;
    cout << "Pick 3, index: " << solution->pick(3) << endl;
    delete solution;
    return 0;
}
