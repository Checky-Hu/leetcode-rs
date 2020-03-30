#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

void printNumber(int x) {
    printf("%d", x);
}

typedef struct {
    int n;
    int state;
    int curN;
    pthread_mutex_t mutex;
    pthread_cond_t cond_zero;
    pthread_cond_t cond_odd;
    pthread_cond_t cond_even;
} ZeroEvenOdd;

ZeroEvenOdd* ZeroEvenOddCreate(int n) {
    ZeroEvenOdd* obj = (ZeroEvenOdd*)malloc(sizeof(ZeroEvenOdd));
    obj->n = n;
    obj->state = 0;
    obj->curN = 0;
    pthread_mutex_init(&(obj->mutex), NULL);
    pthread_cond_init(&(obj->cond_zero), NULL);
    pthread_cond_init(&(obj->cond_odd), NULL);
    pthread_cond_init(&(obj->cond_even), NULL);
    return obj;
}

void zero(ZeroEvenOdd* obj) {
    for (int i = 0; i < obj->n; i++) {
        pthread_mutex_lock(&(obj->mutex));
        while (obj->state != 0) {
            pthread_cond_wait(&(obj->cond_zero), &(obj->mutex));
        }
        printNumber(0);
        obj->state = 1;
        obj->curN++;
        if ((obj->curN & 1) == 1)
            pthread_cond_signal(&(obj->cond_odd));
        else
            pthread_cond_signal(&(obj->cond_even));
        pthread_mutex_unlock(&(obj->mutex));
    }
}

void even(ZeroEvenOdd* obj) {
    for (int i = 2; i <= obj->n; i += 2) {
        pthread_mutex_lock(&(obj->mutex));
        while (obj->state != 1 || obj->curN != i) {
            pthread_cond_wait(&(obj->cond_even), &(obj->mutex));
        }
        printNumber(obj->curN);
        obj->state = 0;
        pthread_cond_signal(&(obj->cond_zero));
        pthread_mutex_unlock(&(obj->mutex));
    }
}

void odd(ZeroEvenOdd* obj) {
    for (int i = 1; i <= obj->n; i += 2) {
        pthread_mutex_lock(&(obj->mutex));
        while (obj->state != 1 || obj->curN != i) {
            pthread_cond_wait(&(obj->cond_odd), &(obj->mutex));
        }
        printNumber(obj->curN);
        obj->state = 0;
        pthread_cond_signal(&(obj->cond_zero));
        pthread_mutex_unlock(&(obj->mutex));
    }
}

void ZeroEvenOddFree(ZeroEvenOdd* obj) {
    free(obj);
}

void* threadFuncZero(void* obj) {
    zero((ZeroEvenOdd*)obj);
    return nullptr;
}

void* threadFuncEven(void* obj) {
    even((ZeroEvenOdd*)obj);
    return nullptr;
}

void* threadFuncOdd(void* obj) {
    odd((ZeroEvenOdd*)obj);
    return nullptr;
}

int main(int argc, char ** argv) {
    if (argc < 2) {
        printf("At least one parameter.\n");
        return -1;
    }
    ZeroEvenOdd* obj = ZeroEvenOddCreate(atoi(argv[1]));
    pthread_t threadZero;
    pthread_create(&threadZero, NULL, threadFuncZero, obj);
    pthread_t threadEven;
    pthread_create(&threadEven, NULL, threadFuncEven, obj);
    pthread_t threadOdd;
    pthread_create(&threadOdd, NULL, threadFuncOdd, obj);
    pthread_join(threadZero, NULL);
    pthread_join(threadEven, NULL);
    pthread_join(threadOdd, NULL);
    printf("\n");
    ZeroEvenOddFree(obj);
    return 0;
}
