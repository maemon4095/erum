use std::any::Any;
use std::any::TypeId;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::messaging::message::MessageHandlerRegistrationContext;
use erum_internal_util::collections::{InternalSortedMap, SortedMapEntry};

pub use super::{ComponentMessage, MessageEmitter, MessageHandler};
pub use crate::ComponentContext;

#[derive(Clone)]
pub struct MessageHandlerMap {
    handlers: InternalSortedMap<HandlerMapEntry>,
}

impl MessageHandlerMap {
    pub fn registered<H: MessageHandler, M: 'static + Into<H::Message>>(
        &self,
        handler: Rc<H>,
    ) -> Self {
        let handlers = self.handlers.inserted(HandlerMapEntry {
            handler: Rc::new(MessageHandlerBox::<H, M> {
                handler,
                marker: PhantomData,
            }),
        });

        Self { handlers }
    }

    pub fn with_handler<H: MessageHandler>(&self, handler: H) -> Self {
        let handler = Rc::new(handler);
        let mut ctx = MessageHandlerRegistrationContext {
            map: self.clone(),
            handler,
        };

        <H::Message as ComponentMessage>::register(&mut ctx);

        Self {
            handlers: ctx.map.handlers,
        }
    }

    pub fn try_invoke<M: 'static>(&self, context: &ComponentContext, message: M) -> Result<(), M> {
        let Some(handler) = self.handlers.get(TypeId::of::<M>()) else {
            return Err(message);
        };

        let mut slot = Some(message);

        handler.handler.handle(context, &mut slot);

        assert!(slot.is_none(), "Handled message must be consumed");

        Ok(())
    }
}

#[derive(Clone)]
struct HandlerMapEntry {
    handler: Rc<dyn AnyMessageHandler>,
}

impl SortedMapEntry for HandlerMapEntry {
    type Key<'a>
        = TypeId
    where
        Self: 'a;

    fn key(&self) -> Self::Key<'_> {
        self.handler.ty()
    }
}

trait AnyMessageHandler {
    fn ty(&self) -> TypeId;
    fn handle(&self, context: &ComponentContext, message: &mut dyn Any);
}

struct MessageHandlerBox<H: MessageHandler, M: 'static + Into<H::Message>> {
    handler: Rc<H>,
    marker: PhantomData<fn(M)>,
}

impl<T: MessageHandler, E: 'static + Into<T::Message>> AnyMessageHandler
    for MessageHandlerBox<T, E>
{
    fn ty(&self) -> TypeId {
        TypeId::of::<E>()
    }

    fn handle(&self, context: &ComponentContext, slot: &mut dyn Any) {
        let message = slot
            .downcast_mut::<Option<E>>()
            .expect("message slot must be an Option")
            .take()
            .expect("unhandled message slot must be Some");

        self.handler.handle(context, message.into());
    }
}
