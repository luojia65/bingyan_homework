use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct LinkedList<T> {
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    // rust为什么这么难写，就难写在这
    // 告诉编译器我用于这个Box，方便借用检查
    _marker: PhantomData<Box<LinkedListNode<T>>>  
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
            tail: None,
            _marker: PhantomData
        }
    }
    //rust为什么厉害，就厉害在这
    push_pop_impl!(T, push_back,  pop_back,  head, tail, next, prev);
    push_pop_impl!(T, push_front, pop_front, tail, head, prev, next);
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
        //println!("dropped list")
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

