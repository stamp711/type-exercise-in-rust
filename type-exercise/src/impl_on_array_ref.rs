use crate::{PrimitiveArray, PrimitiveType, StringArray};

pub trait ArrayRef<'a> {
    type RefItem: 'a;
    fn get_ref(&self, idx: usize) -> Option<Self::RefItem>;
    fn len(&self) -> usize;
    fn iter<'s>(&'s self) -> ArrayIterator<'s, Self>
    where
        Self: Sized,
    {
        ArrayIterator {
            array: &self,
            pos: 0,
        }
    }
}

impl<'a, T: PrimitiveType> ArrayRef<'a> for &'a PrimitiveArray<T> {
    type RefItem = &'a T;
    fn get_ref(&self, idx: usize) -> Option<Self::RefItem> {
        if self.bitmap[idx] {
            Some(&self.data[idx])
        } else {
            None
        }
    }
    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<'a> ArrayRef<'a> for &'a StringArray {
    type RefItem = &'a str;
    fn get_ref(&self, idx: usize) -> Option<Self::RefItem> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        (*self).len()
    }
}

pub struct ArrayIterator<'a, A> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: ArrayRef<'a>> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.array.len() {
            let item = self.array.get_ref(self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[allow(dead_code, unreachable_code, unused_variables)]
fn test() {
    let s: StringArray = todo!();
    for i in (&s).iter() {
        todo!();
    }
}
