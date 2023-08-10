package org.rainplus.qbot.ng

import net.mamoe.mirai.utils.ExternalResource
import net.mamoe.mirai.utils.ExternalResource.Companion.toExternalResource

object Helper {
    @JvmStatic
    fun bytesToExternalResource(bytes: Array<Byte>): ExternalResource {
        return bytes.toByteArray().toExternalResource();
    }
}