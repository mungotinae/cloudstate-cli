# Create Cloudstate Java User Function

## Install CloudState CLI:
```shell script
[sleipnir@sleipnir cloudstate-cli]# 
```

### Create Project from Java template:
```shell script
[sleipnir@sleipnir cloudstate-cli]# ./cloudstate/target/debug/cloudstate --create=shopping-cart --idiom=java
Creating user function project: "shopping-cart"
Using template: "java"
[INFO] Scanning for projects...
[INFO] 
[INFO] ------------------< org.apache.maven:standalone-pom >-------------------
[INFO] Building Maven Stub Project (No POM) 1
[INFO] --------------------------------[ pom ]---------------------------------
[INFO] 
[INFO] >>> maven-archetype-plugin:3.1.1:generate (default-cli) > generate-sources @ standalone-pom >>>
[INFO] 
[INFO] <<< maven-archetype-plugin:3.1.1:generate (default-cli) < generate-sources @ standalone-pom <<<
[INFO] 
[INFO] 
[INFO] --- maven-archetype-plugin:3.1.1:generate (default-cli) @ standalone-pom ---
[INFO] Generating project in Batch mode
[INFO] ----------------------------------------------------------------------------
[INFO] Using following parameters for creating project from Old (1.x) Archetype: maven-archetype-quickstart:1.0
[INFO] ----------------------------------------------------------------------------
[INFO] Parameter: basedir, Value: /home/sleipnir/development/workspace/pessoal/cloudstate-cli
[INFO] Parameter: package, Value: com.example
[INFO] Parameter: groupId, Value: com.example
[INFO] Parameter: artifactId, Value: shopping-cart
[INFO] Parameter: packageName, Value: com.example
[INFO] Parameter: version, Value: 1.0-SNAPSHOT
[INFO] project created from Old (1.x) Archetype in dir: /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  21.675 s
[INFO] Finished at: 2019-10-24T17:03:29-03:00
[INFO] ------------------------------------------------------------------------
Creating Dockerfile
Creating deployment.yml
Creating pom.xml...
Creating resources folder...
Creating application.conf file...
Creating logger file...
Creating proto folder...
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  1402  100  1402    0     0   2686      0 --:--:-- --:--:-- --:--:--  2685
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100   491  100   491    0     0    990      0 --:--:-- --:--:-- --:--:--   991
Successfully changed working directory to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart!
Downloading dependencies...
[INFO] Scanning for projects...
[WARNING] 
[WARNING] Some problems were encountered while building the effective model for com.example:shopping-cart:jar:1.0-SNAPSHOT
[WARNING] 'build.plugins.plugin.version' for org.apache.maven.plugins:maven-compiler-plugin is missing. @ line 24, column 21
[WARNING] 
[WARNING] It is highly recommended to fix these problems because they threaten the stability of your build.
[WARNING] 
[WARNING] For this reason, future Maven versions might no longer support building such malformed projects.
[WARNING] 
[INFO] ------------------------------------------------------------------------
[INFO] Detecting the operating system and CPU architecture
[INFO] ------------------------------------------------------------------------
[INFO] os.detected.name: linux
[INFO] os.detected.arch: x86_64
[INFO] os.detected.version: 3.10
[INFO] os.detected.version.major: 3
[INFO] os.detected.version.minor: 10
[INFO] os.detected.release: centos
[INFO] os.detected.release.version: 7
[INFO] os.detected.release.like.centos: true
[INFO] os.detected.release.like.rhel: true
[INFO] os.detected.release.like.fedora: true
[INFO] os.detected.classifier: linux-x86_64
[INFO] 
[INFO] ---------------------< com.example:shopping-cart >----------------------
[INFO] Building shopping-cart 1.0-SNAPSHOT
[INFO] --------------------------------[ jar ]---------------------------------
[INFO] 
[INFO] --- protobuf-maven-plugin:0.6.1:compile (default) @ shopping-cart ---
[INFO] Compiling 2 proto file(s) to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/target/generated-sources/protobuf/java
[WARNING] PROTOC: example/shoppingcart/shoppingcart.proto:7:1: warning: Import google/api/http.proto but not used.
[libprotobuf WARNING ../../../../../src/google/protobuf/compiler/java/java_file.cc:231] example/shoppingcart/shoppingcart.proto: The file's outer class name, "Shoppingcart", matches the name of one of the types declared inside it when case is ignored. This can cause compilation issues on Windows / MacOS. Please either rename the type or use the java_outer_classname option to specify a different outer class name for the .proto file to be safe.

[INFO] 
[INFO] --- maven-resources-plugin:2.6:resources (default-resources) @ shopping-cart ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] Copying 2 resources
[INFO] Copying 2 resources
[INFO] 
[INFO] --- maven-compiler-plugin:3.1:compile (default-compile) @ shopping-cart ---
[INFO] Changes detected - recompiling the module!
[INFO] Compiling 4 source files to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/target/classes
[INFO] 
[INFO] --- maven-resources-plugin:2.6:testResources (default-testResources) @ shopping-cart ---
[INFO] Using 'UTF-8' encoding to copy filtered resources.
[INFO] skip non existing resourceDirectory /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/src/test/resources
[INFO] 
[INFO] --- maven-compiler-plugin:3.1:testCompile (default-testCompile) @ shopping-cart ---
[INFO] Nothing to compile - all classes are up to date
[INFO] 
[INFO] --- maven-surefire-plugin:2.12.4:test (default-test) @ shopping-cart ---
[INFO] No tests to run.
[INFO] 
[INFO] --- maven-jar-plugin:2.4:jar (default-jar) @ shopping-cart ---
[INFO] Building jar: /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/target/shopping-cart-1.0-SNAPSHOT.jar
[INFO] 
[INFO] --- maven-install-plugin:2.4:install (default-install) @ shopping-cart ---
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/target/shopping-cart-1.0-SNAPSHOT.jar to /root/.m2/repository/com/example/shopping-cart/1.0-SNAPSHOT/shopping-cart-1.0-SNAPSHOT.jar
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/shopping-cart/pom.xml to /root/.m2/repository/com/example/shopping-cart/1.0-SNAPSHOT/shopping-cart-1.0-SNAPSHOT.pom
[INFO] ------------------------------------------------------------------------
[INFO] BUILD SUCCESS
[INFO] ------------------------------------------------------------------------
[INFO] Total time:  3.372 s
[INFO] Finished at: 2019-10-24T17:03:34-03:00
[INFO] ------------------------------------------------------------------------
Project created!
Open editor!
total 8
drwxrwxr-x. 4 root root   30 Out 24 17:03 src
-rw-rw-r--. 1 root root 3659 Out 24 17:03 pom.xml
-rw-rw-r--. 1 root root 1075 Out 24 17:03 Dockerfile
-rw-rw-r--. 1 root root    0 Out 24 17:03 deployment.yml
drwxrwxr-x. 8 root root  175 Out 24 17:03 target
No verbose info
Ok(())

```