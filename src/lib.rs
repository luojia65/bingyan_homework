#![feature(box_into_raw_non_null)]
pub mod linked_list;

use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct BPlusTree<K, V> {
    root: Root<K, V>,
    head: Option<NonNull<Leaf<K, V>>>,
    tail: Option<NonNull<Leaf<K, V>>>,
    _marker: PhantomData<Box<Leaf<K, V>>>
}

impl<K, V> BPlusTree<K, V> {
    fn new() -> Self {
        BPlusTree {
            root: Root::None, 
            head: None,
            tail: None,
            _marker: PhantomData
        }
    }

    fn insert(&mut self, key: K, value: V) {
        match self.root {
            Root::None => {// insert a new leaf page as root
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
                unimplemented!("此处有所有权问题，稍后完善")
            }, 
            Root::Leaf(mut node) => unsafe {node.as_mut()}.insert(key, value),
            Root::Internal(mut node) => unsafe {node.as_mut()}.insert(key, value),
        }
    }
}

enum Root<K, V> {
    Leaf(NonNull<Leaf<K, V>>),
    Internal(NonNull<Internal<K, V>>),
    None
}

struct Leaf<K, V> {
    tree: NonNull<BPlusTree<K, V>>,
    data: Vec<(K, V)>,
    parent: Option<NonNull<Internal<K, V>>>,
    prev: Option<NonNull<Leaf<K, V>>>,
    next: Option<NonNull<Leaf<K, V>>>,
}

impl<K, V> Leaf<K, V> {

    #[inline]
    fn insert(&mut self, key: K, value: V) {
        unimplemented!()
    }
}

struct Internal<K, V> {
    tree: NonNull<BPlusTree<K, V>>,
    
}

impl<K, V> Internal<K, V> {
    #[inline]
    fn insert(&mut self, key: K, value: V) {
        unimplemented!()
    }
}