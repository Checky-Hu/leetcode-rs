#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

static int bad_version = -1;
bool isBadVersion(int version)
{
    return version >= bad_version;
}

int firstBadVersion(int n)
{
    int start = 1, end = n;
    int last_bad_version = -1;
    while (start <= end) {
        int mid = start + (end - start) / 2;
	if (isBadVersion(mid)) {
	    last_bad_version = mid;
	    end = mid - 1;
	} else {
	    start = mid + 1;
	}
    }
    return last_bad_version;
}

int main(int argc, char ** argv)
{
    if (argc < 3) {
        printf("This need at least two parameters.\n");
        return -1;
    }

    bad_version = atoi(argv[2]);
    int n = atoi(argv[1]);
    printf("Bad version: %d\n", firstBadVersion(n));
    return 0;
}
