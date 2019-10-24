#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(vector<int>& nums) {
        v = nums;
    }

    vector<int> reset() {
        return v;
    }

    vector<int> shuffle() {
        vector<int> tmp = v;
        int size = v.size();
        for (int i = 0; i < size; i++) {
            int pos = i + rand() % (size - i);
            int cur = tmp[i];
            tmp[i] = tmp[pos];
            tmp[pos] = cur;
        }
        return tmp;
    }

private:
    vector<int> v;
};

void print(vector<int>& nums) {
    vector<int>::iterator iter = nums.begin();
    for (;iter != nums.end(); iter++)
        cout << *iter << " ";
    cout << endl;
}

int main(int argc, char ** argv)
{
    vector<int> nums{1, 2, 3};
    Solution* solution = new Solution(nums);
    vector<int> result = solution->shuffle();
    print(result);
    result = solution->reset();
    print(result);
    result = solution->shuffle();
    print(result);
    delete solution;
    return 0;
}
