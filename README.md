[![Build Status](https://travis-ci.com/sleipnir/cloudstate-cli.svg?branch=master)](https://travis-ci.com/sleipnir/cloudstate-cli)

# CloudState  
  
Is a specification, protocol, and reference implementation for providing distributed state management patterns suitable for Serverless computing. The current supported and envisioned patterns include:

-   Event Sourcing
-   Conflict-Free Replicated Data Types ([CRDTs](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type))
-   Key-Value storage
-   P2P messaging
-   CQRS read side projections

CloudState is polyglot, which means that services can be written in any language that supports gRPC, and with language specific libraries provided that allow idiomatic use of the patterns in each language. CloudState can be used either by itself, in combination with a Service Mesh, or it is in envisioned that it will be integrated with other Serverless technologies such as [Knative](https://knative.dev).

Read more about the design, architecture, techniques, and technologies behind CloudState in [the documentation](https://cloudstate.io/docs/).

**CloudState CLI** 

Is a *lightweight*, *fast*, *opinative*, *community driven* and fun client that lets you go from zero to production with Cloudstate *Stateful Serveless* functions in minutes. Cloudstate CLI provides:

-   Simple command API.
-   Autocomplete support.
-   Whole life cycle for creating and running your CLI managed Serveless Function.
-   Uncoupled. At any point in your project you can stop using the CLI and use other tools like 'kubectl'.
  
## Index
 1. [Prerequisites](#Prerequisites)
 2. [Installation](#Installation)
 3. [Usage](#Usage)
 4. [CloudState and CloudState CLI language support](#Languages)

## Prerequisites:
CloudState CLI relies on a number of command line tools such as:

<br/>

 * Curl
 * [Docker](https://www.docker.com/)
 * [Minikube](https://github.com/kubernetes/minikube) (for tests)
 * Language specific build tools like [maven](https://maven.apache.org/) for Java, [Cargo](https://doc.rust-lang.org/cargo/) for Rust, etc..
  
## Installation:  
```  
[cloudstate]#  curl https://raw.githubusercontent.com/sleipnir/cloudstate-cli/master/bin/cloudstate-installer.sh | sh -  
```  

> In Mac OSX environments in addition to the script method you can use Homebrew installation:
> ```
> [cloudstate]# brew tap sleipnir/cloudstate-cli
> [cloudstate]# brew install cloudstate-cli
> ```
> Note: The brew installation method so far does not provide shell completions capabilities.
<br/>

## Usage:  
<br/>

```  
[cloudstate]# cloudstate --help
Cloudstate 0.5.0
Adriano Santos <sleipnir@bsd.com.br>
CloudState CLI

USAGE:
 cloudstate [FLAGS] [SUBCOMMAND]

FLAGS:
     --check            Test dependencies
 -h, --help             Prints help information
 -l, --list-profiles    List all profiles supported
     --upgrade          Update CloudState CLI version
 -V, --version          Prints version information

SUBCOMMANDS:
 build        Build project with template specified. Requires path. Example. cloudstate build --path=.
 create       Create a new user function project from template. Example. cloudstate create --name=shopping-cart
              --profile=java --registry=docker.io --tag=1.0.1
 deploy       Deploy user function with CloudState sidecar in K8s environment
 destroy      Destroy CloudState namespace and others resources
 help         Prints this message or the help of the given subcommand(s)
 init         Initialize a CloudState k8s namespace/operator
 logs         Show application logs
 provision    Shortcut to build, push and deploy. Example. cloudstate provision -p . -t 1.0.1 -n production
 run          Running user function & cloudstate proxy in Docker

```  
<br/>

**Or in Specific Command**
```  
[cloudstate]# cloudstate build --help
cloudstate-build 0.5.0
Adriano Santos <sleipnir@bsd.com.br>
Build project with template specified. Requires path. Example cloudstate --path=.

USAGE:
  cloudstate build [FLAGS] [OPTIONS]

FLAGS:
  -h, --help       Prints help information
      --push       Push container image in repository
  -V, --version    Prints version information

OPTIONS:
      --path <path>    Set the path of build
  -t, --tag <tag>      Used in conjunction with 'create' and/or 'build'. Set version of user function. Used to create
                       container images. Example -t 1.0.1 or --tag=0.1.0

```  
<br/>

**Check System Dependencies**
```
[cloudstate]# cloudstate --check
✔ Dependency Docker     OK!
✔ Dependency Kubectl    OK!
✔ Dependency Minikube   OK!
✔ Dependency Dotnet     OK!
✔ Dependency Go         OK!
✔ Dependency Java       OK!
✔ Dependency Mvn        OK!
✔ Dependency Npm        OK!
✔ Dependency Python     OK!
✔ Dependency Cargo      OK!
✖ Dependency Sbt not found in system path. If you use Scala please proceed to install it.
Ok(())

[cloudstate]#  
```
<br/>

**Initialize CloudState Operator:**  
```
[cloudstate]# cloudstate init  
😉 Creating CloudState namespace...
😻 Success on create CloudState namespace
🚀 Initializing CloudState operator...
namespace/cloudstate created
customresourcedefinition.apiextensions.k8s.io/statefulstores.cloudstate.io unchanged
customresourcedefinition.apiextensions.k8s.io/statefulservices.cloudstate.io unchanged
serviceaccount/cloudstate-operator created
configmap/cloudstate-operator-config created
role.rbac.authorization.k8s.io/cloudstate-operator-config-reader created
rolebinding.rbac.authorization.k8s.io/cloudstate-operator-config-reader-binding created
clusterrole.rbac.authorization.k8s.io/cloudstate-operator-role unchanged
clusterrolebinding.rbac.authorization.k8s.io/cloudstate-operator unchanged
🙌 Success on installing CloudState operator
Ok(())   
  
[cloudstate]#  
```

<br/>
 
**Create User Function Project from specific profile:**  
```  
[cloudstate]# cloudstate create \
  --name=shopping-cart \
  --profile=java \
  --registry=docker.io/sleipnir \
  --set-user=sleipnir \
  --set-pass=***** \
  --tag=1.0.1 \
  --set-editor=idea  

Creating user function project: "shopping-cart"  
Using profile: "java"  
Extracting profile template.... /root/.cloudstate/templates/java/java.tar.gz  
Downloading and install dependencies...  
[INFO] Scanning for projects...  
.....  
[INFO] ------------------------------------------------------------------------  
[INFO] BUILD SUCCESS  
[INFO] ------------------------------------------------------------------------  
[INFO] Total time:  3.489 s  
[INFO] Finished at: 2019-10-31T18:16:33-03:00  
[INFO] ------------------------------------------------------------------------  
Compiling project...  
Project successfully compiled  
Project created!  
Open editor!  
total 8  
drwxrwxr-x. 4 root root   30 Out 31 18:16 src  
-rw-rw-r--. 1 root root 3252 Out 31 18:16 pom.xml  
-rw-rw-r--. 1 root root  304 Out 31 18:16 deployment.yml  
drwxrwxr-x. 8 root root  168 Out 31 18:16 target  
Ok(())  
  
[cloudstate]#  
```

<br/>

**Build function:**   
  
```
[shopping-cart]# cloudstate build -p .  
Application { name: "shopping-cart", tag: "1.0.1", home_dir: "/root/.cloudstate", work_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart", user_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/.cloudstate", profile: "java", namespace: "cloudstate", repo: "docker.io/sleipnir/shopping-cart", repo_user: "sleipnir", repo_pass: "bsd*a211003", editor: "idea", data_store: "InMemory", port: 8088 }  
Building Project...  
[INFO] Scanning for projects...  
[WARNING] [WARNING] Some problems were encountered while building the effective model for com.example:shopping-cart:jar:1.0.1  
[WARNING] 'build.plugins.plugin.version' for org.apache.maven.plugins:maven-compiler-plugin is missing. @ line 37, column 21  
[WARNING] [WARNING] It is highly recommended to fix these problems because they threaten the stability of your build.  
[WARNING] [WARNING] For this reason, future Maven versions might no longer support building such malformed projects.  
[WARNING] [INFO] ------------------------------------------------------------------------  
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
[INFO] [INFO] ---------------------< com.example:shopping-cart >----------------------  
[INFO] Building shopping-cart 1.0.1  
[INFO] --------------------------------[ jar ]---------------------------------  
[INFO] [INFO] --- protobuf-maven-plugin:0.6.1:compile (default) @ shopping-cart ---  
[INFO] Compiling 2 proto file(s) to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/generated-sources/protobuf/java  
[WARNING] PROTOC: example/shoppingcart/shoppingcart.proto:7:1: warning: Import google/api/http.proto but not used.  
[libprotobuf WARNING ../../../../../src/google/protobuf/compiler/java/java_file.cc:231] example/shoppingcart/shoppingcart.proto: The file's outer class name, "Shoppingcart", matches the name of one of the types declared inside it when case is ignored. This can cause compilation issues on Windows / MacOS. Please either rename the type or use the java_outer_classname option to specify a different outer class name for the .proto file to be safe.  
  
[INFO] [INFO] --- maven-resources-plugin:2.6:resources (default-resources) @ shopping-cart ---  
[INFO] Using 'UTF-8' encoding to copy filtered resources.  
[INFO] Copying 2 resources  
[INFO] Copying 2 resources  
[INFO] [INFO] --- maven-compiler-plugin:3.1:compile (default-compile) @ shopping-cart ---  
[INFO] Changes detected - recompiling the module!  
[INFO] Compiling 4 source files to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/classes  
[INFO] [INFO] --- maven-resources-plugin:2.6:testResources (default-testResources) @ shopping-cart ---  
[INFO] Using 'UTF-8' encoding to copy filtered resources.  
[INFO] skip non existing resourceDirectory /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/src/test/resources  
[INFO] [INFO] --- maven-compiler-plugin:3.1:testCompile (default-testCompile) @ shopping-cart ---  
[INFO] Nothing to compile - all classes are up to date  
[INFO] [INFO] --- maven-surefire-plugin:2.12.4:test (default-test) @ shopping-cart ---  
[INFO] No tests to run.  
[INFO] [INFO] --- maven-jar-plugin:2.4:jar (default-jar) @ shopping-cart ---  
[INFO] Building jar: /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/shopping-cart-1.0.1.jar  
[INFO] [INFO] --- maven-install-plugin:2.4:install (default-install) @ shopping-cart ---  
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/shopping-cart-1.0.1.jar to /root/.m2/repository/com/example/shopping-cart/1.0.1/shopping-cart-1.0.1.jar  
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/pom.xml to /root/.m2/repository/com/example/shopping-cart/1.0.1/shopping-cart-1.0.1.pom  
[INFO] ------------------------------------------------------------------------  
[INFO] BUILD SUCCESS  
[INFO] ------------------------------------------------------------------------  
[INFO] Total time:  3.763 s  
[INFO] Finished at: 2019-10-31T18:49:45-03:00  
[INFO] ------------------------------------------------------------------------  
Compiling project...  
Project successfully compiled  
Ok(())  
  
[cloudstate]#  
```  

<br/>

**Or Build and Deploy function:**  
```  
[cloudstate]# cloudstate build \
  --path=shopping-cart \
  --tag=1.0.1 \
  --push && \
  cloudstate deploy --namespace=cloudstate  

Application { name: "shopping-cart", tag: "1.0.1", home_dir: "/root/.cloudstate", work_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart", user_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/.cloudstate", profile: "java", namespace: "cloudstate", repo: "docker.io/sleipnir/shopping-cart", repo_user: "sleipnir", repo_pass: "bsd*a211003", editor: "idea", data_store: "InMemory", port: 8088 }  
Building Project...  
[INFO] Scanning for projects...  
[WARNING] [WARNING] Some problems were encountered while building the effective model for com.example:shopping-cart:jar:1.0.1  
[WARNING] 'build.plugins.plugin.version' for org.apache.maven.plugins:maven-compiler-plugin is missing. @ line 37, column 21  
[WARNING] [WARNING] It is highly recommended to fix these problems because they threaten the stability of your build.  
[WARNING] [WARNING] For this reason, future Maven versions might no longer support building such malformed projects.  
[WARNING] [INFO] ------------------------------------------------------------------------  
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
[INFO] [INFO] ---------------------< com.example:shopping-cart >----------------------  
[INFO] Building shopping-cart 1.0.1  
[INFO] --------------------------------[ jar ]---------------------------------  
[INFO] [INFO] --- protobuf-maven-plugin:0.6.1:compile (default) @ shopping-cart ---  
[INFO] Compiling 2 proto file(s) to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/generated-sources/protobuf/java  
[WARNING] PROTOC: example/shoppingcart/shoppingcart.proto:7:1: warning: Import google/api/http.proto but not used.  
[libprotobuf WARNING ../../../../../src/google/protobuf/compiler/java/java_file.cc:231] example/shoppingcart/shoppingcart.proto: The file's outer class name, "Shoppingcart", matches the name of one of the types declared inside it when case is ignored. This can cause compilation issues on Windows / MacOS. Please either rename the type or use the java_outer_classname option to specify a different outer class name for the .proto file to be safe.  
  
[INFO] [INFO] --- maven-resources-plugin:2.6:resources (default-resources) @ shopping-cart ---  
[INFO] Using 'UTF-8' encoding to copy filtered resources.  
[INFO] Copying 2 resources  
[INFO] Copying 2 resources  
[INFO] [INFO] --- maven-compiler-plugin:3.1:compile (default-compile) @ shopping-cart ---  
[INFO] Changes detected - recompiling the module!  
[INFO] Compiling 4 source files to /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/classes  
[INFO] [INFO] --- maven-resources-plugin:2.6:testResources (default-testResources) @ shopping-cart ---  
[INFO] Using 'UTF-8' encoding to copy filtered resources.  
[INFO] skip non existing resourceDirectory /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/src/test/resources  
[INFO] [INFO] --- maven-compiler-plugin:3.1:testCompile (default-testCompile) @ shopping-cart ---  
[INFO] Nothing to compile - all classes are up to date  
[INFO] [INFO] --- maven-surefire-plugin:2.12.4:test (default-test) @ shopping-cart ---  
[INFO] No tests to run.  
[INFO] [INFO] --- maven-jar-plugin:2.4:jar (default-jar) @ shopping-cart ---  
[INFO] Building jar: /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/shopping-cart-1.0.1.jar  
[INFO] [INFO] --- maven-install-plugin:2.4:install (default-install) @ shopping-cart ---  
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/target/shopping-cart-1.0.1.jar to /root/.m2/repository/com/example/shopping-cart/1.0.1/shopping-cart-1.0.1.jar  
[INFO] Installing /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/pom.xml to /root/.m2/repository/com/example/shopping-cart/1.0.1/shopping-cart-1.0.1.pom  
[INFO] ------------------------------------------------------------------------  
[INFO] BUILD SUCCESS  
[INFO] ------------------------------------------------------------------------  
[INFO] Total time:  3.179 s  
[INFO] Finished at: 2019-10-31T18:51:33-03:00  
[INFO] ------------------------------------------------------------------------  
Compiling project...  
Project successfully compiled  
Ok("/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart")  
Path -> /home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/.cloudstate/user.json  
Application { name: "shopping-cart", tag: "1.0.1", home_dir: "/root/.cloudstate", work_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart", user_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/.cloudstate", profile: "java", namespace: "cloudstate", repo: "docker.io/sleipnir/shopping-cart", repo_user: "sleipnir", repo_pass: "bsd*a211003", editor: "idea", data_store: "InMemory", port: 8088 }  
[INFO] Scanning for projects...  
[WARNING] [WARNING] Some problems were encountered while building the effective model for com.example:shopping-cart:jar:1.0.1  
[WARNING] 'build.plugins.plugin.version' for org.apache.maven.plugins:maven-compiler-plugin is missing. @ line 37, column 21  
[WARNING] [WARNING] It is highly recommended to fix these problems because they threaten the stability of your build.  
[WARNING] [WARNING] For this reason, future Maven versions might no longer support building such malformed projects.  
[WARNING] [INFO] ------------------------------------------------------------------------  
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
[INFO] [INFO] ---------------------< com.example:shopping-cart >----------------------  
[INFO] Building shopping-cart 1.0.1  
[INFO] --------------------------------[ jar ]---------------------------------  
[INFO] [INFO] --- jib-maven-plugin:1.7.0:build (default-cli) @ shopping-cart ---  
[INFO] [INFO] Containerizing application to sleipnir/shopping-cart, sleipnir/shopping-cart:1.0.1...  
[WARNING] Base image 'gcr.io/distroless/java:8' does not use a specific image digest - build may not be reproducible  
[INFO] Using base image with digest: sha256:a13ac1ce516ec5e49ae9dfd3b8183e9e8328180a65757d454e594a9ce6d1e35d  
[INFO] [INFO] Container entrypoint set to [java, -XX:+UseG1GC, -XX:+UseStringDeduplication, -cp, /app/resources:/app/classes:/app/libs/*, com.example.Main]  
[INFO] [INFO] Built and pushed image as sleipnir/shopping-cart, sleipnir/shopping-cart:1.0.1  
[INFO] Executing tasks:  
[INFO] [===========================   ] 90,0% complete  
[INFO] > launching layer pushers  
[INFO] [INFO] ------------------------------------------------------------------------  
[INFO] BUILD SUCCESS  
[INFO] ------------------------------------------------------------------------  
[INFO] Total time:  9.165 s  
[INFO] Finished at: 2019-10-31T18:51:44-03:00  
[INFO] ------------------------------------------------------------------------  
Push container image...  
Pushed!  
Ok("/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart")  
Application { name: "shopping-cart", tag: "1.0.1", home_dir: "/root/.cloudstate", work_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart", user_dir: "/home/sleipnir/development/workspace/pessoal/cloudstate-cli/cloudstate/shopping-cart/.cloudstate", profile: "java", namespace: "cloudstate", repo: "docker.io/sleipnir/shopping-cart", repo_user: "sleipnir", repo_pass: "bsd*a211003", editor: "idea", data_store: "InMemory", port: 8088 }  
Success on installing 'User Function' shopping-cart in namespace: cloudstate  
  
```  
<br/>

**Checking deploy:**  
```  
[cloudstate]# kubectl get po -n cloudstate  
NAME                                        READY   STATUS    RESTARTS   AGE  
cloudstate-operator-84b8848647-ds2h8        1/1     Running   0          26m  
shopping-cart-deployment-7657c848fc-tpngd   1/2     Running   0          16m  
  
[cloudstate]# kubectl logs -n cloudstate shopping-cart-deployment-7657c848fc-tpngd -c user-container  
2019-10-31 21:38:54.067 DEBUG io.cloudstate.javasupport.impl.AnySupport - Attempting to load com.google.protobuf.Message class com.example.shoppingcart.Shoppingcart$AddLineItem  
2019-10-31 21:38:54.072 DEBUG io.cloudstate.javasupport.impl.AnySupport - Attempting to load com.google.protobuf.Message class com.google.protobuf.Empty  
2019-10-31 21:38:54.074 DEBUG io.cloudstate.javasupport.impl.AnySupport - Attempting to load com.google.protobuf.Message class com.example.shoppingcart.Shoppingcart$RemoveLineItem  
2019-10-31 21:38:54.077 DEBUG io.cloudstate.javasupport.impl.AnySupport - Attempting to load com.google.protobuf.Message class com.example.shoppingcart.Shoppingcart$GetShoppingCart  
2019-10-31 21:38:54.080 DEBUG io.cloudstate.javasupport.impl.AnySupport - Attempting to load com.google.protobuf.Message class com.example.shoppingcart.Shoppingcart$Cart  
[INFO] [10/31/2019 21:38:56.977] [StatefulService-akka.actor.default-dispatcher-7] [akka.actor.ActorSystemImpl(StatefulService)] Received discovery call from sidecar [cloudstate-proxy-core 0.4.3] supporting CloudState 0.1  
..........  
  
```  

<br/>

**Or use CLI logs command**
```
[sleipnir@sleipnir shopping-cart]# cloudstate logs --name=shopping-cart 
Get logs for shopping-cart
Ok(())
2019/11/13 19:51:02.928093 Received discovery call from sidecar [cloudstate-proxy-core we2154d9a] supporting Cloudstate 0.1
2019/11/13 19:51:02.928179 Responding with: service_name:"shopping-cart" service_version:"0.1.0" service_runtime:"go1.13 linux/amd64" support_library_name:"cloudstate-go-support" support_library_version:"0.1.0" 
2019-11-13 19:51:09.389 INFO akka.management.cluster.bootstrap.internal.BootstrapCoordinator - Initiating new cluster, self-joining [akka://cloudstate-proxy@172.17.0.3:2552]. Other nodes are expected to locate this cluster via continued contact-point probing.
2019-11-13 19:51:09.390 INFO akka.remote.RemoteActorRefProvider$RemoteDeadLetterActorRef - Message [akka.management.cluster.bootstrap.internal.BootstrapCoordinator$Protocol$BootstrappingCompleted$] from Actor[akka://cloudstate-proxy/system/bootstrapCoordinator#1451520250] to Actor[akka://cloudstate-proxy/deadLetters] was not delivered. [1] dead letters encountered. If this is not an expected behavior, then [Actor[akka://cloudstate-proxy/deadLetters]] may have terminated unexpectedly, This logging can be turned off or adjusted with configuration settings 'akka.log-dead-letters' and 'akka.log-dead-letters-during-shutdown'.
2019-11-13 19:51:09.394 INFO akka.cluster.Cluster(akka://cloudstate-proxy) - Cluster Node [akka://cloudstate-proxy@172.17.0.3:2552] - Node [akka://cloudstate-proxy@172.17.0.3:2552] is JOINING itself (with roles [dc-default]) and forming new cluster
2019-11-13 19:51:09.394 INFO akka.cluster.Cluster(akka://cloudstate-proxy) - Cluster Node [akka://cloudstate-proxy@172.17.0.3:2552] - is the new leader among reachable nodes (more leaders may exist)
2019-11-13 19:51:09.398 INFO akka.cluster.Cluster(akka://cloudstate-proxy) - Cluster Node [akka://cloudstate-proxy@172.17.0.3:2552] - Leader is moving node [akka://cloudstate-proxy@172.17.0.3:2552] to [Up]
2019-11-13 19:51:09.404 INFO akka.actor.EmptyLocalActorRef - Message [akka.cluster.sharding.ShardCoordinator$Internal$Register] from Actor[akka://cloudstate-proxy/system/sharding/ShoppingCart#1132162436] to Actor[akka://cloudstate-proxy/system/sharding/ShoppingCartCoordinator/singleton/coordinator] was not delivered. [2] dead letters encountered. If this is not an expected behavior, then [Actor[akka://cloudstate-proxy/system/sharding/ShoppingCartCoordinator/singleton/coordinator]] may have terminated unexpectedly, This logging can be turned off or adjusted with configuration settings 'akka.log-dead-letters' and 'akka.log-dead-letters-during-shutdown'.
2019-11-13 19:51:09.411 INFO akka.cluster.singleton.ClusterSingletonManager - Singleton manager starting singleton actor [akka://cloudstate-proxy/system/sharding/ShoppingCartCoordinator/singleton]
2019-11-13 19:51:09.412 INFO akka.cluster.singleton.ClusterSingletonManager - ClusterSingletonManager state change [Start -> Oldest]
2019-11-13 19:51:09.418 INFO akka.cluster.sharding.DDataShardCoordinator - ShardCoordinator was moved to the active state State(Map())
2019-11-13 19:51:09.434 INFO io.cloudstate.proxy.EntityDiscoveryManager - CloudState proxy online at /0.0.0.0:8013
```

<br/>

**Listing supported profiles:**  
```  
[cloudstate]# cloudstate --list-profiles
Profile    | Dependencies         | Resolved   | Maturity Level |
csharp     | dotnet               | true       | ⌛             |
go         | go                   | true       | 👍             |
java       | java, [maven | sbt]  | true       | 👌             |
kotlin     | java, [maven | sbt]  | true       | 👌             |
node       | node                 | true       | 👌             |
python     | python, virtualenv   | true       | ⌛             |
rust       | rust, cargo          | true       | ⌛             |
scala      | java, scala, sbt     | true       | ⌛             |

Subtitle:
👌 Stable for production usage
👍 Unstable but usable
⌛ Work in progress
👎 Unknown
Ok(())

[cloudstate]#  
```

**Destroying CloudState Resources**
```
[cloudstate]# cloudstate destroy
🔥 Destroying CloudState resources
pod "cloudstate-operator-6579fb749c-d4fhs" deleted
pod "shopping-cart-deployment-668686cbd7-8kzqj" deleted
service "shopping-cart" deleted
deployment.apps "cloudstate-operator" deleted
deployment.apps "shopping-cart-deployment" deleted
replicaset.apps "cloudstate-operator-6579fb749c" deleted
replicaset.apps "shopping-cart-deployment-668686cbd7" deleted
😿 Deleted all resources
namespace "cloudstate" deleted
💔 CloudState dead
Ok(())

```

## Languages
