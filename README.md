# CloudState CLI

Client Line Interface for management CloudState artifacts

Install:
```
[cloudstate]#  curl ...... | sh -
```

Usage:
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

Initialize CloudState Operator:
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

Create User Function Project from specific template:
```
[cloudstate]# cloudstate --create=shopping-cart --template=rust
No verbose info
```

Build function:
```
```

Deploy function:
```
```