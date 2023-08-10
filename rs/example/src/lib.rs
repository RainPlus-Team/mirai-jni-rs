use std::env;

use mirai_jni_rs::{bot_initialize, event::group_message::{GroupMessageHandler, GroupMessageEvent}, bot::{event::EventHandler, Bot, login::BotAuthorization}};

#[bot_initialize]
pub fn Bot_initialize() {
    println!("Initializing...");
    println!("This example uses QR code login by default.");
    let mut bot = Bot::new(&mut env, env::var("QQ_ID").expect("cannot get qq id from env").parse().expect("not a valid qq id"), BotAuthorization::QRCode);
    println!("Bot created");
    println!("Setting protocol...");
    bot.protocol(mirai_jni_rs::bot::login::Protocol::MacOS).expect("failed to set protocol");
    println!("Setting up device info...");
    bot.file_based_device_info("device.json").expect("failed to set device info");
    println!("Logging in...");
    bot.login().expect("Bot not logging in");
    let _ = bot.register_event(GroupMessageHandler::new(handle_msg));
}

fn handle_msg(mut bot: Bot, mut e: GroupMessageEvent) {
    let group = e.from_group();
    let str = e.msg_str();
    let mut sender = e.sender();
    let mut sender_name = sender.name_card();
    if sender_name.is_empty() {
        sender_name = sender.nick();
    }
    if str == "Mirai，你好" {
        let _ = bot.send_message(group, format!("你好，{}，很高兴见到你", sender_name));
    }
}