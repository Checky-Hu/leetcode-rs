#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  void deleteNode(ListNode* node) {
    ListNode* prefix = node;
    while (node->next != nullptr) {
      node->val = node->next->val;
      prefix = node;
      node = node->next;
    }
    prefix->next = nullptr;
  }
};

int main(int argc, char ** argv)
{
  ListNode* head = new ListNode(4);
  ListNode* current = head;
  ListNode* next = new ListNode(5);
  ListNode* target = next;
  current->next = next;
  current = current->next;
  next = new ListNode(1);
  current->next = next;
  current = current->next;
  next = new ListNode(9);
  current->next = next;
  current = current->next;
  Solution().deleteNode(target);
  for (current = head; current != nullptr; current = current->next) {
    cout << current->val << endl;
  }
  return 0;
}
