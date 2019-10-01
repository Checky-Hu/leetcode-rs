#include <deque>
#include <iostream>
#include <vector>

using namespace std;

class Iterator {
    struct Data {
        deque<int> q;
    };
    Data* data;
public:
    Iterator(const vector<int>& nums) {
        data = new Data;
        vector<int>::const_iterator iter = nums.begin();
        for (; iter != nums.end(); iter++) {
            data->q.push_back(*iter);
        }
    }
    ~Iterator() {
        delete data;
    }
    virtual int next() {
        int v = data->q.front();
        data->q.pop_front();
        return v;
    }
    virtual bool hasNext() const { return !data->q.empty(); }
};

class PeekingIterator : public Iterator {
public:
    PeekingIterator(const vector<int>& nums) : Iterator(nums) {
        flag_ = false;
        value_ = 0;
    }

    int peek() {
        if (flag_) {
            return value_;
        } else {
            value_ = Iterator::next();
            flag_ = true;
            return value_;
        }
    }

    int next() override {
        if (flag_) {
            flag_ = false;
            return value_;
        } else {
            return Iterator::next();
        }
    }

    bool hasNext() const override {
        if (flag_) {
            return true;
        } else {
            return Iterator::hasNext();
        }
    }
private:
    bool flag_;
    int value_;
};

int main(int argc, char ** argv)
{
    const vector<int> nums{1, 2, 3};
    PeekingIterator iter(nums);
    cout << "next: " << iter.next() << endl;
    cout << "peek: " << iter.peek() << endl;
    cout << "next: " << iter.next() << endl;
    cout << "next: " << iter.next() << endl;
    cout << "hasNext: " << iter.hasNext() << endl;
}
