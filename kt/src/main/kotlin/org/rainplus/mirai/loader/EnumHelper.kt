package org.rainplus.mirai.loader

class EnumHelper<T : Enum<*>>(clazz: Class<T>) {
    private val map: HashMap<Int, T> = HashMap()

    init {
        clazz.kotlin.members.filter { it.name == "values" }.forEach {
            val values = it.call() as Array<T>?
            if (values != null) {
                for (enum: T in values) {
                    map[enum.ordinal] = enum
                }
            }
        }
    }

    fun getEnumFromOrdinal(ordinal: Int): T {
        return map[ordinal]!!
    }

    companion object {
        private val map: HashMap<String, EnumHelper<Enum<*>>> = HashMap()

        fun getOrCreateEnumHelper(clazz: Class<Enum<*>>): EnumHelper<Enum<*>> {
            return if (map.containsKey(clazz.name)) {
                map[clazz.name]!!
            } else {
                val helper = EnumHelper(clazz)
                map[clazz.name] = helper
                helper
            }
        }
    }
}