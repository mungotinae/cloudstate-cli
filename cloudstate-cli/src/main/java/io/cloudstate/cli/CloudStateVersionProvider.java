package io.cloudstate.cli;

import picocli.CommandLine;

import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class CloudStateVersionProvider implements CommandLine.IVersionProvider {

    @Override
    public String[] getVersion() {
        return getActualVersion();
    }

    private String[] getActualVersion() {
        Properties props = loadGitProperties();
        String commit = String.valueOf(props.get("git.commit.id"));
        String release = String.valueOf(props.get("git.commit.message.full"));
        return new String[]{"CloudState CLI", String.format("%s. Commit: %s", release, commit)};
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
