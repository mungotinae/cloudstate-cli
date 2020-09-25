package io.cloudstate.cli;

import picocli.CommandLine;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class VersionProvider implements CommandLine.IVersionProvider {

    @Override
    public String[] getVersion() {
        return getActualVersion();
    }

    private String[] getActualVersion() {
        Properties props = loadGitProperties();
        String commit = String.valueOf(props.get("git.commit.id"));
        String version ="1.0.1";
        return new String[]{String.format("CloudState CLI v%s", version), String.format("Commit: %s", commit)};
    }

    private Properties loadGitProperties() {
        Properties props = new Properties();
        ClassLoader classLoader = getClass().getClassLoader();
        InputStream inputStream = classLoader.getResourceAsStream("git.properties");

        try {
            props.load(inputStream);
            return props;
        } catch (IOException e) {
            e.printStackTrace();
            throw new IllegalStateException("Version information could not be retrieved");
        }
    }

}
