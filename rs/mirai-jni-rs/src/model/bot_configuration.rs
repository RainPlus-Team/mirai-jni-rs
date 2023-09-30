use jni::objects::JValueGen;

use crate::classes;

use super::{JavaObject, utils::LoginSolver};

pub struct BotConfiguration<'a> {
    obj: JavaObject<'a>
}

impl<'a> BotConfiguration<'a> {
    simple_getter!(heartbeat_period_millis, i64, "heartbeatPeriodMillis", j, "J");
    simple_setter!(set_heartbeat_period_millis, i64, "heartbeatPeriodMillis", Long, "J");

    simple_getter!(stat_heartbeat_period_millis, i64, "statHeartbeatPeriodMillis", j, "J");
    simple_setter!(set_stat_heartbeat_period_millis, i64, "statHeartbeatPeriodMillis", Long, "J");

    pub fn heartbeat_strategy(&mut self) -> HeartbeatStrategy {
        let (env, obj) = self.obj.r#use();
        let strategy = env.get_field(obj, "heartbeatStrategy", format!("L{};", classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY)).unwrap();
        let ordinal = env.call_method(strategy.l().unwrap(), "ordinal", "()I", &[]).unwrap().i().unwrap();
        unsafe { std::mem::transmute(ordinal as i8) }
    }
    pub fn set_heartbeat_strategy(&mut self, strategy: HeartbeatStrategy) {
        let (env, obj) = self.obj.r#use();
        let ordinal = strategy as i32;
        let heartbeat_strategy_enum = env.find_class(classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY).unwrap();
        let enum_helper = env.find_class(classes::ENUM_HELPER).unwrap();
        let helper = env.call_static_method(enum_helper, "getOrCreateEnumHelper", format!("(L{};)L{};", classes::CLASS, classes::ENUM_HELPER), &[JValueGen::Object(&heartbeat_strategy_enum)]).unwrap().l().unwrap();
        let r#enum = env.call_method(helper, "getEnumFromOrdinal", format!("(I)L{};", classes::ENUM), &[JValueGen::Int(ordinal)]).unwrap().l().unwrap();
        env.set_field(obj, "heartbeatStrategy", format!("L{};", classes::BOT_CONFIGURATION_HEARTBEAT_STRATEGY), JValueGen::Object(&r#enum)).unwrap();
    }

    simple_getter!(heartbeat_timeout_millis, i64, "heartbeatTimeoutMillis", j, "J");
    simple_setter!(set_heartbeat_timeout_millis, i64, "heartbeatTimeoutMillis", Long, "J");

    // not sure should we include deprecated fields

    simple_getter!(reconnection_retry_times, i32, "reconnectionRetryTimes", i, "I");
    simple_setter!(set_reconnection_retry_times, i32, "reconnectionRetryTimes", Int, "I");

    simple_getter!(auto_reconnect_on_force_offline, bool, "autoReconnectOnForceOffline", z, "Z");
    simple_setter!(set_auto_reconnect_on_force_offline, bool, "autoReconnectOnForceOffline", Bool, "Z", |_, _, val| {
        JValueGen::Bool(val as u8)
    });

    simple_getter!(login_solver, LoginSolver, "loginSolver", l, format!("L{};", classes::LOGIN_SOLVER), |env, _, val| {
        JavaObject::new(env, &val).into()
    });
    pub fn set_login_solver(&mut self, login_solver: LoginSolver) {
        let (env, obj) = self.obj.r#use();
        env.set_field(obj, "loginSolver", format!("L{};", classes::LOGIN_SOLVER), JValueGen::Object(&(<LoginSolver<'_> as Into<JavaObject<'_>>>::into(login_solver)).object)).unwrap();
    }

    pub fn protocol(&mut self) -> Protocol {
        let (env, obj) = self.obj.r#use();
        let strategy = env.get_field(obj, "protocol", format!("L{};", classes::BOT_CONFIGURATION_MIRAI_PROTOCOL)).unwrap();
        let ordinal = env.call_method(strategy.l().unwrap(), "ordinal", "()I", &[]).unwrap().i().unwrap();
        unsafe { std::mem::transmute(ordinal as i8) }
    }
    pub fn set_protocol(&mut self, protocol: Protocol) {
        let (env, obj) = self.obj.r#use();
        let ordinal = protocol as i32;
        let protocol_enum = env.find_class(classes::BOT_CONFIGURATION_MIRAI_PROTOCOL).unwrap();
        let enum_helper = env.find_class(classes::ENUM_HELPER).unwrap();
        let helper = env.call_static_method(enum_helper, "getOrCreateEnumHelper", format!("(L{};)L{};", classes::CLASS, classes::ENUM_HELPER), &[JValueGen::Object(&protocol_enum)]).unwrap().l().unwrap();
        let r#enum = env.call_method(helper, "getEnumFromOrdinal", format!("(I)L{};", classes::ENUM), &[JValueGen::Int(ordinal)]).unwrap().l().unwrap();
        env.set_field(obj, "protocol", format!("L{};", classes::BOT_CONFIGURATION_MIRAI_PROTOCOL), JValueGen::Object(&r#enum)).unwrap();
    } // TODO: remove duplicated code

    simple_getter!(highway_upload_coroutine_count, i32, "highwayUploadCoroutineCount", i, "I");
    simple_setter!(set_highway_upload_coroutine_count, i32, "highwayUploadCoroutineCount", Int, "I");

    // TODO: Device info

    // TODO: Logger suppliers

    simple_getter!(show_verbose_event_log, bool, "isShowingVerboseEventLog", z, "Z");
    simple_setter!(set_show_verbose_event_log, bool, "isShowingVerboseEventLog", Bool, "Z", |_, _, val| {
        JValueGen::Bool(val as u8)
    });

    // TODO: Contact list cache

    simple_getter!(login_cache_enabled, bool, "loginCacheEnabled", z, "Z");
    simple_setter!(set_login_cache_enabled, bool, "loginCacheEnabled", Bool, "Z", |_, _, val| {
        JValueGen::Bool(val as u8)
    });
} // TODO: all callable methods

#[derive(Debug)]
pub enum HeartbeatStrategy {
    StatHeartbeat,
    Register,
    None
}

#[derive(Debug)]
pub enum Protocol {
    AndroidPhone,
    AndroidPad,
    AndroidWatch,
    IPad,
    MacOS
}

impl<'a> From<JavaObject<'a>> for BotConfiguration<'a> {
    fn from(value: JavaObject<'a>) -> Self {
        BotConfiguration { obj: value }
    }
}