use const_format::concatcp;

pub const CLASS: &str = "java/lang/Class";
pub const ENUM: &str = "java/lang/Enum";
pub const STRING: &str = "java/lang/String";

pub const BOT_FACTORY: &str = "net/mamoe/mirai/BotFactory";
pub const BOT_AUTHORIZATION: &str = "net/mamoe/mirai/auth/BotAuthorization";

pub const BOT_CONFIGURATION: &str = "net/mamoe/mirai/utils/BotConfiguration";
pub const BOT_CONFIGURATION_HEARTBEAT_STRATEGY: &str = concatcp!(BOT_CONFIGURATION, "$HeartbeatStrategy");
pub const BOT_CONFIGURATION_MIRAI_PROTOCOL: &str = concatcp!(BOT_CONFIGURATION, "$MiraiProtocol");
pub const BOT_CONFIGURATION_CONTACT_LIST_CACHE: &str = concatcp!(BOT_CONFIGURATION, "$ContactListCache");

pub const BOT: &str = "net/mamoe/mirai/Bot";

pub const GROUP: &str = "net/mamoe/mirai/contact/Group";
pub const MEMBER: &str = "net/mamoe/mirai/contact/Member";

pub const MESSAGE_CHAIN: &str = "net/mamoe/mirai/message/data/MessageChain";

pub const BOT_ONLINE_EVENT: &str = "net/mamoe/mirai/event/events/BotOnlineEvent";
pub const BOT_OFFLINE_EVENT: &str = "net/mamoe/mirai/event/events/BotOfflineEvent";
pub const GROUP_MESSAGE_EVENT: &str = "net/mamoe/mirai/event/events/GroupMessageEvent";

pub const EXTERNAL_RESOURCE: &str = "net/mamoe/mirai/utils/ExternalResource";
pub const LOGIN_SOLVER: &str = "net/mamoe/mirai/utils/LoginSolver";

pub const HELPER: &str = "org/rainplus/mirai/loader/Helper";
pub const ENUM_HELPER: &str = "org/rainplus/mirai/loader/EnumHelper";

pub const BOT_LOGIN: &str = "org/rainplus/mirai/loader/api/BotLogin";
pub const BOT_ACTION: &str = "org/rainplus/mirai/loader/api/BotAction";
pub const BOT_EVENT: &str = "org/rainplus/mirai/loader/api/BotEvent";