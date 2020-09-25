package io.cloudstate.cli.configuration;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import javax.inject.Inject;
import java.io.IOException;

@QuarkusTest
public class ProjectConfigurationStoreTest {

    @Inject
    ProjectConfigurationStore store;

    @Test
    public void testWriteConfigurationToFile() throws IOException {
        ProjectConfiguration config = ProjectConfiguration
                .builder()
                .projectName("cloudstate-cli")
                .version("1.0.1")
                .gitUrl("https://github.com/sleipnir/cloudstate-cli")
                .gitUser("sleipnir")
                .language(Language.JAVA)
                .cloudstate(
                        CloudStateSetup.builder()
                                .protocolVersion("0.5.1")
                                .operatorNamespaceDefault("cloudstate")
                                .userFunctionNamespaceDefault("default")
                                .proxyImplementation(ProxyImplementation.Akka_Reference_Implementation)
                                .operatorImplementation(OperatorImplementation.Go_Reference_Implementation)
                                .build()
                )
                .docker(
                        DockerSetup.builder()
                                .host("unix:///var/run/docker.sock")
                                .tlsVerify(false)
                                .registry(
                                        RegistrySetup.builder()
                                                .username("sleipnir")
                                                .email("sleipnir@bsd.com.br")
                                                .repository("sleipnir/cloudstate-clit:latest")
                                                .registryDocker("docker.io")
                                                .build()
                                )
                                .build())
                .build();

        store.save(config);
        Assertions.assertEquals(config.gitUser, store.get(config.projectName).gitUser);
    }
}
