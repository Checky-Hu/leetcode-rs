#include <stdio.h>
#include <stdlib.h>

struct Node {
    int val;
    int numChildren;
    struct Node** children;
};

void postorderLoop(int* result, int* index, struct Node* node) {
    if (node == NULL) {
        return;
    }
    for (int i = 0; i < node->numChildren; i++) {
        postorderLoop(result, index, node->children[i]);
    }
    result[*index] = node->val;
    *index += 1;
}

int* postorder(struct Node* root, int* returnSize) {
    int* result = malloc(sizeof(int) * 10000);
    *returnSize = 0;
    postorderLoop(result, returnSize, root);
    return result;
}

int main(int argc, char ** argv)
{
    if (argc < 2) {
        printf("This need at least 1 parameter.\n");
        return -1;
    }

    struct Node node;
    node.val = atoi(argv[1]);
    node.numChildren = 0;
    node.children = NULL;
    int returnSize = 0;
    int* result = postorder(&node, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", result[i]);
    }
    printf("\n");
    free(result);
    return 0;
}
