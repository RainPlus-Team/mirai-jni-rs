package org.rainplus.mirai.loader

import net.mamoe.mirai.utils.ExternalResource
import net.mamoe.mirai.utils.ExternalResource.Companion.toExternalResource

object Helper {
    @JvmStatic
    fun bytesToExternalResource(bytes: ByteArray): ExternalResource {
        return bytes.toExternalResource();
    }
}