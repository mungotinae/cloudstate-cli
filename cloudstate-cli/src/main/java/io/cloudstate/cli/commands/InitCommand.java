package io.cloudstate.cli.commands;

import picocli.CommandLine;

@CommandLine.Command(name = "init", description = "Install if note installed Cloudstate Operator on Kubernetes")
class InitCommand implements Runnable {

    @Override
    public void run() {
        System.out.println("Hello World!");
    }
}
