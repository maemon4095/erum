pub mod messaging;

pub use messaging::ComponentMessage;
pub use messaging::MessageEmitter;
pub use messaging::MessageHandler;

pub trait Component<C: ComponentMountContext> {
    fn mount(ctx: C); // ctxはImmutable構造、子にはサブコンテキストを渡す。
    // いったん親の初期化を終わらせてから子のマウントをしてほしいため。親の初期化が終わった後のみ渡せるように型を調整したい。
}

pub trait ComponentMountContext {
    fn create_context(&self) -> ComponentContext;
}

pub struct ComponentContext {
    // immutable binary tree
    // map: erum_internal_util::collections::InternalSortedMap,
}

impl ComponentContext {
    pub fn perform<T>(&mut self) {}
}

pub trait Element {
    type Message;

    fn handle(&mut self, context: ComponentContext, message: Self::Message);
}
