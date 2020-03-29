#include <unistd.h>

#include <functional>
#include <iostream>

using namespace std;

class Foo {
public:
  Foo() {
      state_ = 0;
  }

  void first(function<void()> printFirst) {
      while (state_ != 0) {
          usleep(1000);
      }
      printFirst();
      state_ = 1;
  }

  void second(function<void()> printSecond) {
      while (state_ != 1) {
          usleep(1000);
      }
      printSecond();
      state_ = 2;
  }

  void third(function<void()> printThird) {
      while (state_ != 2) {
          usleep(1000);
      }
      printThird();
      state_ = 0;
  }

private:
    int state_;
};

void print() {
    static int state = 0;
    if (state == 0) {
        cout << "first" << endl;
        state = 1;
    } else if (state == 1) {
        cout << "second" << endl;
        state = 2;
    } else {
        cout << "third" << endl;
        state = 0;
    }
}

int main(int argc, char ** argv) {
    Foo foo;
    foo.first(print);
    foo.second(print);
    foo.third(print);
    return 0;
}
