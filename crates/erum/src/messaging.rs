mod function;
mod handler_map;
mod message;

use crate::ComponentContext;

pub use function::FunctionMessageHandler;
pub use handler_map::MessageHandlerMap;
pub use message::ComponentMessage;

pub trait MessageHandler: 'static {
    type Message: ComponentMessage;

    fn handle(&self, context: &ComponentContext, message: Self::Message);
}

pub trait MessageEmitter: Sized {
    type WithHandler;

    fn handle<Msg: ComponentMessage, F: 'static + Fn(&ComponentContext, Msg)>(
        self,
        handler: F,
    ) -> Self::WithHandler {
        self.handle_with(FunctionMessageHandler::new(handler))
    }

    fn handle_with<H: MessageHandler>(self, handler: H) -> Self::WithHandler;
}
