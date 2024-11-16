use std::{mem, ptr};
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicUsize, Ordering};
use bytes::Bytes;
use crate::mem::arena::Arena;
use crate::utils::comparators::Comparator;

const MAX_HEIGHT: usize = 20;

const HEIGHT_INCREASE: u32 = u32::MAX / 3;

pub struct Node {
    key: Bytes,
    height: usize,
    next_nodes: [AtomicPtr<Self>; MAX_HEIGHT],
}

impl Node {
    fn new<A: Arena>(key: Bytes, height: usize, arena: &A) -> *mut Self {
        let size =
            mem::size_of::<Self>() - (MAX_HEIGHT - height) * mem::size_of::<AtomicPtr<Self>>();
        let align = mem::align_of::<Self>();
        let p = unsafe { arena.allocate::<Node>(size, align) };
        assert!(!p.is_null());
        unsafe {
            let node = &mut *p;
            ptr::write(&mut node.key, key);
            ptr::write(&mut node.height, height);
            ptr::write_bytes(node.next_nodes.as_mut_ptr(), 0, height);
            p
        }
    }

    #[inline]
    // height, 0-based index
    fn get_next(&self, height: usize) -> *mut Node {
        self.next_nodes[height].load(Ordering::SeqCst)
    }

    #[inline]
    // height, 0-based index
    fn set_next(&self, height: usize, node: *mut Node) {
        self.next_nodes[height].store(node, Ordering::SeqCst);
    }

    #[inline]
    fn key(&self) -> &[u8] {
        &self.key
    }
}

#[derive(Clone)]
pub struct InlineSkipList<C: Comparator, A: Arena + Clone + Send + Sync> {
    inner: Arc<InlineSkipListInner<A>>,

}

struct InlineSkipListInner<A: Arena> {
    height: AtomicUsize,
    head: NonNull<Node>,
    arena: A,
    size: AtomicUsize,
}