use std::cmp::Ordering;
use crate::db_builder::InternalKeyComparator;
use crate::utils::comparators::Comparator;
use crate::utils::varint::VarintU32;

mod inlineskiplist;
mod arena;



#[derive(Clone, Default)]
pub struct KeyComparator<C: Comparator> {
    icmp: InternalKeyComparator<C>,
}

impl<C:Comparator> Comparator for KeyComparator<C> {

    fn compare(&self,mut a: &[u8],mut b: &[u8]) -> Ordering {
        let ia = extract_varint32_encoded_slice(&mut a);
        let ib = extract_varint32_encoded_slice(&mut b);

        if ia.is_empty() || ib.is_empty() {
            ia.cmp(ib)
        } else{
            self.icmp.compare(ia, ib)
        }

    }

    fn name(&self) -> &str {
        self.icmp.name()
    }

    fn separator(&self, mut a: &[u8], mut b: &[u8]) -> Vec<u8> {
        let ia = extract_varint32_encoded_slice(&mut a);
        let ib = extract_varint32_encoded_slice(&mut b);

        self.icmp.separator(ia, ib)
    }

    fn successors(&self, mut key:&[u8]) -> Vec<u8> {
        let ia = extract_varint32_encoded_slice(&mut key);
        self.icmp.successor(ia)
    }
}

pub struct MemTable<C: Comparator> {
    cmp: KeyComparator<C>,
    table: InlineSkipList<KeyComparator<C>, OffsetArena>,
}



fn extract_varint32_encoded_slice<'a>(src:&mut &'a[u8]) -> &'a [u8] {
    if src.is_empty() {
        return src;
    }

    VarintU32::get_varint_prefixed_slice(src).unwrap_or(src)
}