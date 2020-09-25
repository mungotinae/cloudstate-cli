package io.cloudstate.cli.configuration;

import com.fasterxml.jackson.databind.ObjectMapper;

import javax.enterprise.context.Dependent;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

@Dependent
public class ProjectConfigurationStore {
    private final ObjectMapper mapper = new ObjectMapper();
    private final String CONFIG_ROOT_PATH =
            String.format( "%s%s.cloudstate%sprojects", System.getProperty("user.home"), File.separator, File.separator );

    void save(ProjectConfiguration configuration) throws IOException {
        createFileStructure(CONFIG_ROOT_PATH);

        String configAsJson = mapper.writeValueAsString(configuration);
        Path path = Paths.get(CONFIG_ROOT_PATH, configuration.projectName);
        byte[] bytes = configAsJson.getBytes();
        Files.write(path, bytes);
    }

    ProjectConfiguration get(String projectName) throws IOException {
        final byte[] bytes = Files.readAllBytes( Paths.get(getProjectPath(projectName)) );
        return mapper.readValue(bytes, ProjectConfiguration.class);
    }

    private String getProjectPath(String projectName) {
        return String.format("%s%s%s", CONFIG_ROOT_PATH, File.separator, projectName);
    }

    private void createFileStructure(String projectRoot) {
        File f = new File(projectRoot);
        if ((!f.exists())) f.mkdirs();
    }
}
