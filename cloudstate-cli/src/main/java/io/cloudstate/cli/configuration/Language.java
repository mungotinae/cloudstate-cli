package io.cloudstate.cli.configuration;

public enum Language {
    BALLERINA("ballerina"),
    CAMEL("camel"),
    CLOJURE("clojure"),
    CPLUSPLUS("c++"),
    CSHARP("c#"),
    DART("dart"),
    ELIXIR("elixir"),
    GO("go"),
    GROOVY("groovy"),
    JAVA("java"),
    JRUBY("jruby"),
    KOTLIN("kotlin"),
    LUA("lua"),
    NODEJS("nodejs"),
    PYTHON("python"),
    RUBY("ruby"),
    RUST("rust"),
    SCALA("scala"),
    SPRINGBOOT("springboot"),
    SMALLTALK("smalltalk"),
    SWIFT("swift"),
    OBJECTIVEC("objective-c");

    private final String language;

    Language(String language) {
        this.language = language;
    }

    public String getLanguage() {
        return language;
    }

    @Override
    public String toString() {
        return getLanguage();
    }
}
