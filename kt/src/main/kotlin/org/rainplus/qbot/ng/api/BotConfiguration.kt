package org.rainplus.qbot.ng.api

import net.mamoe.mirai.Bot
import net.mamoe.mirai.utils.BotConfiguration

object BotConfiguration {
    @JvmStatic
    fun setProtocol(bot: Bot, protocol: Int) {
        bot.configuration.protocol = BotConfiguration.MiraiProtocol.entries.toTypedArray()[protocol]
    }

    @JvmStatic
    fun setFileBasedDeviceInfo(bot: Bot, file: String) {
        bot.configuration.fileBasedDeviceInfo(file)
    }
}