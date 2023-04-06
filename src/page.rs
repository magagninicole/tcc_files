pub trait Entry<T> {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool;
    fn is_leaf(&self) -> bool;
    fn is_branch(&self) -> bool;
    fn set_entry(&mut self, entry: T);
    fn get_entry(&self) -> T;
}

pub trait Table {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

#[derive(Copy, Clone)]
pub enum PageBits {
    UserReadExecute,
    UserReadWrite,
    UserReadWriteExecute,

    ReadExecute,
    ReadWrite,
}
