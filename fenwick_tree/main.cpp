#include <cstdlib>
#include <iostream>
#include <vector>

#define LSB(i) ((i) & -(i))

struct fenwick_tree {
 private:
  int n;
  std::vector<int> tree;

 public:
  fenwick_tree() { n = 0; }
  explicit fenwick_tree(int sz, std::vector<int> values) {
    tree.reserve(sz);
    if (values.size() < 1) return;
    auto N = values.size();
    values[0] = 0;

    tree = values;

    for (int i = 1; i < N; ++i) {
      int parent = i + LSB(i);
      if (parent < N) tree[parent] += tree[i];
    }
  }

  auto prefix_sum(int right) -> int64_t {
    int64_t sum = tree[0];
    for (; right != 0; right -= LSB(right))
      sum += tree[right];
    return sum;
  }

  auto sum(int left, int right) -> int64_t {
    return prefix_sum(right) - prefix_sum(left - 1);
  }

  auto get_vector() -> std::vector<int> { return tree; }
};

auto main() -> int {
  // must be zero based
  std::vector<int> values{0, 5, 4, -3, 8, 1, 2};

  fenwick_tree fwt(7, values);
  auto vec = fwt.get_vector();
  for (size_t i = 1; i < vec.size(); ++i) {
    std::cout << vec[i] << ' ';
  }

  std::cout << '\n';

  return EXIT_SUCCESS;
}
