#include <math.h>

#include <iostream>
#include <vector>

using namespace std;

class Solution {
public:
    Solution(double radius, double x_center, double y_center) {
        r = radius, x = x_center, y = y_center;
    }

    vector<double> randPoint() {
        double theta = 2 * M_PI * ((double)rand() / RAND_MAX);
        double len = sqrt((double)rand() / RAND_MAX) * r;
        return {x + len * cos(theta), y + len * sin(theta)};
    }

private:
    double r, x, y;
};

int main(int argc, char ** argv)
{
    Solution* solution = new Solution(1, 0, 0);
    vector<double> result = solution->randPoint();
    cout << "x = " << result[0] << ", y = " << result[1] << endl;
    delete solution;
    return 0;
}
