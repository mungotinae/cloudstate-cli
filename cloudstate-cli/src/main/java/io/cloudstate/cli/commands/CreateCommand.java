package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(
        name = "create",
        mixinStandardHelpOptions = true,
        description = "Create new user function project")
class CreateCommand implements Runnable {

    @CommandLine.Option(names = {"-n", "--name"}, description = "Set the name of project", required = true)
    String name;

    @CommandLine.Option(names = {"-l", "--language"}, description = "Create the project for the specified language")
    String language;

    @CommandLine.Option(names = {"-r", "--registry"}, description = "Set the the registry address", defaultValue = "docker.io")
    String registry;

    @CommandLine.Option(names = {"-u", "--user"}, description = "Set the Username for the target docker registry")
    String user;

    @CommandLine.Option(names = {"-p", "--password"}, description = "Passphrase", interactive = true)
    char[] password;

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
