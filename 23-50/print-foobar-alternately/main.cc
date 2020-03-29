#include <unistd.h>

#include <functional>
#include <iostream>

using namespace std;

class FooBar {
private:
    int n;
    int state;

public:
  FooBar(int n) {
      this->n = n;
      this->state = 0;
  }

  void foo(function<void()> printFoo) {
      for (int i = 0; i < n; i++) {
          while (state != 0) {
              usleep(1000);
          }
          printFoo();
          state = 1;
      }
  }

  void bar(function<void()> printBar) {
      for (int i = 0; i < n; i++) {
          while (state != 1) {
              usleep(1000);
          }
          printBar();
          state = 1;
      }
  }
};

void printFoo() {
    cout << "foo" << endl;
}

void printBar() {
    cout << "bar" << endl;
}

int main(int argc, char ** argv) {
    FooBar fooBar(1);
    fooBar.foo(printFoo);
    fooBar.bar(printBar);
    return 0;
}
