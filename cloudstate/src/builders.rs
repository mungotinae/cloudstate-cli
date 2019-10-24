extern crate cargo_toml_builder;

use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use cargo_toml_builder::prelude::*;
use std::path::Path;
use std::{env, fs};

pub trait ProjectBuilder {
    fn build(self, name: &str);
}

pub struct RustBuilder;
pub struct JavaBuilder;
pub struct NodeBuilder;
pub struct GoBuilder;
pub struct DotNetBuilder;
pub struct PythonBuilder;
pub struct ScalaBuilder;

impl RustBuilder {
    fn get_cargo_toml(name: &str, version: &str) -> String {
        let log_dep = Dependency::version("log", "0.4.8");
        let log_rs_dep = Dependency::version("log4rs","0.8.3");
        // let log_rs_dep = Dependency::repo("cloudstate", "https://github.com/foo/bar");
        let cloud_state_dep =  Dependency::version("cloudstate", "0.0.1");

        let dependencies = vec![log_dep, log_rs_dep, cloud_state_dep];

        let cargo_toml = CargoToml::builder()
            .name(name)
            .version(version)
            .author(whoami::username().as_ref())
            .dependencies(&dependencies)
            .build();

        cargo_toml.unwrap().to_string()

    }

    fn get_main() -> &'static str {
        let main_rs_contents = r###"
extern crate log;
extern crate log4rs;
extern crate cloudstate;

use log::{info};
use cloudstate::serveless::{CloudState, EntityService};

fn main() {

    // CloudState depends of log4rs to print messages
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
    info!("Starting CloudState Server...");

    let service = EntityService::new()
        .persistence_id("shopping-cart".to_string())
        .protos(vec!["shoppingcart/shoppingcart.proto".to_string(), "shoppingcart/persistence/domain.proto".to_string()])
        .snapshot(1)
        .event_sourced();

    CloudState::new()
        .register_entity_service(
            String::from("com.example.shoppingcart.ShoppingCart"),
            service)
        .start();
}
        "###;
        main_rs_contents
    }

    fn get_dockerfile() -> &'static str {
        let dockerfile_contents = r###"
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS cargo-build

RUN sudo apt-get update && sudo apt-get install -y upx-ucl

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

RUN /usr/bin/upx --brute /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
# ------------------------------------------------------------------------------
# Final Stage
# -------------------- ----------------------------------------------------------
FROM scratch
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp \
    /usr/local/bin/

CMD ["/usr/local/bin/myapp"]
            "###;
        dockerfile_contents
    }
}

impl ProjectBuilder for RustBuilder {

    fn build(self, name: &str) {
        let status = Command::new("cargo")
            .arg("new")
            //.arg("--bin")
            .arg(name)
            .status();

        if status.is_ok() {

            //TODO: Create Dockerfile
            println!("Creating Dockerfile");

            let dockerfile_contents = RustBuilder::get_dockerfile();

            let dockerfile = dockerfile_contents.replace("myapp", name);

            let mut docker_file = File::create(name.to_owned() + "/" + "Dockerfile").unwrap();
            docker_file.write_all(dockerfile.as_ref());

            //TODO: Create deployment.yml
            println!("Creating deployment.yml");
            let mut file = File::create(name.to_owned() + "/" + "deployment.yml");

            //TODO: Add CloudState Crate dependency
            let cargo_contents = RustBuilder::get_cargo_toml(name, "0.0.1");
            let mut cargo_file = File::create(name.to_owned() + "/Cargo.toml").unwrap();
            cargo_file.write_all(cargo_contents.as_ref());

            //TODO: Create main.rs
            let main_rs_contents = RustBuilder::get_main();
            let mut main_file = File::create(name.to_owned() + "/src/main.rs").unwrap();
            main_file.write_all(main_rs_contents.as_ref());

            println!("Project created!");
            Command::new("ls")
                .arg("-ltr")
                .arg(name)
                .spawn()
                .expect("Error during create Rust project");
        } else {
            println!("Error on create Rust project")
        }

    }
}

impl JavaBuilder {

