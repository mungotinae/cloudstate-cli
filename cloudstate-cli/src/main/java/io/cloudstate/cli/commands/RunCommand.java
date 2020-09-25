package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(
        name = "run",
        mixinStandardHelpOptions = true,
        description = "Run user function")
class RunCommand implements Runnable {

    @CommandLine.Option(names = {"-l", "--local"}, description = "Running user function in dev mode using Docker", defaultValue = "noLocal")
    String local;

    @CommandLine.Option(names = {"-n", "--namespace"}, description = "Use specified namespace", defaultValue = "default")
    String namespace;

    @CommandLine.Option(names = {"-p", "--proxy-port"}, description = "Set Proxy port", defaultValue = "9000")
    int proxyPort;

    @CommandLine.Option(names = {"-i", "--proxy-image"}, description = "Set Proxy image")
    String proxyImage;

    @CommandLine.Option(names = {"-P", "--user-function-port"}, description = "Set User Function port", defaultValue = "8080")
    int functionPort;

    @CommandLine.Option(names = {"-I", "--user-function-image"}, description = "Set User Function image")
    String functionImage;

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
