use crate::messaging::handler_map::MessageHandlerMap;

use super::MessageHandler;
use std::rc::Rc;

pub trait ComponentMessage: 'static + Sized {
    fn register<H: MessageHandler<Message = Self>>(ctx: &mut MessageHandlerRegistrationContext<H>);
}

pub struct MessageHandlerRegistrationContext<H: MessageHandler> {
    pub(super) map: MessageHandlerMap,
    pub(super) handler: Rc<H>,
}

impl<H: 'static + MessageHandler> MessageHandlerRegistrationContext<H> {
    pub fn register<T: 'static + Into<H::Message>>(&mut self) {
        self.map = self.map.registered::<H, T>(self.handler.clone());
    }
}
