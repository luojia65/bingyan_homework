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

//未考虑析构函数泄漏，未考虑生命周期，未考虑PhantomData
//这是一段不合格的rust代码！

struct BPlusTree<K: Ord, V> {
    // I tried &Node, Box, Rc, Arc, Cell, RefCell and Mutex here, none of them succeeded
    root: NodePtr<K, V>,
    head: NodePtr<K, V>,
    order: usize
}

impl<K: Ord, V> BPlusTree<K, V> {
    fn new() -> Self {
        BPlusTree {
            root: std::ptr::null_mut(),
            head: std::ptr::null_mut(),
            order: 6, // todo!
        }
    }
}

struct BPlusNode<K: Ord, V> {
    is_leaf: bool,
    parent: NodePtr<K, V>,
    prev: NodePtr<K, V>,
    next: NodePtr<K, V>,
    entries: Vec<(K, V)>,
    children: Vec<NodePtr<K, V>>,
    tree: *mut BPlusTree<K, V>
}

impl<K: Ord, V> BPlusNode<K, V> {

    fn new_leaf(parent: NodePtr<K, V>, prev: NodePtr<K, V>, next: NodePtr<K, V>, tree: *mut BPlusTree<K, V>) -> Self {
        BPlusNode {
            is_leaf: true,
            parent,
            prev,
            next,
            entries: Vec::new(),
            children: Vec::new(),
            tree
        }
    }

    fn try_search(&self, key: &K) -> Result<&V, usize> {
        if self.is_leaf {
            match self.entries.binary_search_by(|(k, _v)| k.cmp(key)) {
                Ok(index) => return Ok(&self.entries[index].1),
                Err(index) => return Err(index)
            }
        }
        if self.entries.is_empty() {
            return Err(0);
        }
        let ans = if key < &self.entries[0].0 {
            unsafe { &*self.children[0] }
        } else if key >= &self.entries[self.entries.len() - 1].0 {
            unsafe { &*self.children[self.children.len() - 1] }
        } else {
            match self.entries.binary_search_by(|(k, _v)| k.cmp(key)) {
                Ok(index) => return Ok(&self.entries[index].1),
                Err(index) => unsafe { &*self.children[index + 1] }
            }
        }.try_search(key);
        ans
    }

    fn try_replace_into(&mut self, (key, value): (K, V)) {
        if self.is_leaf {
            let order = unsafe { (*self.tree).order };
            if self.entries.len() < order {
                match self.entries.binary_search_by(|(k, _v)| k.cmp(&key)) {
                    Ok(index) => self.entries[index].1 = value,
                    Err(index) => self.entries.insert(index, (key, value))
                } 
            } else {
                let mut left = BPlusNode::new_leaf(self.parent, self.prev, self.next, self.tree);
                let left_ptr = Box::into_raw(Box::new(left));
                let mut right = BPlusNode::new_leaf(self.parent, self.prev, self.next, self.tree);
                let right_ptr = Box::into_raw(Box::new(right));
                if self.prev != std::ptr::null_mut() {
                    unsafe {
                        (*self.prev).next = left_ptr;// as *mut BPlusNode<K, V>;
                        left.prev =(*self).prev
                    }
                } 
                if self.next != std::ptr::null_mut() {
                    unsafe {
                        (*self.next).prev = right_ptr;
                        right.next = (*self).next
                    }
                }
                if self.prev != std::ptr::null_mut() {
                    unsafe {
                        (*self.tree).head = left_ptr;
                    }
                }

            }
        }
    }
}

type NodePtr<K, V> = *mut BPlusNode<K, V>;


