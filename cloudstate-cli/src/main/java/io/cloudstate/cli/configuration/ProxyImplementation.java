package io.cloudstate.cli.configuration;

public enum ProxyImplementation {
    Akka_Reference_Implementation("Akka");

    private final String implementation;

    ProxyImplementation(String implementation) {
        this.implementation = implementation;
    }
}
