extern mod core;
use core::vec::*;

pub struct PendingList<T> {
    objects: ~[T],
    pendingAdd: ~[T],
    pendingRemove: ~[T]
}
pub fn PendingList<T>() -> PendingList<T> {
    PendingList {
        objects: ~[],
        pendingAdd: ~[],
        pendingRemove: ~[]
    }
}
pub impl<T> PendingList<T> {
    fn add(&mut self,object: T) {
        self.pendingAdd.push(object);
    }
    fn remove(&mut self,object: T) {
        self.pendingRemove.push(object);
    }
    fn handlePending(&mut self) {
        while self.pendingAdd.len() > 0 {
            self.objects.push(self.pendingAdd.pop());
        }

        while self.pendingRemove.len() > 0 {
            remove_elem(&mut self.objects,self.pendingRemove.pop());
        }
    }
    pub fn each_mut(&mut self,f: &fn(elem: &mut T) -> bool) {
        each_mut(self.objects,f);
    }
}
impl<T> iter::BaseIter<T> for PendingList<T> {
    pure fn each(&self, blk: fn(v: &T) -> bool) { self.objects.each(blk) }
    pure fn size_hint(&self) -> Option<uint> { self.objects.size_hint() }
}
fn remove_elem<T>(v:&mut ~[T], x:T) -> Option<uint> {
    let result = match position(*v, |&y| { core::ptr::ref_eq(&x,&y) }) {
        None => None,
        Some(index) => {
            remove(v,index);
            Some(index)
        }
    };
    result
}
