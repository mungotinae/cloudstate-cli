package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(name = "run", description = "Run user function")
class RunCommand implements Runnable {

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