    fn get_dockerfile() -> &'static str {
        let dockerfile_contents = r###"
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS cargo-build

RUN sudo apt-get update && sudo apt-get install -y upx-ucl

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

RUN /usr/bin/upx --brute /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
# ------------------------------------------------------------------------------
# Final Stage
# -------------------- ----------------------------------------------------------
FROM scratch
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp \
    /usr/local/bin/

CMD ["/usr/local/bin/myapp"]
            "###;
        dockerfile_contents
    }

    fn get_pom() -> &'static str {
        let pom_contents = r###"

<project xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xmlns="http://maven.apache.org/POM/4.0.0"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0
                      http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>
    <groupId>com.example</groupId>
    <artifactId>shopping-cart</artifactId>
    <version>1.0-SNAPSHOT</version>

    <packaging>jar</packaging>

    <build>
        <extensions>
            <extension>
                <groupId>kr.motd.maven</groupId>
                <artifactId>os-maven-plugin</artifactId>
                <version>1.6.0</version>
            </extension>
        </extensions>
        <plugins>

            <plugin>
                <groupId>org.apache.maven.plugins</groupId>
                <artifactId>maven-compiler-plugin</artifactId>
                <configuration>
                    <source>1.8</source>
                    <target>1.8</target>
                </configuration>
            </plugin>

            <plugin>
                <groupId>org.xolstice.maven.plugins</groupId>
                <artifactId>protobuf-maven-plugin</artifactId>
                <version>0.6.1</version>
                <configuration>
                    <protocArtifact>com.google.protobuf:protoc:3.9.1:exe:${os.detected.classifier}</protocArtifact>
                </configuration>
                <executions>
                    <execution>
                        <goals>
                            <goal>compile</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>

            <!--
            <plugin>
                <groupId>io.fabric8</groupId>
                <artifactId>docker-maven-plugin</artifactId>
                <version>0.26.1</version>
                <configuration>
                    <images>
                        <image>
                            <build>
                                <name>my-dockerhub-username/my-stateful-service:%l</name>
                                <from>adoptopenjdk/11-jre-hotspot</from>
                                <tags>
                                    <tag>latest</tag>
                                </tags>
                                <assembly>
                                    <descriptorRef>artifact-with-dependencies</descriptorRef>
                                </assembly>
                                <entryPoint>
                                    <arg>java</arg>
                                    <arg>-cp</arg>
                                    <arg>/maven/*</arg>
                                    <arg>com.example.ShoppingCartMain</arg>
                                </entryPoint>
                            </build>
                        </image>
                    </images>
                </configuration>
                <executions>
                    <execution>
                        <id>build-docker-image</id>
                        <phase>package</phase>
                        <goals>
                            <goal>build</goal>
                        </goals>
                    </execution>
                </executions>
            </plugin>
            -->

        </plugins>
    </build>

    <dependencies>
        <dependency>
            <groupId>io.cloudstate</groupId>
            <artifactId>cloudstate-java-support</artifactId>
            <version>0.4.3</version>
        </dependency>
    </dependencies>

    <properties>
        <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    </properties>
</project>
       "###;

        pom_contents
    }

    fn get_main() -> &'static str {
        let main_contents = r###"
package com.example;

import com.example.shoppingcart.Shoppingcart;
import io.cloudstate.javasupport.CloudState;

public class Main {

  public static void main(String... args) {
    new CloudState()
        .registerEventSourcedEntity(
            ShoppingCartEntity.class,
            Shoppingcart.getDescriptor().findServiceByName("ShoppingCart"))
        .start();
  }
}
        "###;

        main_contents
    }

    fn get_application_conf() -> &'static str {
        let application_conf_contents = r###"
cloudstate {
  system {
    loggers = ["akka.event.slf4j.Slf4jLogger"]
    loglevel = "DEBUG"
    logging-filter = "akka.event.slf4j.Slf4jLoggingFilter"
  }
}
        "###;
        application_conf_contents
    }

    fn get_logger_conf() -> &'static str {
        let logger_conf_contents = r###"
org.slf4j.simpleLogger.logFile=System.out
org.slf4j.simpleLogger.cacheOutputStream=false
org.slf4j.simpleLogger.defaultLogLevel=debug
org.slf4j.simpleLogger.log.io.cloudstate.javasupport=debug
org.slf4j.simpleLogger.log.io.cloudstate=debug
org.slf4j.simpleLogger.log.akka=debug
org.slf4j.simpleLogger.showDateTime=true
org.slf4j.simpleLogger.dateTimeFormat=yyyy-MM-dd HH:mm:ss.SSS
org.slf4j.simpleLogger.showThreadName=false
org.slf4j.simpleLogger.showLogName=true
org.slf4j.simpleLogger.showShortLogName=false
org.slf4j.simpleLogger.levelInBrackets=false
org.slf4j.simpleLogger.warnLevelString=WARN
        "###;
        logger_conf_contents
    }

    fn get_shopping_cart_entity() -> &'static str {
        let java_contents = r###"
package com.example;

import com.example.shoppingcart.Shoppingcart;
import com.example.shoppingcart.persistence.Domain;
import com.google.protobuf.Empty;
import io.cloudstate.javasupport.EntityId;
import io.cloudstate.javasupport.eventsourced.*;

import java.util.*;
import java.util.stream.Collectors;

/**
 * An event sourced entity.
 */
