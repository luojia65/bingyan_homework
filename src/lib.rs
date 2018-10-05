#![feature(box_into_raw_non_null)]
pub mod linked_list;

// 尝试一次性建完整的树，比如说B树
use std::ptr::NonNull;

// pub struct BTree<K, V> {

// }

// struct Node<K, V> {
//     tree: NonNull<BTree<K, V>>
// }