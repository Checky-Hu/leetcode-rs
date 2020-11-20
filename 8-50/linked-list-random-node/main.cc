#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

class Solution {
 public:
  Solution(ListNode* head) {
    h = head;
  }

  int getRandom() {
    int ret = h->val;
    ListNode* cur = h->next;
    int num = 2;
    while (cur) {
      int index = random() % num;
      if (index == 0) {
        ret = cur->val;
      }
      num += 1;
      cur = cur->next;
    }
    return ret;
  }

 private:
  ListNode* h;
};

int main(int argc, char ** argv)
{
  ListNode* head = new ListNode(1);
  ListNode* current = head;
  ListNode* next = new ListNode(2);
  current->next = next;
  current = current->next;
  next = new ListNode(3);
  current->next = next;
  current = current->next;
  Solution s = Solution(head);
  cout << s.getRandom() << endl;
  cout << s.getRandom() << endl;
  cout << s.getRandom() << endl;
  return 0;
}
