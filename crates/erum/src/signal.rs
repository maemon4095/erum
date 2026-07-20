pub trait Signal {
    type Item;
    type Hint;
    type Subscription;

    fn get(&self) -> Self::Item;
    fn subscribe<L: SignalListener<Self::Item, Self::Hint>>(
        &self,
        listener: L,
    ) -> Self::Subscription;
}

pub trait SignalListener<T, H> {
    fn on_changed(&self, value: T, hint: H);
}
