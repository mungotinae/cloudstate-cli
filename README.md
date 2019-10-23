# CloudState CLI

## Client Line Interface for management CloudState artifacts

### Install:
```
[cloudstate]#  curl ...... | sh -
```

### Usage:
```
[cloudstate]# cloudstate --help
cloudstate 0.0.1
Adriano Santos <sleipnir@bsd.com.br>
CloudState CLI

USAGE:
    cloudstate [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
        --build             Build project with template specified
        --deploy            Deploy user function with CloudState sidecar in K8s environment
    -h, --help              Prints help information
    -i, --init              Init a CloudState k8s namespace/operator
    -l, --list-templates    List all templates supported
        --namespace         Set k8s namespace for user functioncargo
    -t, --template          Set language template for this project. Possible values is [java, node, go, csharp, rust,
                            python]
    -V, --version           Prints version information

OPTIONS:
    -c, --create <create>    Create a new user function project from template
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

### Listing supported templates:
```
[cloudstate]# cloudstate --list-templates
Template Name, Dependencies
go:[go]
node:[node]
python:[python, virtualenv]
rust:[rust, cargo]
dotnet:[dotnet]
scala:[java, scala, sbt]
java:[java, [maven | sbt]]
No verbose info
```

### Build function:
```
```

### Deploy function:
```
```