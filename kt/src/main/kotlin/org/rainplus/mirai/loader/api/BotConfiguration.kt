package org.rainplus.mirai.loader.api

import net.mamoe.mirai.Bot
import net.mamoe.mirai.utils.BotConfiguration

object BotConfiguration {
    @JvmStatic
    fun setFileBasedDeviceInfo(bot: Bot, file: String) {
        bot.configuration.fileBasedDeviceInfo(file)
    }
}