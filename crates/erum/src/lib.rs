pub mod messaging;

use std::any::Any;

pub use messaging::ComponentMessage;
pub use messaging::MessageEmitter;
pub use messaging::MessageHandler;

use crate::messaging::MessageHandlerMap;

pub trait Component<B: Backend> {
    fn mount(&self, builder: ComponentBuilder<B>) -> ComponentBuildResult<B>;
}

// backendをcomponentに公開する必要がある。immutableに寄せたいため、backendをmapできる機能が必要。
pub trait Backend {
    type Element;
    fn create_child(&self) -> Self;
    fn with_child(&self, child: Self::Element) -> Self;
    // thread-safeなcomponent contextを渡してあげる。 -> さきに渡してあげた方がよい？
    // invokeがthread-safeというかevent-threadで実行されるようにする必要がある。
    //
    // rcをつかっているため、Arcでくるむ必要がある。
    //  -> edtが別スレッドよりも長く生き残る可能性があるため全体をarcにする必要があるか？
    //
    //  しかし全体をarcはやりすぎな気がする。
    fn build(self) -> Self::Element;
}

pub struct ComponentBuilder<B: Backend> {
    parent_handlers: MessageHandlerMap,
    handlers: MessageHandlerMap,
    backend: B,
}

impl<B: Backend> ComponentBuilder<B> {
    pub fn build(self) -> ComponentChildBuilder<B> {
        ComponentChildBuilder {
            parent_handlers: self.parent_handlers,
            handlers: self.handlers,
            backend: self.backend,
        }
    }

    pub fn with_handler<H: MessageHandler>(self, handler: H) -> Self {
        Self {
            parent_handlers: self.parent_handlers,
            handlers: self.handlers.with_handler(handler),
            backend: self.backend,
        }
    }
}

pub struct ComponentChildBuilder<B: Backend> {
    parent_handlers: MessageHandlerMap,
    handlers: MessageHandlerMap,
    backend: B,
}

impl<B: Backend> ComponentChildBuilder<B> {
    pub fn with_child<C: Component<B>>(self, child: &C) -> ComponentChildBuilder<B> {
        let backend = self.backend.create_child();
        let builder = ComponentBuilder {
            parent_handlers: self.handlers.clone(),
            handlers: self.handlers.clone(),
            backend,
        };
        let result = child.mount(builder);

        ComponentChildBuilder {
            parent_handlers: self.parent_handlers,
            handlers: self.handlers,
            backend: self.backend.with_child(result.element),
        }
    }

    pub fn build(self) -> ComponentBuildResult<B> {
        ComponentBuildResult {
            element: self.backend.build(),
        }
    }
}

pub struct ComponentBuildResult<B: Backend> {
    element: B::Element,
}

pub struct ComponentContext {
    handlers: MessageHandlerMap,
}

impl ComponentContext {
    pub fn try_perform<T>(&self, message: T) -> Result<(), T>
    where
        T: Any,
    {
        self.handlers.try_invoke(self, message)
    }

    pub fn perform<T>(&self, message: T)
    where
        T: Any,
    {
        assert!(self.try_perform(message).is_ok(), "unhandled message")
    }
}

pub trait Element {
    type Message;

    fn handle(&mut self, context: ComponentContext, message: Self::Message);
}
