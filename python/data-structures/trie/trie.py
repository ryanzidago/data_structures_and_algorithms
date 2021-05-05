class TrieNode:
    def __init__(self):
        self.children = {}


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def search(self, word):
        # Time complexity:
        # O(K)
        current_node = self.root

        for char in word:
            if current_node.children.get(char):
                current_node = current_node.children[char]
            else:
                return None
        return current_node

    def insert(self, word):
        # Time complexity:
        # O(K)
        current_node = self.root

        for char in word:
            if current_node.children.get(char):
                current_node = current_node.children[char]
            else:
                new_node = TrieNode()
                current_node.children[char] = new_node
                current_node = new_node

        current_node.children["*"] = None

    def collect_all_words(self, node=None, word="", words=[]):
        current_node = node or self.root

        for key, child_node in current_node.children.items():
            if key == "*":
                words.append(word)
            else:
                self.collect_all_words(child_node, word + key, words)
        return words

    def autocomplete(self, prefix):
        current_node = self.search(prefix)
        if not current_node:
            return None
        return self.collect_all_words(current_node)


trie = Trie()
trie.insert("get")
trie.insert("go")
trie.insert("got")
trie.insert("gotten")
trie.insert("hall")
trie.insert("ham")
trie.insert("hammer")
trie.insert("hill")
trie.insert("zebra")

print(trie)
print(trie.autocomplete("go"))
print(trie.search("gottt"))
