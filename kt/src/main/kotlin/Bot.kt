import net.mamoe.mirai.event.Event

class Bot {
    init {
        System.loadLibrary("bot")
    }
    external fun initialize()

    external fun eventListener(className: String, ptr: Long, bot: net.mamoe.mirai.Bot, event: Event)
}