package org.rainplus.mirai.loader

import Bot

private var _bot: Bot? = null;
val bot: Bot
    get() = _bot!!

fun main(args: Array<String>) {
    println("== Mirai-JNI-Loader ==")
    println("Loading bot core...")

    try {
        _bot = Bot()
    } catch(ex: UnsatisfiedLinkError) {
        println("Unable to locate or load the bot core.")
        kotlin.system.exitProcess(-1)
    }

    println("Using core ${bot.libPath}")

    try {
        bot.initialize()
    } catch(ex: UnsatisfiedLinkError) {
        println("This core cannot be used as an standalone application.")
        kotlin.system.exitProcess(-1)
    }
}
