#include <iostream>

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  ListNode* getIntersectionNode(ListNode* headA, ListNode* headB) {
    ListNode* nodeA = headA;
    int countA = 0;
    while (nodeA != nullptr && nodeA->next != nullptr) {
      nodeA = nodeA->next;
      countA += 1;
    }
    ListNode* nodeB = headB;
    int countB = 0;
    while (nodeB != nullptr && nodeB->next != nullptr) {
      nodeB = nodeB->next;
      countB += 1;
    }
    if (nodeA != nodeB || nodeA == nullptr) {
      return nullptr;
    } else {
      nodeA = headA;
      nodeB = headB;
      if (countA >= countB) {
        for (int i = 0; i < countA - countB; i++) {
          nodeA = nodeA->next;
        }
      } else {
        for (int i = 0; i < countB - countA; i++) {
          nodeB = nodeB->next;
        }
      }
      while (nodeA != nodeB) {
        nodeA = nodeA->next;
        nodeB = nodeB->next;
      }
      return nodeA;
    }
  }
};

int main(int argc, char ** argv)
{
  ListNode* headA = new ListNode(0);
  ListNode* headB = new ListNode(1);
  ListNode* node1 = new ListNode(2);
  headA->next = node1;
  headB->next = node1;
  cout << "Intersection Node:" << Solution().getIntersectionNode(headA, headB) << endl;
  return 0;
}