@EventSourcedEntity
public class ShoppingCartEntity {
    private final String entityId;
    private final Map<String, Shoppingcart.LineItem> cart = new LinkedHashMap<>();

    public ShoppingCartEntity(@EntityId String entityId) {
        this.entityId = entityId;
    }

    @Snapshot
    public Domain.Cart snapshot() {
        return Domain.Cart.newBuilder()
                .addAllItems(cart.values().stream().map(this::convert).collect(Collectors.toList()))
                .build();
    }

    @SnapshotHandler
    public void handleSnapshot(Domain.Cart cart) {
        this.cart.clear();
        for (Domain.LineItem item : cart.getItemsList()) {
            this.cart.put(item.getProductId(), convert(item));
        }
    }

    @EventHandler
    public void itemAdded(Domain.ItemAdded itemAdded) {
        Shoppingcart.LineItem item = cart.get(itemAdded.getItem().getProductId());
        if (item == null) {
            item = convert(itemAdded.getItem());
        } else {
            item =
                    item.toBuilder()
                            .setQuantity(item.getQuantity() + itemAdded.getItem().getQuantity())
                            .build();
        }
        cart.put(item.getProductId(), item);
    }

    @EventHandler
    public void itemRemoved(Domain.ItemRemoved itemRemoved) {
        cart.remove(itemRemoved.getProductId());
    }

    @CommandHandler
    public Shoppingcart.Cart getCart() {
        return Shoppingcart.Cart.newBuilder().addAllItems(cart.values()).build();
    }

    @CommandHandler
    public Empty addItem(Shoppingcart.AddLineItem item, CommandContext ctx) {
        if (item.getQuantity() <= 0) {
            ctx.fail("Cannot add negative quantity of to item" + item.getProductId());
        }
        ctx.emit(
                Domain.ItemAdded.newBuilder()
                        .setItem(
                                Domain.LineItem.newBuilder()
                                        .setProductId(item.getProductId())
                                        .setName(item.getName())
                                        .setQuantity(item.getQuantity())
                                        .build())
                        .build());
        return Empty.getDefaultInstance();
    }

    @CommandHandler
    public Empty removeItem(Shoppingcart.RemoveLineItem item, CommandContext ctx) {
        if (!cart.containsKey(item.getProductId())) {
            ctx.fail("Cannot remove item " + item.getProductId() + " because it is not in the cart.");
        }
        ctx.emit(Domain.ItemRemoved.newBuilder().setProductId(item.getProductId()).build());
        return Empty.getDefaultInstance();
    }

    private Shoppingcart.LineItem convert(Domain.LineItem item) {
        return Shoppingcart.LineItem.newBuilder()
                .setProductId(item.getProductId())
                .setName(item.getName())
                .setQuantity(item.getQuantity())
                .build();
    }

    private Domain.LineItem convert(Shoppingcart.LineItem item) {
        return Domain.LineItem.newBuilder()
                .setProductId(item.getProductId())
                .setName(item.getName())
                .setQuantity(item.getQuantity())
                .build();
    }
}
        "###;

        java_contents
    }

}

impl ProjectBuilder for JavaBuilder {

