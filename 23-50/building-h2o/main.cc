#include <pthread.h>

#include <functional>
#include <iostream>

void releaseHydrogen() {
  std::cout << "H";
}

void releaseOxygen() {
  std::cout << "O";
}

class H2O {
public:
  H2O() {
    h = 0;
    o = 0;
    pthread_mutex_init(&(mutex), NULL);
    pthread_cond_init(&(cond_h), NULL);
    pthread_cond_init(&(cond_o), NULL);
  }

  void hydrogen(std::function<void()> releaseHydrogen) {
    pthread_mutex_lock(&mutex);
    while (h == 2) {
      pthread_cond_wait(&cond_h, &mutex);
    }
    releaseHydrogen();
    h++;
    if (h == 2) {
      o = 0;
      pthread_cond_signal(&cond_o);
    }
    pthread_mutex_unlock(&mutex);
  }

  void oxygen(std::function<void()> releaseOxygen) {
    pthread_mutex_lock(&mutex);
    while (o == 1) {
      pthread_cond_wait(&cond_o, &mutex);
    }
    releaseOxygen();
    o++;
    if (o == 1) {
      h = 0;
      pthread_cond_signal(&cond_h);
    }
    pthread_mutex_unlock(&mutex);
  }

private:
  int h;
  int o;
  pthread_mutex_t mutex;
  pthread_cond_t cond_h;
  pthread_cond_t cond_o;
};

void* threadFuncH(void* obj) {
  ((H2O*)obj)->hydrogen(releaseHydrogen);
  return nullptr;
}

void* threadFuncO(void* obj) {
  ((H2O*)obj)->oxygen(releaseOxygen);
  return nullptr;
}

int main(int argc, char ** argv) {
  H2O obj = H2O();
  pthread_t threadH1;
  pthread_create(&threadH1, NULL, threadFuncH, &obj);
  pthread_t threadH2;
  pthread_create(&threadH2, NULL, threadFuncH, &obj);
  pthread_t threadO;
  pthread_create(&threadO, NULL, threadFuncO, &obj);
  pthread_join(threadH1, NULL);
  pthread_join(threadH2, NULL);
  pthread_join(threadO, NULL);
  std::cout << std::endl;
  return 0;
}
