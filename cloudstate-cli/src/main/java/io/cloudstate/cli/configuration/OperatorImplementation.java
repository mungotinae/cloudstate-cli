package io.cloudstate.cli.configuration;

public enum OperatorImplementation {
    Go_Reference_Implementation("CloudState_Go");

    private final String operator;

    OperatorImplementation(String operator) {
        this.operator = operator;
    }
}