    fn build(self, name: &str) {
        // mvn archetype:generate -DgroupId=ToolsQA -DartifactId=DemoMavenProject -DarchetypeArtifactId=maven-archetype-quickstart -DinteractiveMode=false
        let package: &str = "com.example.";

        let status = Command::new("mvn")
            .arg("archetype:generate")
            .arg(format!("{}{}","-DgroupId=", "com.example"))
            .arg(format!("{}{}", "-DartifactId=",name))
            .arg("-DarchetypeArtifactId=maven-archetype-quickstart")
            .arg("-DinteractiveMode=false")
            .status();

        if status.is_ok() {

            //TODO: Create Dockerfile
            println!("Creating Dockerfile");

            let dockerfile_contents = JavaBuilder::get_dockerfile();

            let dockerfile = dockerfile_contents.replace("myapp", name);

            let mut docker_file = File::create(name.to_owned() + "/" + "Dockerfile").unwrap();
            docker_file.write_all(dockerfile.as_ref());

            //TODO: Create deployment.yml
            println!("Creating deployment.yml");
            let mut file = File::create(name.to_owned() + "/" + "deployment.yml");

            // Create Pom
            println!("Creating pom.xml...");
            let pom_contents = JavaBuilder::get_pom();
            let mut pom_file = File::create(name.to_owned() + "/" + "pom.xml").unwrap();
            pom_file.write_all(pom_contents.as_ref());

            // Create resources folder
            println!("Creating resources folder...");
            fs::create_dir_all(name.to_owned() + "/src/main/resources");
            let logger_conf = JavaBuilder::get_logger_conf();
            let application_conf = JavaBuilder::get_application_conf();

            println!("Creating application.conf file...");
            let mut application_conf_file = File::create(name.to_owned() + "/src/main/resources/application.conf").unwrap();
            application_conf_file.write_all(application_conf.as_ref());

            println!("Creating logger file...");
            let mut logger_file = File::create(name.to_owned() + "/src/main/resources/simplelogger.properties").unwrap();
            logger_file.write_all(logger_conf.as_ref());

            println!("Creating proto folder...");
            fs::create_dir_all(name.to_owned() + "/src/main/proto/example/shoppingcart/persistence");

            let download_shopping_proto = Command::new("curl")
                .arg("-o")
                .arg(name.to_owned() + "/src/main/proto/example/shoppingcart/shoppingcart.proto")
                .arg("https://raw.githubusercontent.com/cloudstateio/cloudstate/latest/protocols/example/shoppingcart/shoppingcart.proto")
                .status();

            let download_shopping_proto = Command::new("curl")
                .arg("-o")
                .arg(name.to_owned() + "/src/main/proto/example/shoppingcart/persistence/domain.proto")
                .arg("https://raw.githubusercontent.com/cloudstateio/cloudstate/latest/protocols/example/shoppingcart/persistence/domain.proto")
                .status();

            let java_test_path = format!("/src/test/java/{}", package.replace(".", "/"));
            fs::remove_file(name.to_owned() + java_test_path.as_str() + "AppTest.java");

            // Create Main class
            let java_path = format!("/src/main/java/{}", package.replace(".", "/"));
            fs::remove_file(name.to_owned() + java_path.clone().as_ref() + "App.java");

            let main_contents = JavaBuilder::get_main();
            let mut main_file = File::create(name.to_owned() + java_path.as_str() + "Main.java").unwrap();
            main_file.write_all(main_contents.as_ref());

            // Create ShoppingCartEntity class
            let java_path = format!("/src/main/java/{}", package.replace(".", "/"));
            let java_contents = JavaBuilder::get_shopping_cart_entity();
            let mut shopping_file = File::create(name.to_owned() + java_path.as_str() + "ShoppingCartEntity.java").unwrap();
            shopping_file.write_all(java_contents.as_ref());

            let cd_status = Command::new("cd")
                .arg(name)
                .status();

            let current_path = env::current_dir().unwrap();
            let app_path = format!("{}/{}", current_path.to_str().unwrap(), name);
            let path = Path::new(app_path.as_str());

            if env::set_current_dir(&path).is_ok() {
                println!("Successfully changed working directory to {}!", path.display());
                println!("Downloading dependencies...");
                let install_status = Command::new("mvn")
                    .arg("install")
                    .status();
            }

            println!("Project created!");
            Command::new("ls")
                .arg("-ltr")
                .spawn()
                .expect("Error during create Java project");

        } else {
            println!("Error on create Java project")
        }
    }
}


impl ProjectBuilder for NodeBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for GoBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for DotNetBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for PythonBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for ScalaBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}


