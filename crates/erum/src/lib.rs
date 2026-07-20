pub mod backend;
pub mod context;
pub mod effect;
pub mod messaging;
pub mod signal;

use std::any::Any;

pub use backend::{Backend, ContainerBackend};
pub use messaging::{ComponentMessage, MessageEmitter, MessageHandler};

use crate::context::ContextProviderMap;
use crate::effect::{MountEffect, MountEffectHandler};
use crate::messaging::MessageHandlerMap;

pub trait Component<B: Backend> {
    fn mount(&self, context: ComponentMountContext<B>) -> ComponentBuildResult<B>;
}

// mount effectを使ってeffectの管理も行うか？
// -> 無理そう。あきらめて別途扱う。
pub struct ComponentMountContext<B: Backend> {
    context_providers: ContextProviderMap,
    parent_handlers: MessageHandlerMap,
    handlers: MessageHandlerMap,
    backend: B,
}

impl<B: Backend> ComponentMountContext<B> {
    pub fn build(self) -> ComponentMountChildrenContext<B> {
        ComponentMountChildrenContext {
            context_providers: self.context_providers,
            parent_handlers: self.parent_handlers,
            handlers: self.handlers,
            backend: self.backend,
        }
    }

    pub fn backend(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn perform<E: MountEffect>(&mut self, effect: E) -> E::Return
    where
        B: MountEffectHandler<E>,
    {
        self.backend.handle(effect)
    }

    pub fn with_handler<H: MessageHandler>(self, handler: H) -> Self {
        Self {
            context_providers: self.context_providers,
            parent_handlers: self.parent_handlers,
            handlers: self.handlers.with_handler(handler),
            backend: self.backend,
        }
    }
}

pub struct ComponentMountChildrenContext<B: Backend> {
    context_providers: ContextProviderMap,
    parent_handlers: MessageHandlerMap,
    handlers: MessageHandlerMap,
    backend: B,
}

impl<B: Backend> ComponentMountChildrenContext<B> {
    pub fn with_child<C: Component<B::Child>>(self, child: &C) -> ComponentMountChildrenContext<B>
    where
        B: ContainerBackend,
    {
        let backend = self.backend.create_child();
        let builder = ComponentMountContext {
            context_providers: self.context_providers.clone(),
            parent_handlers: self.handlers.clone(),
            handlers: self.handlers.clone(),
            backend,
        };
        let result = child.mount(builder);

        ComponentMountChildrenContext {
            context_providers: self.context_providers,
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

// 別threadからmessageを呼びたいケースが存在するはずなので、channel実装にならざるおえない。
//
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
