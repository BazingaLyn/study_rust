use std::cmp::Ordering;
use crate::leveldb::utils::coding::decode_fixed_64;
use crate::leveldb::utils::comparators::Comparator;


pub const INTERNAL_KEY_SIZE: usize = 8;

#[derive(Clone, Default)]
pub struct InternalKeyComparator<C: Comparator> {
    pub user_comparator: C,
}

impl<C: Comparator> InternalKeyComparator<C> {
    pub fn new(user_comparator: C) -> Self {
        user_comparator
    }
}


impl<C: Comparator> Comparator for InternalKeyComparator<C> {
    fn compare(&self, a: &[u8], b: &[u8]) -> Ordering {
        let user_a = extract_user_key(a);
        let user_b = extract_user_key(b);

        match self.user_comparator(user_a, user_b) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let sa = extract_seq_number(a);
                let sb = extract_seq_number(b);
                if sa > sb {
                    Ordering::Less
                } else if sa == sb {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }
        }
    }

    fn name(&self) -> &str {
        "leveldb.InternalKeyComparator"
    }

    fn separator(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        todo!()
    }

    fn successors(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        todo!()
    }
}
// 在leveldb中用户输入key后，在程序内部会在后面增加8位的后缀，7个seqnumber + 1 operation type
#[inline] // 内联函数，告诉编译器，尽量编译到对应的二进制文件中去
pub fn extract_user_key(key: &[u8]) -> &[u8] {
    let size = key.len();
    assert!(
        size >= INTERNAL_KEY_SIZE,
        "[internal key] invalid size of internal key : expect >= {} but got {}",
        INTERNAL_KEY_SIZE,
        size
    );
    &key[..size - INTERNAL_KEY_SIZE]
}


pub fn extract_seq_number(key: &[u8]) -> u64 {
    let size = key.len();
    assert!(
        size >= INTERNAL_KEY_SIZE,
        "[internal key] invalid size of internal key : expect >= {} but got {}",
        INTERNAL_KEY_SIZE,
        size
    );
    decode_fixed_64(&key[size - INTERNAL_KEY_SIZE..]) >> INTERNAL_KEY_SIZE
}