use std::rc::Rc;

pub fn create_rc_slice_from_iter<I: ExactSizeIterator>(mut iter: I) -> Rc<[<I as Iterator>::Item]> {
    let len = iter.len();
    let mut rc = Rc::new_uninit_slice(len);

    let slice = Rc::get_mut(&mut rc).unwrap();

    for out in slice.iter_mut() {
        let item = iter.next().unwrap();
        out.write(item);
    }

    unsafe { rc.assume_init() }
}
