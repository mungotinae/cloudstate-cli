# CloudState CLI

## Client Line Interface for management CloudState artifacts

### Install:
```
[cloudstate]#  curl https://raw.githubusercontent.com/sleipnir/cloudstate-cli/master/bin/cloudstate-installer.sh | sh -
```

### Usage:
```
[cloudstate]# cloudstate --help
cloudstate 0.0.1
Adriano Santos <sleipnir@bsd.com.br>
CloudState CLI

USAGE:
    cloudstate [FLAGS] [OPTIONS]

FLAGS:
        --build          Build project with specified idiom
        --deploy         Deploy user function with CloudState sidecar in K8s environment
    -h, --help           Prints help information
    -i, --init           Initialize a CloudState k8s namespace/operator
    -l, --list-idioms    List all idioms supported
        --namespace      Set k8s namespace for user function
        --publish        Publish container image in repository
    -V, --version        Prints version information

OPTIONS:
    -c, --create <create>          Create a new user function project from template. Example --create=shopping-cart
    -d, --datastore <datastore>    Used in conjunction with 'create'. Enable CloudState Stateful stores. Example
                                   --datastore=Cassandra. Valid values [Cassandra, Postgres or InMemory]
    -I, --idiom <idiom>            Used in conjunction with 'create'. Set language template for this project. Possible
                                   values is [java, node, go, dotnet, rust, python, scala]
    -r, --repo <repo>              Used in conjunction with 'create'. Set the docker repository. Used to create
                                   container images. Example -r quay.io/myuser or --repo=sleipnir/test
    -t, --tag <tag>                Used in conjunction with 'create' and/or 'build'. Set version of user function. Used
                                   to create container images. Example -t 1.0.1 or --tag=0.1.0
```

### Initialize CloudState Operator:
```
[cloudstate]# cloudstate --init
Creating CloudState namespace...
namespace/cloudstate created
Initializing CloudState operator...
customresourcedefinition.apiextensions.k8s.io/statefulservices.cloudstate.io unchanged
customresourcedefinition.apiextensions.k8s.io/journals.cloudstate.io unchanged
serviceaccount/cloudstate-operator created
configmap/cloudstate-operator-config created
clusterrole.rbac.authorization.k8s.io/cloudstate-operator-role unchanged
role.rbac.authorization.k8s.io/cloudstate-operator-role created
clusterrolebinding.rbac.authorization.k8s.io/cloudstate-operator unchanged
rolebinding.rbac.authorization.k8s.io/cloudstate-operator created
deployment.apps/cloudstate-operator created

[cloudstate]#
```

### Create User Function Project from specific template:
```
[cloudstate]# cloudstate --create=shopping-cart --template=rust
Creating user function project: "shopping-cart"
Using template: "rust"
     Created binary (application) `shopping-cart` package
Creating Dockerfile
Creating deployment.yml
Project created!
No verbose info
Ok(())
total 4
drwxrwxr-x. 2 root root  21 Out 23 17:07 src
-rw-rw-r--. 1 root root 233 Out 23 17:07 Cargo.toml
-rw-rw-r--. 1 root root   0 Out 23 17:07 Dockerfile
-rw-rw-r--. 1 root root   0 Out 23 17:07 deployment.yml
[cloudstate]#
```

