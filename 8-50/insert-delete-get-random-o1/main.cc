#include <iostream>
#include <unordered_map>
#include <vector>

class RandomizedSet {
public:
    RandomizedSet() {
    }

    bool insert(int val) {
        if (map.count(val) != 0) return false;
        nums.push_back(val);
        map[val] = nums.size() - 1;
        return true;
    }

    bool remove(int val) {
        if (map.count(val) == 0) return false;
        int last = nums.back();
        map[last] = map[val];
        nums[map[val]] = last;
        nums.pop_back();
        map.erase(val);
        return true;
    }

    int getRandom() {
        return nums[rand() % nums.size()];
    }
private:
    std::vector<int> nums;
    std::unordered_map<int, int> map;
};

int main(int argc, char ** argv)
{
    RandomizedSet* obj = new RandomizedSet();
    std::cout << "Insert 1: " << obj->insert(1) << std::endl;
    std::cout << "Remove 2: " << obj->remove(2) << std::endl;
    std::cout << "Insert 2: " << obj->insert(2) << std::endl;
    std::cout << "Get random: " << obj->getRandom() << std::endl;
    std::cout << "Remove 1: " << obj->remove(1) << std::endl;
    std::cout << "Insert 2: " << obj->insert(2) << std::endl;
    std::cout << "Get random: " << obj->getRandom() << std::endl;
    delete obj;
    return 0;
}
