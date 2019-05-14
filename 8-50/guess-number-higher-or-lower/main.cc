#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static int target = -1;
int guess(int num)
{
    if (num < target) {
        return 1;
    } else if (num == target) {
        return 0;
    } else {
        return -1;
    }
}

class Solution {
public:
    int guessNumber(int n) {
        int start = 1, end = n;
        while (start <= end) {
            int mid = start + (end - start) / 2;
	    int res = guess(mid);
	    if (res == 0) {
	        return mid;
	    } else if (res == -1) {
	        end = mid - 1;
	    } else {
	        start = mid + 1;
	    }
	}
	return -1;
    }
};

int main(int argc, char ** argv)
{
    if (argc < 3) {
        printf("This need at least two parameters.\n");
        return -1;
    }

    target = atoi(argv[2]);
    int n = atoi(argv[1]);
    printf("Target: %d\n", Solution().guessNumber(n));
    return 0;
}
