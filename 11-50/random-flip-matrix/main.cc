#include <math.h>

#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(int n_rows, int n_cols) {
        rows_ = n_rows;
        cols_ = n_cols;
        size_ = rows_ * cols_;
    }

    vector<int> flip() {
        int id = rand() % size_;
        int val = id;
        size_--;
        if (set_.count(id))
            id = set_[id];
        set_[val] = set_.count(size_) ? set_[size_]: size_;
        return {id / cols_, id % cols_};
    }

    void reset() {
        set_.clear();
        size_ = rows_ * cols_;
    }

private:
    unordered_map<int, int> set_;
    int rows_, cols_;
    int size_;
};

int main(int argc, char ** argv)
{
    Solution* solution = new Solution(1, 2);
    vector<int> result = solution->flip();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    result = solution->flip();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    delete solution;
    return 0;
}
