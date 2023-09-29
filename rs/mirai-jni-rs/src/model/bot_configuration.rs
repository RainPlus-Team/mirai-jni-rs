use jni::objects::JValueGen;

use crate::classes::{BOT_CONFIGURATION_HEARTBEAT_STRATEGY, self};

use super::JavaObject;

pub struct BotConfiguration<'a> {
    obj: JavaObject<'a>
}

impl<'a> BotConfiguration<'a> {
    simple_getter!(get_heartbeat_period_millis, i64, "heartbeatPeriodMillis", j, "J");
    simple_setter!(set_heartbeat_period_millis, i64, "heartbeatPeriodMillis", Long, "J");

    pub fn get_heartbeat_strategy(&mut self) -> HeartbeatStrategy {
        let (env, obj) = self.obj.r#use();
        let strategy = env.get_field(obj, "heartbeatStrategy", format!("L{};", BOT_CONFIGURATION_HEARTBEAT_STRATEGY)).unwrap();
        let ordinal = env.call_method(strategy.l().unwrap(), "ordinal", "()I", &[]).unwrap().i().unwrap();
        unsafe { std::mem::transmute(ordinal as i8) }
    }
    pub fn set_heartbeat_strategy(&mut self, strategy: HeartbeatStrategy) {
        let (env, obj) = self.obj.r#use();
        let ordinal = strategy as i32;
        let heartbeat_strategy_enum = env.find_class(classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY).unwrap();
        let enum_helper = env.find_class(classes::ENUM_HELPER).unwrap();
        let helper = env.call_static_method(enum_helper, "getOrCreateEnumHelper", format!("(L{};)L{};", classes::CLASS, classes::ENUM_HELPER), &[JValueGen::Object(&heartbeat_strategy_enum)]).unwrap().l().unwrap();
        let r#enum = env.call_method(helper, "getEnumFromOrdinal", format!("(I)L{};", classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY), &[JValueGen::Int(ordinal)]).unwrap().l().unwrap();
        env.set_field(obj, "heartbeatStrategy", format!("L{};", classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY), JValueGen::Object(&r#enum)).unwrap();
    }
}

pub enum HeartbeatStrategy {
    StatHeartbeat,
    Register,
    None
}

impl<'a> From<JavaObject<'a>> for BotConfiguration<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        BotConfiguration { obj: value }
    }
}