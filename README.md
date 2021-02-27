# leetcode

执行
```
cargo run
```

输出
```
Welcome to leetcode-rust system.
Please enter a problem id, or enter "random" to generate a random problem.
```

输入问题Id，以108([将有序数组转换为二叉搜索树](https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree/))为例，输出：
```
Problem 108:"Convert Sorted Array to Binary Search Tree" is generated. Enjoy!
```

创建./src/solution/n0108_convert_sorted_array_to_binary_search_tree.rs

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {}
}
```

开心刷题吧。
