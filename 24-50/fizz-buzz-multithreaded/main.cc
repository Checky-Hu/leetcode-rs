#include <pthread.h>

#include <functional>
#include <iostream>

using namespace std;

void printFizz() {
    cout << "fizz ";
}

void printBuzz() {
    cout << "buzz ";
}

void printFizzBuzz() {
    cout << "fizzbuzz ";
}

void printNormal(int x) {
    cout << x << " ";
}

class FizzBuzz {
private:
    int n;
    int cur;
    pthread_mutex_t mutex;
    pthread_cond_t cond_fifteen;
    pthread_cond_t cond_five;
    pthread_cond_t cond_normal;
    pthread_cond_t cond_three;

public:
    FizzBuzz(int n) {
        this->n = n;
        cur = 1;
        pthread_mutex_init(&mutex, NULL);
        pthread_cond_init(&cond_fifteen, NULL);
        pthread_cond_init(&cond_five, NULL);
        pthread_cond_init(&cond_normal, NULL);
        pthread_cond_init(&cond_three, NULL);
    }

    void fizz(function<void()> printFizz) {
        while (true) {
            pthread_mutex_lock(&mutex);
            while (cur <= n && cur % 3 != 0) {
                pthread_cond_wait(&cond_three, &mutex);
            }
            if (cur > n) {
                break;
            }
            printFizz();
            cur++;
            if (cur % 5 == 0) {
                pthread_cond_signal(&cond_five);
            } else {
                pthread_cond_signal(&cond_normal);
            }
            pthread_mutex_unlock(&mutex);
        }
        pthread_cond_signal(&cond_fifteen);
        pthread_cond_signal(&cond_five);
        pthread_cond_signal(&cond_normal);
        pthread_mutex_unlock(&mutex);
    }

    void buzz(function<void()> printBuzz) {
        while (true) {
            pthread_mutex_lock(&mutex);
            while (cur <= n && cur % 5 != 0) {
                pthread_cond_wait(&cond_five, &mutex);
            }
            if (cur > n) {
                break;
            }
            printBuzz();
            cur++;
            if (cur % 3 == 0)
                pthread_cond_signal(&cond_three);
            else
                pthread_cond_signal(&cond_normal);
            pthread_mutex_unlock(&mutex);
        }
        pthread_cond_signal(&cond_fifteen);
        pthread_cond_signal(&cond_normal);
        pthread_cond_signal(&cond_three);
        pthread_mutex_unlock(&mutex);
    }

    void fizzbuzz(function<void()> printFizzBuzz) {
        while (cur <= n) {
            pthread_mutex_lock(&mutex);
            while (cur <= n && cur % 15 != 0) {
                pthread_cond_wait(&cond_fifteen, &mutex);
            }
            if (cur > n) {
                break;
            }
            printFizzBuzz();
            cur++;
            pthread_cond_signal(&cond_normal);
            pthread_mutex_unlock(&mutex);
        }
        pthread_cond_signal(&cond_five);
        pthread_cond_signal(&cond_normal);
        pthread_cond_signal(&cond_three);
        pthread_mutex_unlock(&mutex);
    }

    void number(function<void(int)> printNumber) {
        while (cur <= n) {
            pthread_mutex_lock(&mutex);
            while (cur <= n && (cur % 15 == 0 || cur % 3 == 0 || cur % 5 == 0)) {
                pthread_cond_wait(&cond_normal, &mutex);
            }
            if (cur > n) {
                break;
            }
            printNumber(cur);
            cur++;
            if (cur % 15 == 0) {
                pthread_cond_signal(&cond_fifteen);
            } else if (cur % 3 == 0) {
                pthread_cond_signal(&cond_three);
            } else if (cur % 5 == 0) {
                pthread_cond_signal(&cond_five);
            }
            pthread_mutex_unlock(&mutex);
        }
        pthread_cond_signal(&cond_fifteen);
        pthread_cond_signal(&cond_five);
        pthread_cond_signal(&cond_three);
        pthread_mutex_unlock(&mutex);
    }
};

void* threadFuncFizz(void* obj) {
    ((FizzBuzz*)obj)->fizz(printFizz);
    return nullptr;
}

void* threadFuncBuzz(void* obj) {
    ((FizzBuzz*)obj)->buzz(printBuzz);
    return nullptr;
}

void* threadFuncFizzBuzz(void* obj) {
    ((FizzBuzz*)obj)->fizzbuzz(printFizzBuzz);
    return nullptr;
}

void* threadFuncNormal(void* obj) {
    ((FizzBuzz*)obj)->number(printNormal);
    return nullptr;
}

int main(int argc, char ** argv) {
    if (argc < 2) {
        printf("At least one parameter.\n");
        return -1;
    }
    FizzBuzz* obj = new FizzBuzz(atoi(argv[1]));
    pthread_t threadFizz;
    pthread_create(&threadFizz, NULL, threadFuncFizz, obj);
    pthread_t threadBuzz;
    pthread_create(&threadBuzz, NULL, threadFuncBuzz, obj);
    pthread_t threadFizzBuzz;
    pthread_create(&threadFizzBuzz, NULL, threadFuncFizzBuzz, obj);
    pthread_t threadNormal;
    pthread_create(&threadNormal, NULL, threadFuncNormal, obj);
    pthread_join(threadFizz, NULL);
    pthread_join(threadBuzz, NULL);
    pthread_join(threadFizzBuzz, NULL);
    pthread_join(threadNormal, NULL);
    cout << endl;
    delete obj;
    return 0;
}
