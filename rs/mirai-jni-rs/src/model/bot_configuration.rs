use crate::classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY;

use super::JavaObject;

pub struct BotConfiguration<'a> {
    obj: JavaObject<'a>
}

impl<'a> BotConfiguration<'a> {
    pub fn get_heartbeat_strategy(&mut self) -> HeartbeatStrategy {
        let (env, obj) = self.obj.r#use();
        let strategy = env.get_field(obj, "heartbeatStrategy", format!("L{};", BOT_CONFIGURATION_HEARTBEAT_STRATEGY)).unwrap();
        let ordinal = env.call_method(strategy.l().unwrap(), "ordinal", "()I", &[]).unwrap().i().unwrap();
        unsafe { std::mem::transmute(ordinal as i8) }
    }
    pub fn set_heartbeat_strategy(&mut self, strategy: HeartbeatStrategy) {
        let (env, obj) = self.obj.r#use();
        let ordinal = strategy as i32;
        todo!()
    }
}

pub enum HeartbeatStrategy {
    StatHeartbeat,
    Register,
    None
}