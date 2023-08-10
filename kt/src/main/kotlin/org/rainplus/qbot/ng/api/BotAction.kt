package org.rainplus.qbot.ng.api

import kotlinx.coroutines.runBlocking
import net.mamoe.mirai.Bot
import net.mamoe.mirai.contact.Contact.Companion.sendImage
import net.mamoe.mirai.utils.ExternalResource

object BotAction {
    @JvmStatic
    fun sendGroupMessage(bot: Bot, group: Long, msg: String) {
        runBlocking {
            bot.getGroup(group)?.sendMessage(msg);
        }
    }

    @JvmStatic
    fun sendGroupImage(bot: Bot, group: Long, img: ExternalResource) {
        runBlocking {
            bot.getGroup(group)?.sendImage(img)
        }
    }
}