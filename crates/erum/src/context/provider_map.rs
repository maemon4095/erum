use crate::{context::ContextDefinition, signal::SignalListener};

// もうちょっと主張の強い作りにした方がよいか？signalの値がrender中に書き変わらないようにevent-threadのキューに値を乗せるだけにしたほうがよいかも？
#[derive(Debug, Clone)]
pub struct ContextProviderMap {
    // typeでmapしたあとインスタンスでmapする。
}

impl ContextProviderMap {
    pub fn provided(&self) -> Self {
        todo!()
    }

    pub fn try_register<C: ContextDefinition, L: SignalListener<C::Item, C::Hint>>(
        &self,
        context: C,
        listener: L,
    ) -> Result<(), L> {
        todo!()
    }
}
