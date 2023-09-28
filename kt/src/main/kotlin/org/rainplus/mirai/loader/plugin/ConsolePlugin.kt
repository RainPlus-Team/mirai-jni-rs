package org.rainplus.mirai.loader.plugin

import net.mamoe.mirai.console.plugin.jvm.JvmPluginDescription
import net.mamoe.mirai.console.plugin.jvm.KotlinPlugin

object ConsolePlugin : KotlinPlugin(description = JvmPluginDescription("org.rainplus.mirai.loader", "1.0.0", "Mirai-JNI-RS Loader") {
    // TODO: apply description from native lib, might need a new way due to https://github.com/mamoe/mirai/blob/dev/mirai-console/docs/plugin/JVMPlugin.md#%E5%9C%A8%E6%9E%84%E9%80%A0%E5%99%A8%E5%8A%A8%E6%80%81%E6%8F%90%E4%BE%9B
}) {

}