#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  bool hasCycle(ListNode* head) {
    while (head != nullptr) {
      if (head->val == 100001) {
        return true;
      } else {
        head->val = 100001;
        head = head->next;
      }
    }
    return false;
  }
};

int main(int argc, char ** argv)
{
  ListNode* head = new ListNode(0);
  ListNode* next = new ListNode(1);
  next->next = head;
  head->next = next;
  cout << "Has cycle:" << Solution().hasCycle(head) << endl;
  return 0;
}
