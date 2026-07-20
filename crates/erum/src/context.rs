mod provider_map;

pub use provider_map::ContextProviderMap;

pub trait ContextDefinition: 'static + Ord {
    type Item;
    type Hint;
}
