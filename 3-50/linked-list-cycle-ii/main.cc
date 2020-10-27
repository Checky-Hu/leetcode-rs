#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  ListNode* detectCycle(ListNode* head) {
    ListNode* fast = head;
    ListNode* slow = head;
    while (fast != nullptr && fast->next != nullptr) {
      fast = fast->next->next;
      slow = slow->next;
      if (fast == slow) {
        break;
      }
    }
    if (fast == nullptr || fast->next == nullptr) {
      return nullptr;
    }
    slow = head;
    while (slow != fast) {
      fast = fast->next;
      slow = slow->next;
    }
    return slow;
  }
};

int main(int argc, char ** argv)
{
  ListNode* head = new ListNode(0);
  ListNode* next = new ListNode(1);
  next->next = head;
  head->next = next;
  cout << "Detect cycle:" << Solution().detectCycle(head) << endl;
  return 0;
}
