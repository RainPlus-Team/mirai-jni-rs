val releaseName = if (hasProperty("release")) {"release"} else {"debug"}
rootProject.ext.set("releaseName", releaseName)