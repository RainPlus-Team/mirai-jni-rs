package org.rainplus.mirai.loader.enums

import net.mamoe.mirai.utils.BotConfiguration

object HeartbeatStrategyMap {
    private val map: HashMap<Int, BotConfiguration.HeartbeatStrategy> = HashMap();

    init {
        for (strategy: BotConfiguration.HeartbeatStrategy in BotConfiguration.HeartbeatStrategy.entries) {
            map[strategy.ordinal] = strategy
        }
    }

    fun fromInt(int: Int): BotConfiguration.HeartbeatStrategy {
        return map[int]!!
    }
}