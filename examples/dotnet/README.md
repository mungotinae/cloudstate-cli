# Create Cloudstate .Net Core User Function

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
        --build          Build project with template specified
        --deploy         Deploy user function with CloudState sidecar in K8s environment
    -h, --help           Prints help information
    -i, --init           Initialize a CloudState k8s namespace/operator
    -l, --list-idioms    List all idioms supported
        --namespace      Set k8s namespace for user functioncargo
        --publish        Publish container image in repository
    -V, --version        Prints version information

OPTIONS:
    -c, --create <create>          Create a new user function project from template. Example --create=shopping-cart
    -d, --datastore <datastore>    Used in conjunction with 'create'. Enable CloudState Stateful stores. Example
                                   --datastore=Cassandra. Valid values [Cassandra, Postgres or InMemory]
        --group-id <group-id>      Used in conjunction with 'create'. Only for java/dotnet idioms. Set the name of
                                   package or namespace
    -I, --idiom <idiom>            Used in conjunction with 'create'. Set language template for this project. Possible
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

### Create Project from .Net Core template:
```shell script
[sleipnir@sleipnir cloudstate-cli]# cloudstate --create=shopping-cart --idiom=dotnet --group-id=com.example --repo=sleipnir --tag=1.0-SNAPSHOT -d InMemory

No verbose info
Ok(())

```

*** At the end of this process your default editor will open with the sample code. Now go hacking!

### Build, Publish and Deploy your own User function in CloudState:
TODO: Commands not implemented yet
```shell script
[sleipnir@sleipnir shopping-cart]# cloudstate --build --publish
No verbose info
Ok(())
[sleipnir@sleipnir shopping-cart]# cloudstate --deploy
No verbose info
Ok(())

```
