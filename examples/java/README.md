# Create Cloudstate Java User Function

### Install CloudState CLI:

```shell script
[sleipnir@sleipnir cloudstate-cli]# curl https://raw.githubusercontent.com/sleipnir/cloudstate-cli/master/bin/cloudstate-installer.sh | sh -
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  2369  100  2369    0     0  13077      0 --:--:-- --:--:-- --:--:-- 13088
linux-gnu
Update docker
Update kubectl
Update minikube
Update curl
openjdk version "11.0.4" 2019-07-16 LTS
OpenJDK Runtime Environment Zulu11.33+15-CA (build 11.0.4+11-LTS)
OpenJDK 64-Bit Server VM Zulu11.33+15-CA (build 11.0.4+11-LTS, mixed mode)
Install java
Update mvn
Update rustc
Update cargo
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0* About to connect() to raw.githubusercontent.com port 443 (#0)
*   Trying 151.101.0.133...
* Connected to raw.githubusercontent.com (151.101.0.133) port 443 (#0)
* Initializing NSS with certpath: sql:/etc/pki/nssdb
*   CAfile: /etc/pki/tls/certs/ca-bundle.crt
  CApath: none
* SSL connection using TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
* Server certificate:
* 	subject: CN=www.github.com,O="GitHub, Inc.",L=San Francisco,ST=California,C=US
* 	start date: Mar 23 00:00:00 2017 GMT
* 	expire date: Mai 13 12:00:00 2020 GMT
* 	common name: www.github.com
* 	issuer: CN=DigiCert SHA2 High Assurance Server CA,OU=www.digicert.com,O=DigiCert Inc,C=US
> GET /sleipnir/cloudstate-cli/master/bin/linux-x86/cloudstate HTTP/1.1
> User-Agent: curl/7.29.0
> Host: raw.githubusercontent.com
> Accept: */*
> Cache-Control: no-cache
> 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0< HTTP/1.1 200 OK
< Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; sandbox
< Strict-Transport-Security: max-age=31536000
< X-Content-Type-Options: nosniff
< X-Frame-Options: deny
< X-XSS-Protection: 1; mode=block
< ETag: "9efe413f68e66b3cc59d677987fafff2440eb596c82f2e289c969161462b46b0"
< Content-Type: application/octet-stream
< Cache-Control: max-age=300
< X-Geo-Block-List:
< X-GitHub-Request-Id: 22AC:32B8:2F384:3BEDB:5DB210EA
< Content-Length: 926376
< Accept-Ranges: bytes
< Date: Thu, 24 Oct 2019 21:02:17 GMT
< Via: 1.1 varnish
< Connection: keep-alive
< X-Served-By: cache-gru17143-GRU
< X-Cache: HIT
< X-Cache-Hits: 1
< X-Timer: S1571950937.063955,VS0,VE2
< Vary: Authorization,Accept-Encoding
< Access-Control-Allow-Origin: *
< X-Fastly-Request-ID: 6d949d351b9a160c6134152db08eb4a93aae7130
< Expires: Thu, 24 Oct 2019 21:07:17 GMT
< Source-Age: 110
< 
{ [data not shown]
100  904k  100  904k    0     0  2231k      0 --:--:-- --:--:-- --:--:-- 2228k
* Connection #0 to host raw.githubusercontent.com left intact

[sleipnir@sleipnir cloudstate-cli]#
```

