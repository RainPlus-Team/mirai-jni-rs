import net.mamoe.mirai.event.Event
//import org.rainplus.mirai.loader.plugin.ConsolePluginDescription // this needs to be fixed for dynamic configuration
import java.io.File
import java.nio.file.Path
import java.nio.file.Paths
import java.util.*
import kotlin.io.path.exists

class Bot {
    public val libPath: Path

    init {
        val props = Properties()
        props.load(Bot::class.java.getResourceAsStream("native.properties"))

        val workDir = System.getProperty("user.dir")
        val codeFile = File(Bot::class.java.protectionDomain.codeSource.location.toURI().path)
        val codeDir = codeFile.absolutePath
        var name = System.mapLibraryName(codeFile.nameWithoutExtension)
        var libPath = Paths.get(workDir.toString(), name)
        if (props["private"].toString() == "yes" || !libPath.exists()) {
            name = System.mapLibraryName(props["filename"].toString())
            libPath = Paths.get(workDir.toString(), name)
            if (!libPath.exists()) {
                libPath = Paths.get(codeDir, name)
            }
        }
        System.load(libPath.toString())
        this.libPath = libPath
    }
    external fun initialize()

    external fun eventListener(className: String, ptr: Long, bot: net.mamoe.mirai.Bot, event: Event)

    //external fun pluginDescription(): ConsolePluginDescription

    external fun pluginInit()
    external fun pluginLoad()
    external fun pluginEnable()
    external fun pluginDisable()
}