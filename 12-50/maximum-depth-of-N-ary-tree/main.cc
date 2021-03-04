#include <iostream>
#include <vector>

using namespace std;

class Node {
public:
  int val;
  vector<Node*> children;

  Node() {}

  Node(int _val) {
    val = _val;
  }

  Node(int _val, vector<Node*> _children) {
    val = _val;
    children = _children;
  }
};

class Solution {
public:
  int maxDepth(Node* root) {
    if (root == nullptr) {
      return 0;
    }
    int max = 0;
    for (vector<Node*>::const_iterator iter = root->children.begin();
        iter != root->children.end(); iter++) {
      int tmp = maxDepth(*iter);
      if (tmp > max) {
        max = tmp;
      }
    }
    return max + 1;
  }
};

int main(int argc, char ** argv)
{
  vector<Node*> children;
  children.push_back(new Node(2));
  children.push_back(new Node(3));
  Node* root = new Node(1, children);
  cout << Solution().maxDepth(root) << endl;
  return 0;
}
