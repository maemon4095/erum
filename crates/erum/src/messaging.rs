mod function;

use crate::ComponentContext;

pub use function::FunctionMessageHandler;

pub trait MessageHandler {
    type Message: ComponentMessage;

    fn handle(&self, context: ComponentContext, message: Self::Message);
}

pub trait MessageEmitter: Sized {
    type WithHandler;

    fn handle<Msg: ComponentMessage, F: Fn(ComponentContext, Msg)>(
        self,
        handler: F,
    ) -> Self::WithHandler {
        self.handle_with(FunctionMessageHandler::new(handler))
    }

    fn handle_with<H: MessageHandler>(self, handler: H) -> Self::WithHandler;
}

pub trait ComponentMessage {
    fn register();
}
