package io.cloudstate.cli.configuration;

import io.quarkus.runtime.annotations.RegisterForReflection;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Builder
@NoArgsConstructor
@AllArgsConstructor
@RegisterForReflection
public @Data class ProjectConfiguration {
    String projectName;
    String version;
    Language language;
    String gitUrl;
    String gitUser;
    DockerSetup docker;
    CloudStateSetup cloudstate;
}
