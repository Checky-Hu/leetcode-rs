#include <stdio.h>
#include <stdlib.h>

int rand7()
{
    return random() % 7 + 1;
}

int rand10()
{
    int result;
    while (1) {
        result = (rand7() - 1) * 7 + rand7();
	if (result <= 40)
	    break;
    }
    return result % 10 + 1;
}

int main(int argc, char ** argv)
{
    if (argc < 2) {
        printf("This need at least one parameter.\n");
        return -1;
    }

    int n = atoi(argv[1]);
    int i = 0;
    while (i < n) {
        printf("Random: %d\n", rand10());
	i += 1;
    }
    return 0;
}
