#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// Example: v = {3, 5, 7,}
// t = 2 => return -1
// t = 3|5|7 => return -2
// t = 4 => return 0
// t = 6 => return 1
// t = 8 => return 2
int binarySearch(int* v, int left, int right, int t) {
    int l = left;
    int r = right;
    while (l <= r) {
        int mid = l + (r - l) / 2;
	if (v[mid] == t) {
	    return -2;
	} else if (v[mid] > t) {
	    r = mid - 1;
	} else {
	    l = mid + 1;
	}
    }
    if (r < left) {
        return -1;
    } else if (l > right) {
        return right;
    } else {
        return l;
    }
}

bool searchMatrix(int** matrix, int matrixRowSize, int matrixColSize, int target) {
    bool ret = false;
    int* v = NULL;

    if (matrixRowSize <= 0 || matrixColSize <= 0) {
        goto EXIT;
    }

    v = (int*)malloc(sizeof(int) * matrixRowSize);
    if (NULL == v) {
        goto EXIT;
    }

    for (int i = 0; i < matrixRowSize; i++) {
        v[i] = matrix[i][0];
    }
    int colIndex = binarySearch(v, 0, matrixRowSize - 1, target);
    if (colIndex == -2) {
        ret = true;
    } else if (colIndex == -1) {
        ret = false;
    } else {
        ret = false;
        for (int i = colIndex; i >= 0; i--) {
            int rowIndex = binarySearch(matrix[i], 0, matrixColSize - 1, target);
            if (rowIndex == -2) {
                ret = true;
                break;
            }
        }
    }

EXIT:
    if (v) {
        free(v);
        v = NULL;
    }
    return ret;
}

int main(int argc, char ** argv)
{
    int ret = -1;
    int target = -1;
    int row = 0;
    int column = 0;
    int** matrix = NULL;
    if (argc < 3) {
        printf("This need at least (arg1 * arg2 + 3) parameters.\n");
        goto EXIT;
    }

    target = atoi(argv[1]);
    row = atoi(argv[2]);
    column = atoi(argv[3]);
    if (row * column + 4 > argc) {
        printf("This need at least (arg1 * arg2 + 3) parameters.\n");
        goto EXIT;
    }

    matrix = (int**)malloc(sizeof(int*) * row);
    if (NULL == matrix) {
        printf("Malloc fail.\n");
        goto EXIT;
    }
    for (int i = 0; i < row; i++) {
        matrix[i] = malloc(sizeof(int) * column);
        if (NULL == matrix[i]) {
            printf("Malloc fail.\n");
            goto EXIT;
        } else {
            for (int j = 0; j < column; j++) {
                matrix[i][j] = atoi(argv[4 + i * row + j]);
            }
        }
    }

    printf("Find %d: %d\n", target, searchMatrix(matrix, row, column, target));

EXIT:
    if (matrix) {
        for (int i = 0; i < row; i++) {
	    if (matrix[i]) {
	        free(matrix[i]);
		matrix[i] = NULL;
	    } else {
	        break;
	    }
	}
	free(matrix);
	matrix = NULL;
    }
    return ret;
}
