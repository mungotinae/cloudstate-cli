package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(name = "create", description = "Create new user function project")
class CreateCommand implements Runnable {

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
