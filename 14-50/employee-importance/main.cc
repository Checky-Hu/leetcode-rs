#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Employee {
public:
    Employee(int id, int importance, vector<int> subordinates) {
        this->id = id;
        this->importance = importance;
        this->subordinates = subordinates;
    }

    int id;
    int importance;
    vector<int> subordinates;
};

class Solution {
public:
    int getImportance(vector<Employee*> employees, int id) {
        unordered_map<int, Employee*> map;
        for (vector<Employee*>::const_iterator iter = employees.begin(); iter != employees.end(); iter++) {
            map[(*iter)->id] = *iter;
        }
        vector<int> subordinatesId = {id};
        int result = 0;
        while (!subordinatesId.empty()) {
            vector<int> tmpId;
            for (vector<int>::const_iterator idIter = subordinatesId.begin();
                idIter != subordinatesId.end(); idIter++) {
                result += map[*idIter]->importance;
                for (vector<int>::const_iterator nextIdIter = map[*idIter]->subordinates.begin();
                    nextIdIter != map[*idIter]->subordinates.end(); nextIdIter++) {
                    tmpId.push_back(*nextIdIter);
                }
            }
            subordinatesId = tmpId;
        }
        return result;
    }
};

int main(int argc, char ** argv)
{
    vector<Employee*> employees;
    employees.push_back(new Employee(1, 5, {2, 3}));
    employees.push_back(new Employee(2, 3, {}));
    employees.push_back(new Employee(3, 3, {}));
    Solution solution;
    std::cout << "Importance of 1: " << solution.getImportance(employees, 1) << std::endl;
    return 0;
}
