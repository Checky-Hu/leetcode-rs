#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Node {
 public:
  int val;
  vector<Node*> neighbors;
  Node() {
    val = 0;
    neighbors = vector<Node*>();
  }

  Node(int _val) {
    val = _val;
    neighbors = vector<Node*>();
  }

  Node(int _val, vector<Node*> _neighbors) {
    val = _val;
    neighbors = _neighbors;
  }
};

class Solution {
 public:
  unordered_map<int, Node*> map;
  Node* cloneGraph(Node* node) {
    if (node == nullptr) {
      return nullptr;
    }
    return getNextNode(node);
  }

  Node* getNextNode(Node* node) {
    auto iter = map.find(node->val);
    if (iter != map.end()) {
      return iter->second;
    }
    Node* result = new Node(node->val);
    map.emplace(node->val, result);
    for (auto next : node->neighbors) {
        result->neighbors.push_back(getNextNode(next));
    }
    return result;
  }
};

void printNode(Node* node) {
  cout << node->val << endl;
  for (auto next : node->neighbors) {
      printNode(next);
  }
}

int main(int argc, char ** argv)
{
  vector<Node*> neighbors;
  for (int i = 0; i < 3; i++) {
    neighbors.push_back(new Node(i));
  }
  Node* result = Solution().cloneGraph(new Node(3, neighbors));
  printNode(result);
  return 0;
}
