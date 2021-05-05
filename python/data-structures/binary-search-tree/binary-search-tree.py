import queue
from collections import deque


class TreeNode:
    def __init__(self, value, left_child=None, right_child=None):
        self.value = value
        self.left_child = left_child
        self.right_child = right_child


def search(node, searched_value):
    if node is None or node.value == searched_value:
        return node
    elif searched_value < node.value:
        return search(node.left_child, searched_value)
    else:
        return search(node.right_child, searched_value)


def insert(node, value):
    if value < node.value:
        if node.left_child is None:
            node.left_child = TreeNode(value)
        else:
            insert(node.left_child, value)

    if value > node.value:
        if node.right_child is None:
            node.right_child = TreeNode(value)
        else:
            insert(node.right_child, value)


def delete(node, value_to_delete):
    if node is None:
        return None
    elif value_to_delete < node.value:
        node.left_child = delete(node.left_child, value_to_delete)
        return node
    elif value_to_delete > node.value:
        node.right_child = delete(node.right_child, value_to_delete)
        return node
    elif value_to_delete == node.value:
        if node.left_child is None:
            return node.right_child
        elif node.right_child is None:
            return node.left_child
        else:
            node.right_child = lift(node.right_child, node)
            return node


def lift(node, node_to_delete):
    if node.left_child:
        node.left_child = lift(node.left_child, node_to_delete)
        return node
    else:
        node_to_delete.value = node.value
        return node.right_child


def inorder_traversal(node, values=[]):
    if node is None:
        return
    inorder_traversal(node.left_child)
    values.append(node.value)
    inorder_traversal(node.right_child)
    return values


def preorder_traversal(node, values=[]):
    if node is None:
        return
    values.append(node.value)
    preorder_traversal(node.left_child)
    preorder_traversal(node.right_child)
    return values


def postorder_traversal(node, values=[]):
    if node is None:
        return
    postorder_traversal(node.left_child)
    postorder_traversal(node.right_child)
    values.append(node.value)
    return values


# Breadth-First Search
def levelorder_traversal(node):
    q = queue.Queue()
    result = []

    if node:
        q.put(node)

    while q.qsize():
        level = []
        for _ in range(q.qsize()):
            node = q.get()
            level.append(node.value)

            if node.left_child:
                q.put(node.left_child)
            if node.right_child:
                q.put(node.right_child)
        result.append(level)
    return result
