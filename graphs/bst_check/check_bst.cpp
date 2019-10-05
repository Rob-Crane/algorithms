struct Node {
	int data; // 0 <= data <= 1E4
	Node* left;
	Node* right;
}

bool checkBST(Node* root) {
	int min_so_far = -1;
	return checkSubtree(root, min_so_far);
}

bool checkSubtree(Node* node, int& min_so_far) {
	if (node->left != nullptr && !checkSubtree(node->left, min_so_far)) {
		return false;
	}
	if (node->data <= min_so_far) {
		return false;
	}
	min_so_far = node->data;
	if (node->right != nullptr && !checkSubtree(node->right, min_so_far)) {
		return false;
	}
	return true;
}
