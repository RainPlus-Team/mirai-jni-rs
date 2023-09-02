package org.rainplus.mirai.loader.api

import net.mamoe.mirai.Bot
import net.mamoe.mirai.event.Event
import org.rainplus.mirai.loader.bot as mainBot

object BotEvent {
    @JvmStatic
    fun registerEvent(bot: Bot, cls: Class<Event>, ptr: Long) {
        bot.eventChannel.subscribeAlways(cls) { event ->
            mainBot.eventListener(cls.name.replace(".", "/"), ptr, bot, event)
        }
    }
}