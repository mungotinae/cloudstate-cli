package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(name = "scale", description = "Scale user function")
class ScaleCommand implements Runnable {

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