### Build function: 
```
[shopping-cart]# cloudstate --build=com.acme/shopping-cart
Sending build context to Docker daemon  6.144kB
Step 1/10 : ARG BASE_IMAGE=ekidd/rust-musl-builder:latest
Step 2/10 : FROM ${BASE_IMAGE} AS cargo-build
 ---> 04b7a7c5fd0a
Step 3/10 : RUN sudo apt-get update && sudo apt-get install -y upx-ucl
 ---> Running in 720ef81f00a1
Get:1 http://security.ubuntu.com/ubuntu bionic-security InRelease [88.7 kB]
Get:2 http://archive.ubuntu.com/ubuntu bionic InRelease [242 kB]
Get:3 http://security.ubuntu.com/ubuntu bionic-security/multiverse amd64 Packages [5945 B]
Get:4 http://security.ubuntu.com/ubuntu bionic-security/restricted amd64 Packages [12.6 kB]
Get:5 http://archive.ubuntu.com/ubuntu bionic-updates InRelease [88.7 kB]
Get:6 http://security.ubuntu.com/ubuntu bionic-security/main amd64 Packages [695 kB]
Get:7 http://archive.ubuntu.com/ubuntu bionic-backports InRelease [74.6 kB]
Get:8 http://archive.ubuntu.com/ubuntu bionic/restricted amd64 Packages [13.5 kB]
Get:9 http://security.ubuntu.com/ubuntu bionic-security/universe amd64 Packages [782 kB]
Get:10 http://archive.ubuntu.com/ubuntu bionic/universe amd64 Packages [11.3 MB]
Get:11 http://archive.ubuntu.com/ubuntu bionic/main amd64 Packages [1344 kB]
Get:12 http://archive.ubuntu.com/ubuntu bionic/multiverse amd64 Packages [186 kB]
Get:13 http://archive.ubuntu.com/ubuntu bionic-updates/multiverse amd64 Packages [9023 B]
Get:14 http://archive.ubuntu.com/ubuntu bionic-updates/restricted amd64 Packages [23.2 kB]
Get:15 http://archive.ubuntu.com/ubuntu bionic-updates/universe amd64 Packages [1299 kB]
Get:16 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 Packages [990 kB]
Get:17 http://archive.ubuntu.com/ubuntu bionic-backports/universe amd64 Packages [4227 B]
Get:18 http://archive.ubuntu.com/ubuntu bionic-backports/main amd64 Packages [2496 B]
Fetched 17.2 MB in 20s (862 kB/s)
Reading package lists...
Reading package lists...
Building dependency tree...
Reading state information...
The following additional packages will be installed:
  libucl1
The following NEW packages will be installed:
  libucl1 upx-ucl
0 upgraded, 2 newly installed, 0 to remove and 34 not upgraded.
Need to get 401 kB of archives.
After this operation, 2083 kB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu bionic/universe amd64 libucl1 amd64 1.03+repack-4 [23.9 kB]
Get:2 http://archive.ubuntu.com/ubuntu bionic/universe amd64 upx-ucl amd64 3.94-4 [377 kB]
debconf: delaying package configuration, since apt-utils is not installed
Fetched 401 kB in 2s (180 kB/s)
Selecting previously unselected package libucl1:amd64.
(Reading database ... 20378 files and directories currently installed.)
Preparing to unpack .../libucl1_1.03+repack-4_amd64.deb ...
Unpacking libucl1:amd64 (1.03+repack-4) ...
Selecting previously unselected package upx-ucl.
Preparing to unpack .../upx-ucl_3.94-4_amd64.deb ...
Unpacking upx-ucl (3.94-4) ...
Setting up libucl1:amd64 (1.03+repack-4) ...
Processing triggers for libc-bin (2.27-3ubuntu1) ...
Setting up upx-ucl (3.94-4) ...
update-alternatives: error: no alternatives for upx
update-alternatives: using /usr/bin/upx-ucl to provide /usr/bin/upx (upx) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/upx.1.gz because associated file /usr/share/man/man1/upx-ucl.1.gz (of link group upx) doesn't exist
Removing intermediate container 720ef81f00a1
 ---> 6a221b663780
Step 4/10 : ADD . ./
 ---> 40aaf88290da
Step 5/10 : RUN sudo chown -R rust:rust /home/rust
 ---> Running in b7cfbdf218a6
Removing intermediate container b7cfbdf218a6
 ---> 6bc0ff9a3982
Step 6/10 : RUN cargo build --release
 ---> Running in 0f0a76776562
   Compiling shopping-cart v0.1.0 (/home/rust/src)
    Finished release [optimized] target(s) in 0.78s
Removing intermediate container 0f0a76776562
 ---> 402eb736d0ea
Step 7/10 : RUN /usr/bin/upx --brute /home/rust/src/target/x86_64-unknown-linux-musl/release/shopping-cart
 ---> Running in 115e1425d413
                       Ultimate Packer for eXecutables
                          Copyright (C) 1996 - 2017
UPX 3.94        Markus Oberhumer, Laszlo Molnar & John Reiser   May 12th 2017

        File size         Ratio      Format      Name
   --------------------   ------   -----------   -----------
   2515048 ->    512832   20.39%   linux/amd64   shopping-cart

Packed 1 file.
Removing intermediate container 115e1425d413
 ---> 70852fddd98b
Step 8/10 : FROM scratch
 ---> 
Step 9/10 : COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/shopping-cart     /usr/local/bin/
 ---> f7aea6e9f8ac
Step 10/10 : CMD ["/usr/local/bin/shopping-cart"]
 ---> Running in 039b6e1759d9
Removing intermediate container 039b6e1759d9
 ---> 26e0ce1ae31d
Successfully built 26e0ce1ae31d
Successfully tagged shopping-cart:latest
[shopping-cart]# docker images | grep shopping-cart
shopping-cart                               latest               26e0ce1ae31d        10 seconds ago      513kB
No verbose info
Ok(())
[root@sleipnir shopping-cart]#
```

### Deploy function:
```
```

### Listing supported templates:
```
[cloudstate]# cloudstate --list-templates
[Template Name]:[Dependencies]:[Resolved]
[go]:[go]:[true]
[java]:[java, [maven | sbt]]:[true]
[dotnet]:[dotnet]:[true]
[rust]:[rust, cargo]:[true]
[python]:[python, virtualenv]:[true]
[node]:[node]:[true]
[scala]:[java, scala, sbt]:[true]
No verbose info
Ok(())
[cloudstate]#
```