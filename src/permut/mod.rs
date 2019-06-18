
use subset::*;

pub fn permut<'a, T> (object: &'a [T], k: usize) -> PermutationIterator<T> {
    PermutationIterator {
        object: object,
        buf: Vec::with_capacity(k)
    }
}

pub fn permut_buf<'a, T> (object: &'a [T], k: usize, buf: Vec<usize>) -> PermutationIterator<T> {
    if buf.len() < k {
        buf.reserve_exact(k - buf.len());
    }
    buf.clear();
    PermutationIterator {
        object:  object,
        buf: buf
    }
}

struct PermutationIterator<'a, T> {
    object: &'a [T],
    buf: Vec<usize>
}

impl<'a, T> PermutationIterator<'a, T> {
    pub fn release(self) -> Vec<usize> {
        self.buf
    }
}

impl<'a, T> Iterator for PermutationIterator<'a, T> {
    type Item = Subset<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            Subset::new_unchecked(set: &'a Vec<T>, idxs: &'a Vec<usize>)
        }
    }
}

// impl DoubleEndedIterator for PermutationIterator {

// }




// TODO iter(), iter_mut(), into_iter()
// TODO ExactSizeIterator
// TODO FusedIterator (???)

// TODO subset: над slice, а не vec
// TODO subset: lifetime сета должен быть больше, чем лайфтайм индексов (??? скорее всего, не нужно)
