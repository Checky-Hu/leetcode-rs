#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

uint32_t reverseBits(uint32_t n) {
    uint32_t result = 0;
    for (int i = 0; i < 32; i++) {
        result = (result << 1) + (n & 1);
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
    printf("Reverse bits: %d\n", reverseBits(n));
    return 0;
}
