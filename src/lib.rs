#![feature(box_into_raw_non_null)]
pub mod linked_list;

use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct BPlusTree<K, V> {
    root: Option<NonNull<Node<K, V>>>,
    head: Option<NonNull<Leaf<K, V>>>,
    tail: Option<NonNull<Leaf<K, V>>>,
    _marker: PhantomData<Box<Node<K, V>>>
}

impl<K, V> BPlusTree<K, V> {
    fn new() -> Self {
        BPlusTree {
            root: None, 
            head: None,
            tail: None,
            _marker: PhantomData
        }
    }

    fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => {// insert a new leaf page as root
                let leaf = Leaf {
                    tree: unsafe { NonNull::new_unchecked(self) },
                    data: Vec::new(),
                    parent: None,
                    prev: None,
                    next: None,
                };
                unimplemented!("此处有所有权问题，稍后完善")
            }, 
            Some(mut ptr) => unsafe {ptr.as_mut()}.insert(key, value) 
        }
    }
}

enum Node<K, V> {
    Leaf(Leaf<K, V>),
    Internal(Internal<K, V>)
}

impl<K, V> Node<K, V> {
    #[inline]
    fn insert(&mut self, key: K, value: V) {
        match self {
            Node::Leaf(l) => l.insert(key, value),
            Node::Internal(l) => l.insert(key, value),
        }
    }
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