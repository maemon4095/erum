pub trait Backend {
    type Element;
    fn build(self) -> Self::Element;
}

pub trait ContainerBackend: Backend {
    type ChildElement;
    type Child: Backend<Element = Self::ChildElement>;

    fn create_child(&self) -> Self::Child;
    fn with_child(&self, child: Self::ChildElement) -> Self;
}
