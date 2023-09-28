package org.rainplus.mirai.loader

import net.mamoe.mirai.utils.BotConfiguration
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.DisplayName
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.MethodSource

class EnumHelperTest {
    companion object {
        @JvmStatic
        fun enumClasses(): Array<Class<*>> {
            return arrayOf(BotConfiguration.HeartbeatStrategy::class.java, BotConfiguration.MiraiProtocol::class.java)
        }
    }

    @ParameterizedTest
    @MethodSource("enumClasses")
    @DisplayName("Test if ordinal to enum works")
    fun testOrdinalToEnum(clazz: Class<*>) {
        val helper = EnumHelper(clazz as Class<Enum<*>>)
        Assertions.assertEquals(helper.getEnumFromOrdinal(0), (clazz.kotlin.members.first { it.name == "values" }.call() as Array<*>)
            .firstOrNull())
    }
}