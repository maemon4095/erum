// mount時に実行するeffect 定数が戻り値になる。
pub trait MountEffect {
    type Return;
}

pub trait MountEffectHandler<E: MountEffect>: Sized {
    fn handle(&mut self, effect: E) -> E::Return;
}
