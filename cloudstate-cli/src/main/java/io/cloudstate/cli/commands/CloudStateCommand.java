package io.cloudstate.cli.commands;

import io.cloudstate.cli.CloudStateVersionProvider;
import io.quarkus.picocli.runtime.annotations.TopCommand;
import picocli.CommandLine;

@TopCommand
@CommandLine.Command(
        mixinStandardHelpOptions = true,
        versionProvider = CloudStateVersionProvider.class,
        subcommands = {
                InitCommand.class,
                CreateCommand.class,
                RunCommand.class,
                ScaleCommand.class,
                LogCommand.class
        })
public class CloudStateCommand {}
