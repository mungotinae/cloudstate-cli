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
public @Data class DockerSetup {
    String host;
    String certPath;
    boolean tlsVerify;
    RegistrySetup registry;
}