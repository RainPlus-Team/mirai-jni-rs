package org.rainplus.mirai.loader

import Bot

private var _bot: Bot? = null;
val bot: Bot
    get() = _bot!!

fun main(args: Array<String>) {
    println("== Mirai-JNI-Loader ==")
    println("Loading bot core...")
    _bot = Bot()
    bot.initialize()
}
