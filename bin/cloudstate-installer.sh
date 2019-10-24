#!/bin/bash
HOME_DIR=$( getent passwd "$USER" | cut -d: -f6 )

#TODO: Extract function
echo $OSTYPE
#if [[ "$OSTYPE" == "linux-gnu" ]]; then
#        # ...
#elif [[ "$OSTYPE" == "darwin"* ]]; then
#        # Mac OSX
#elif [[ "$OSTYPE" == "cygwin" ]]; then
#        # POSIX compatibility layer and Linux environment emulation for Windows
#elif [[ "$OSTYPE" == "msys" ]]; then
#        # Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
#elif [[ "$OSTYPE" == "win32" ]]; then
#        # I'm not sure this can happen.
#elif [[ "$OSTYPE" == "freebsd"* ]]; then
#        # ...
#else
#        # Unknown.
#fi

mkdir -p $HOME_DIR/.cloudstate

#TODO: Extract function
# Verify Docker
if [[ $(which docker) && $(docker --version) ]]; then
    echo "Update docker"
    # command
  else
    echo "Install docker"
    # command
fi

#TODO: Extract function
# Verify Kubectl
if [[ $(which kubectl) && $(kubectl version) ]]; then
    echo "Update kubectl"
    # command
  else
    echo "Install kubectl"
    # command
fi

#TODO: Extract function
# Verify Minikube
if [[ $(which minikube) && $(minikube status) ]]; then
    echo "Update minikube"
    # command
  else
    echo "Install minikube"
    # command
fi

#TODO: Extract function
# Verify Curl
if [[ $(which curl) && $(curl --version) ]]; then
    echo "Update curl"
    # command
  else
    echo "Install curl"
    # command
fi

#TODO: Extract function
# Verify languages dependencies

# Verify Java
if [[ $(which java) && $(java -version) ]]; then
    echo "Update java"
    # command
  else
    echo "Install java"
    # command
fi

# Verify Maven
if [[ $(which mvn) && $(mvn --version) ]]; then
    echo "Update mvn"
    # command
  else
    echo "Install mvn"
    # command
fi

# Verify Rust
if [[ $(which rustc) && $(rustc --version) ]]; then
    echo "Update rustc"
    # command
  else
    echo "Install rustc"
    # command
fi

if [[ $(which cargo) && $(cargo --version) ]]; then
    echo "Update cargo"
    # command
  else
    echo "Install cargo"
    # command
fi

# And many others dependencies validations here:
#......

# Download binary
curl --url https://raw.githubusercontent.com/sleipnir/cloudstate-cli/master/bin/linux-x86/cloudstate --output /tmp/cloudstate
mv /tmp/cloudstate /usr/local/bin
chmod +x /usr/local/bin/cloudstate