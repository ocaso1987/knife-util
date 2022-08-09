pub trait VecUtil<T> {
    fn map<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R;
}

impl<T> VecUtil<T> for Vec<T> {
    fn map<F, R>(&self, fun: F) -> Vec<R>
    where
        F: Fn(&T) -> R,
    {
        self.iter().map(fun).collect()
    }
}
