package org.rainplus.qbot.ng.api

import kotlinx.coroutines.runBlocking
import net.mamoe.mirai.Bot

object BotAction {
    @JvmStatic
    fun sendMessage(bot: Bot, qq: Long, msg: String) {
        runBlocking {
            bot.getGroup(qq)?.sendMessage(msg);
        }
    }
}