package org.rainplus.mirai.loader.enums

import net.mamoe.mirai.utils.BotConfiguration

object MiraiProtocolMap {
    private val map: HashMap<Int, BotConfiguration.MiraiProtocol> = HashMap();

    init {
        for (protocol: BotConfiguration.MiraiProtocol in BotConfiguration.MiraiProtocol.entries) {
            map[protocol.ordinal] = protocol
        }
    }

    fun fromInt(int: Int): BotConfiguration.MiraiProtocol {
        return map[int]!!
    }
}