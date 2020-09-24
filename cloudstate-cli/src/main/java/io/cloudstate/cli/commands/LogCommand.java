package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(name = "logs", description = "Show logs of user function")
class LogCommand implements Runnable {

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
