#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Node {
 public:
  int val;
  Node* next;
  Node* random;

  Node(int _val) {
    val = _val;
    next = NULL;
    random = NULL;
  }
};

class Solution {
 public:
  // <source, target>
  unordered_map<Node*, Node*> map;
  // <source random, targets that point to this random>
  unordered_map<Node*, vector<Node*>> needs;
  Node* copyRandomList(Node* head) {
    Node* result = nullptr;
    Node** current = &result;
    while (head != nullptr) {
      *current = new Node(head->val);
      map.emplace(head, *current);
      auto need = needs.find(head);
      if (need != needs.end()) {
        for (Node* node : need->second) {
          node->random = *current;
        }
      }
      auto random = map.find(head->random);
      if (random == map.end()) {
        need = needs.find(head->random);
        if (need == needs.end()) {
          needs.emplace(head->random, vector<Node*>{*current});
        } else {
          need->second.push_back(*current);
        }
      } else {
        (*current)->random = random->second;
      }
      current = &((*current)->next);
      head = head->next;
    }
    return result;
  }
};

void printNode(Node* node) {
  while (node != nullptr) {
    cout << "val=" << node->val << ", random=" << node->random << endl;
    node = node->next;
  }
}

int main(int argc, char ** argv)
{
  Node* node1 = new Node(1);
  Node* node2 = new Node(2);
  node1->next = node2;
  node1->random = node2;
  node2->random = node2;
  Node* result = Solution().copyRandomList(node1);
  printNode(result);
  return 0;
}
