//use std::io::{Read, Write, Seek, Result};
//use bytes::{ReadBytesExt, WriteBytesExt};
//use std::fs::File;
//use std::path::Path;
//
//struct BPlusTree {
//    index_file: File, // all internal nodes
//    data_file: File, // leaf nodes
//}
//
//impl BPlusTree {
//    fn open(index_file: File, data_file: File) -> Self {
//        BPlusTree {
//            index_file, data_file
//        }
//    }
//
//    fn create<P1, P2>(index_path: P1, data_path: P2) -> Result<Self>
//    where P1: AsRef<Path>, P2: AsRef<Path> {
//        Ok(BPlusTree {
//            index_file: File::create(index_path)?,
//            data_file: File::create(data_path)?
//        })
//    }
//
//    fn search(&self, key: u64) -> u64 {
//
//    }
//}
//
//
//#[cfg(test)]
//mod b_plus_tree_tests {
//
//}

////未考虑析构函数泄漏，未考虑生命周期，未考虑PhantomData
////这是一段不合格的rust代码！
//
//struct BPlusTree<K: Ord, V> {
//    // I tried &Node, Box, Rc, Arc, Cell, RefCell and Mutex here, none of them succeeded
//    root: NodePtr<K, V>,
//    head: NodePtr<K, V>,
//    order: usize
//}
//
//impl<K: Ord, V> BPlusTree<K, V> {
//    fn new() -> Self {
//        BPlusTree {
//            root: std::ptr::null_mut(),
//            head: std::ptr::null_mut(),
//            order: 6, // todo!
//        }
//    }
//}
//
//struct BPlusNode<K: Ord, V> {
//    is_leaf: bool,
//    parent: NodePtr<K, V>,
//    prev: NodePtr<K, V>,
//    next: NodePtr<K, V>,
//    entries: Vec<(K, V)>,
//    children: Vec<NodePtr<K, V>>,
//    tree: *mut BPlusTree<K, V>
//}
//
//impl<K: Ord, V> BPlusNode<K, V> {
//
//    fn new_leaf(parent: NodePtr<K, V>, prev: NodePtr<K, V>, next: NodePtr<K, V>, tree: *mut BPlusTree<K, V>) -> Self {
//        BPlusNode {
//            is_leaf: true,
//            parent,
//            prev,
//            next,
//            entries: Vec::new(),
//            children: Vec::new(),
//            tree
//        }
//    }
//
//    fn try_search(&self, key: &K) -> Result<&V, usize> {
//        if self.is_leaf {
//            match self.entries.binary_search_by(|(k, _v)| k.cmp(key)) {
//                Ok(index) => return Ok(&self.entries[index].1),
//                Err(index) => return Err(index)
//            }
//        }
//        if self.entries.is_empty() {
//            return Err(0);
//        }
//        let ans = if key < &self.entries[0].0 {
//            unsafe { &*self.children[0] }
//        } else if key >= &self.entries[self.entries.len() - 1].0 {
//            unsafe { &*self.children[self.children.len() - 1] }
//        } else {
//            match self.entries.binary_search_by(|(k, _v)| k.cmp(key)) {
//                Ok(index) => return Ok(&self.entries[index].1),
//                Err(index) => unsafe { &*self.children[index + 1] }
//            }
//        }.try_search(key);
//        ans
//    }
//
//    fn try_replace_into(&mut self, (key, value): (K, V)) {
//        if self.is_leaf {
//            let order = unsafe { (*self.tree).order };
//            if self.entries.len() < order {
//                match self.entries.binary_search_by(|(k, _v)| k.cmp(&key)) {
//                    Ok(index) => self.entries[index].1 = value,
//                    Err(index) => self.entries.insert(index, (key, value))
//                }
//            } else {
//                let mut left = BPlusNode::new_leaf(self.parent, self.prev, self.next, self.tree);
//                let left_ptr = Box::into_raw(Box::new(left));
//                let mut right = BPlusNode::new_leaf(self.parent, self.prev, self.next, self.tree);
//                let right_ptr = Box::into_raw(Box::new(right));
//                if self.prev != std::ptr::null_mut() {
//                    unsafe {
//                        (*self.prev).next = left_ptr;// as *mut BPlusNode<K, V>;
//                        left.prev =(*self).prev
//                    }
//                }
//                if self.next != std::ptr::null_mut() {
//                    unsafe {
//                        (*self.next).prev = right_ptr;
//                        right.next = (*self).next
//                    }
//                }
//                if self.prev != std::ptr::null_mut() {
//                    unsafe {
//                        (*self.tree).head = left_ptr;
//                    }
//                }
//
//            }
//        }
//    }
//}
//
//type NodePtr<K, V> = *mut BPlusNode<K, V>;
//

#![feature(box_into_raw_non_null)]

// 在链表上建树，而不是在树上建链表
// 以下为"叶子节点"链表（并非单个数据的链表）

use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>
}

macro_rules! push_pop_impl {
    ($inner_type: ty, $push_fn: ident, $pop_fn: ident, 
    $new_node_from: ident, $old_node_from: ident, $new_node_dir: ident, $old_node_dir: ident) => {
        
    pub fn $push_fn(&mut self, data: $inner_type) {
        let mut new_node = Box::new(LinkedListNode {
            prev: None,
            next: None,
            data
        });
        new_node.$old_node_dir = self.$old_node_from;
        let new_node = Some(Box::into_raw_non_null(new_node));
        match self.$old_node_from {
            Some(mut node) => unsafe { node.as_mut() } .$new_node_dir = new_node,
            None => self.$new_node_from = new_node
        }
        self.$old_node_from = new_node;
    }

    pub fn $pop_fn(&mut self) -> Option<$inner_type> {
        if let Some(old_node) = self.$old_node_from {
            let old_node = unsafe { Box::from_raw(old_node.as_ptr()) };
            self.$old_node_from = old_node.$old_node_dir;
            match self.$old_node_from {
                Some(mut old_ptr) => unsafe { old_ptr.as_mut() }.$new_node_dir = None,
                None => self.$new_node_from = None
            }
            return Some(old_node.data);
        }
        None
    }

    };
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None
        }
    }
    //rust为什么厉害，就厉害在这
    push_pop_impl!(T, push_back,  pop_back,  head, tail, next, prev);
    push_pop_impl!(T, push_front, pop_front, tail, head, prev, next);
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
        println!("dropped list")
    }
}

struct LinkedListNode<T> {
    prev: Option<NonNull<LinkedListNode<T>>>,
    next: Option<NonNull<LinkedListNode<T>>>,
    data: T
}

#[cfg(test)]
mod linked_list_tests {
    use super::*;
    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(Some(3), list.pop_back());
        assert_eq!(Some(2), list.pop_back());
        assert_eq!(Some(1), list.pop_back());
        assert_eq!(None, list.pop_back());
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(Some(1), list.pop_front());
        assert_eq!(Some(2), list.pop_front());
        assert_eq!(Some(3), list.pop_front());
        assert_eq!(None, list.pop_front());
    }

    #[test]
    fn test_drop() {
        struct Data(u8);
        impl Drop for Data {
            fn drop(&mut self) {
                println!("dropped {}", self.0)
            }
        }
        let mut list = LinkedList::new();
        list.push_back(Data(1));
        list.push_back(Data(2));
        list.push_back(Data(3));
        // Now list is out of scope
    }
}

