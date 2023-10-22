package org.rainplus.mirai.loader.plugin;

import net.mamoe.mirai.console.plugin.description.PluginDependency;
import org.jetbrains.annotations.Nullable;

import java.util.HashSet;
import java.util.Set;

public class ConsolePluginDescription {
    public final String id;
    public final String version;
    public final String name;
    public String author = "";
    public String info = "";

    public Set<PluginDependency> dependencies = new HashSet<>();

    public ConsolePluginDescription(String id, String version, String name) {
        this.id = id;
        this.version = version;
        this.name = name;
    }

    public void dependsOn(String id, @Nullable String versionRequirement, boolean isOptional) {
        dependencies.add(new PluginDependency(id, versionRequirement, isOptional));
    }
}
