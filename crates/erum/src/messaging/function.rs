use std::marker::PhantomData;

use crate::{
    ComponentContext,
    messaging::{ComponentMessage, MessageHandler},
};

pub struct FunctionMessageHandler<Msg: ComponentMessage, F: Fn(ComponentContext, Msg)> {
    handler: F,
    _phantom: PhantomData<Msg>,
}

impl<Msg: ComponentMessage, F: Fn(ComponentContext, Msg)> FunctionMessageHandler<Msg, F> {
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _phantom: PhantomData,
        }
    }
}

impl<Msg: ComponentMessage, F: Fn(ComponentContext, Msg)> MessageHandler
    for FunctionMessageHandler<Msg, F>
{
    type Message = Msg;

    fn handle(&self, context: ComponentContext, message: Self::Message) {
        (self.handler)(context, message)
    }
}