### Validating: 
```shell script
[sleipnir@sleipnir cloudstate-cli]# cloudstate --version
cloudstate 0.0.1

[sleipnir@sleipnir cloudstate-cli]# cloudstate --help
cloudstate 0.0.1
Adriano Santos <sleipnir@bsd.com.br>
CloudState CLI

USAGE:
    cloudstate [FLAGS] [OPTIONS]

FLAGS:
        --build          Build project with profile specified
        --deploy         Deploy user function with CloudState sidecar in K8s environment
    -h, --help           Prints help information
    -i, --init           Initialize a CloudState k8s namespace/operator
    -l, --list-profiles    List all profiles supported
        --namespace      Set k8s namespace for user functioncargo
        --push        Publish container image in repository
    -V, --version        Prints version information

OPTIONS:
    -c, --create <create>          Create a new user function project from profile. Example --create=shopping-cart
    -d, --datastore <datastore>    Used in conjunction with 'create'. Enable CloudState Stateful stores. Example
                                   --datastore=Cassandra. Valid values [Cassandra, Postgres or InMemory]
        --group-id <group-id>      Used in conjunction with 'create'. Only for java/dotnet profiles. Set the name of
                                   package or namespace
    -I, --profile <profile>            Used in conjunction with 'create'. Set language profile for this project. Possible
                                   values is [java, node, go, dotnet, rust, python, scala]
    -r, --repo <repo>              Used in conjunction with 'create'. Set the docker repository. Used to create
                                   container images. Example -r quay.io/myuser or --repo=sleipnir/test
    -t, --tag <tag>                Used in conjunction with 'create' and/or 'build'. Set version of user function. Used
                                   to create container images. Example -t 1.0.1 or --tag=0.1.0
[sleipnir@sleipnir cloudstate-cli]# 

```

### Make sure your kubernetes or minikube environment is working:
```shell script
[sleipnir@sleipnir cloudstate-cli]# minikube status
host: Running
kubelet: Running
apiserver: Running
kubectl: Correctly Configured: pointing to minikube-vm at 192.168.39.222

[sleipnir@sleipnir cloudstate-cli]# kubectl cluster-info
Kubernetes master is running at https://192.168.39.222:8443
KubeDNS is running at https://192.168.39.222:8443/api/v1/namespaces/kube-system/services/kube-dns:dns/proxy

To further debug and diagnose cluster problems, use 'kubectl cluster-info dump'.
[sleipnir@sleipnir cloudstate-cli]#
```

### Now let's initialize CloudState operator:
```shell script
[sleipnir@sleipnir cloudstate-cli]# kubectl get namespaces
NAME              STATUS   AGE
default           Active   84d
kube-node-lease   Active   84d
kube-public       Active   84d
kube-system       Active   84d

[sleipnir@sleipnir cloudstate-cli]# cloudstate --init
Creating CloudState namespace...
Initializing CloudState operator...
namespace/cloudstate created
customresourcedefinition.apiextensions.k8s.io/statefulservices.cloudstate.io unchanged
customresourcedefinition.apiextensions.k8s.io/journals.cloudstate.io unchanged
serviceaccount/cloudstate-operator created
configmap/cloudstate-operator-config created
clusterrole.rbac.authorization.k8s.io/cloudstate-operator-role unchanged
role.rbac.authorization.k8s.io/cloudstate-operator-role created
clusterrolebinding.rbac.authorization.k8s.io/cloudstate-operator unchanged
rolebinding.rbac.authorization.k8s.io/cloudstate-operator created
deployment.apps/cloudstate-operator created
No verbose info
Ok(())

[sleipnir@sleipnir cloudstate-cli]# kubectl get namespaces
NAME              STATUS   AGE
cloudstate        Active   73s
default           Active   84d
kube-node-lease   Active   84d
kube-public       Active   84d
kube-system       Active   84d

[sleipnir@sleipnir cloudstate-cli]# 

```

### Create Project from Java profile:
```shell script
[sleipnir@sleipnir cloudstate-cli]# cloudstate --create=shopping-cart --profile=java --group-id=com.example --repo=sleipnir --tag=1.0-SNAPSHOT -d InMemory
Creating user function project: "shopping-cart"
Using profile: "java"
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

*** At the end of this process your default editor will open with the sample code. Now go hacking!

### Build, Publish and Deploy your own User function in CloudState:
TODO: Commands not implemented yet
```shell script
[sleipnir@sleipnir shopping-cart]# cloudstate --build --push
No verbose info
Ok(())
[sleipnir@sleipnir shopping-cart]# cloudstate --deploy
No verbose info
Ok(())

```
