#![feature(box_into_raw_non_null)]
use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct BPlusTree<K: Ord, V> {
    root: Root<K, V>,
    head: Option<NonNull<Leaf<K, V>>>,
    tail: Option<NonNull<Leaf<K, V>>>,
    order: usize, 
    _marker: PhantomData<Box<Leaf<K, V>>>
}

impl<K: Ord, V> BPlusTree<K, V> {
    fn new() -> Self {
        BPlusTree {
            root: Root::None, 
            head: None,
            tail: None,
            order: 10,  // todo!
            _marker: PhantomData
        }
    }

    fn replace(&mut self, key: K, value: V) {
        if let Root::None = self.root {
            // replace a new leaf page as root
            let leaf = Leaf {
                tree: unsafe { NonNull::new_unchecked(self) },
                data: Vec::new(),
                parent: None,
                prev: None,
                next: None,
            };
            let leaf = Box::into_raw_non_null(Box::new(leaf));
            self.root = Root::Leaf(leaf);
            self.head = Some(leaf);
            self.tail = Some(leaf);
        }
        match self.root {
            Root::None => unreachable!(),
            Root::Leaf(mut node) => unsafe {node.as_mut()}.replace(key, value),
            Root::Internal(mut node) => unsafe {node.as_mut()}.replace(key, value),
        }
    }
}

enum Root<K: Ord, V> {
    Leaf(NonNull<Leaf<K, V>>),
    Internal(NonNull<Internal<K, V>>),
    None
}

struct Leaf<K: Ord, V> {
    tree: NonNull<BPlusTree<K, V>>,
    data: Vec<(K, V)>,
    parent: Option<NonNull<Internal<K, V>>>,
    prev: Option<NonNull<Leaf<K, V>>>,
    next: Option<NonNull<Leaf<K, V>>>,
}

impl<K: Ord, V> Leaf<K, V> {
    #[inline]
    fn replace(&mut self, key: K, value: V) {
        let full = self.data.len() >= unsafe { self.tree.as_ref() }.order;
        let (found, index) = match self.data.binary_search_by(|(key_probe, _)| key_probe.cmp(&key)) {
            Ok(index) => (false, index),
            Err(index) => (true, index)
        };
        let need_split = full && !found;
        if need_split {
            let mut left = Leaf {
                tree: self.tree,
                data: Vec::new(),
                parent: None,
                prev: None,
                next: None,
            };
            let mut right = Leaf {
                tree: self.tree,
                data: Vec::new(),
                parent: None,
                prev: None,
                next: None,
            };
            if let Some(prev) = self.prev {
                unimplemented!("尚未完成")
            }
        } else { // !need_split
            if found {
                self.data[index] = (key, value);
            } else {
                self.data.insert(index, (key, value));
            } 
            unimplemented!("update the parent")
        }
    }
}

struct Internal<K: Ord, V> {
    tree: NonNull<BPlusTree<K, V>>,
    parent: Option<NonNull<Internal<K, V>>>,
}

impl<K: Ord, V> Internal<K, V> {
    #[inline]
    fn replace(&mut self, key: K, value: V) {
        unimplemented!()
    }

    #[inline]
    fn do_update_on_replace(&mut self) {

    }
}

#[cfg(test)]
mod b_plus_tree_tests {

}

// --- 


#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<Box<LinkedListNode<T>>>  
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct LinkedListNode<T> {
    prev: Option<NonNull<LinkedListNode<T>>>,
    next: Option<NonNull<LinkedListNode<T>>>,
    data: T
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
        Default::default()
    }
    push_pop_impl!(T, push_back,  pop_back,  head, tail, next, prev);
    push_pop_impl!(T, push_front, pop_front, tail, head, prev, next);
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: None,
            _marker: PhantomData
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
        //println!("dropped list")
    }
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

