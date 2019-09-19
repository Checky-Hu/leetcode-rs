#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int hammingWeight(uint32_t n) {
    int result = 0;
    while (n > 0) {
        result += n & 1;
	n >>= 1;
    }
    return result;
}

int main(int argc, char ** argv)
{
    if (argc < 2) {
        printf("This need at least one parameter.\n");
        return -1;
    }

    long n = atol(argv[1]);
    printf("Number of 1 bits: %d\n", hammingWeight(n));
    return 0;
}
