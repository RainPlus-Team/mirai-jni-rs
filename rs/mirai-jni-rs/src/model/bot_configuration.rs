use jni::objects::{JValueGen, JObject};

use crate::classes;

use super::{JavaObject, utils::LoginSolver, env::MiraiEnv};

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

    pub fn disable_account_secrets(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "disableAccountSecretes", "()V", &[]).unwrap();
    }

    pub fn login_solver(&mut self) -> Option<LoginSolver> {
        let (env, obj) = self.obj.r#use();
        let val = env.get_field(obj, "loginSolver", format!("L{};", classes::LOGIN_SOLVER)).unwrap().l().unwrap();
        if val.is_null() {
            None
        } else {
            Some(JavaObject::new(env, &val).into())
        }
    }
    pub fn set_login_solver(&mut self, login_solver: Option<LoginSolver>) {
        let (env, obj) = self.obj.r#use();
        let val = if let Some(login_solver) = login_solver {
            (<LoginSolver<'_> as Into<JavaObject<'_>>>::into(login_solver)).object
        } else {
            JObject::null()
        };
        env.set_field(obj, "loginSolver", format!("L{};", classes::LOGIN_SOLVER), JValueGen::Object(&val)).unwrap();
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

    pub fn random_device_info(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "randomDeviceInfo", "()V", &[]).unwrap();
    }
    pub fn load_device_info_json<J>(&mut self, json: J)
        where J: AsRef<str>
    {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "loadDeviceInfoJson", format!("(L{};)V", classes::STRING), &[
            JValueGen::Object(jni_str!(env, json))
        ]).unwrap();
    }
    // TODO: Device info getter/setter

    // TODO: Logger suppliers

    pub fn no_network_log(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "noNetworkLog", "()V", &[]).unwrap();
    }

    pub fn no_bot_log(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "noBotLog", "()V", &[]).unwrap();
    }

    simple_getter!(show_verbose_event_log, bool, "isShowingVerboseEventLog", z, "Z");
    simple_setter!(set_show_verbose_event_log, bool, "isShowingVerboseEventLog", Bool, "Z", |_, _, val| {
        JValueGen::Bool(val as u8)
    });

    // TODO: Contact list cache

    pub fn disable_contact_cache(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "disableContactCache", "()V", &[]).unwrap();
    }

    pub fn enable_contact_cache(&mut self) {
        let (env, obj) = self.obj.r#use();
        env.call_method(obj, "enableContactCache", "()V", &[]).unwrap();
    }

    simple_getter!(login_cache_enabled, bool, "loginCacheEnabled", z, "Z");
    simple_setter!(set_login_cache_enabled, bool, "loginCacheEnabled", Bool, "Z", |_, _, val| {
        JValueGen::Bool(val as u8)
    });

    pub fn default(env: &'a MiraiEnv<'a>) -> Self {
        let mut env = env.get_env();
        let class = env.find_class(classes::BOT_CONFIGURATION).unwrap();
        let obj = env.get_static_field(class, "Default", format!("L{};", classes::BOT_CONFIGURATION)).unwrap().l().unwrap();
        JavaObject::new(&env, &obj).into()
    }
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