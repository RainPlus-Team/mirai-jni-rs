package org.rainplus.qbot.ng

import Bot

private var _bot: Bot? = null;
val bot: Bot
    get() = _bot!!

fun main(args: Array<String>) {
    println("== RainBot-NG ==")
    println("Loading bot core...")
    _bot = Bot()
    bot.initialize()
}
