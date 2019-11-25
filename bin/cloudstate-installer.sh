#!/bin/bash
VERSION=0.3.26
DARWIN_RELEASE_URL=https://github.com/sleipnir/cloudstate-cli/releases/download/$VERSION/cloudstate-cli-$VERSION-x86_64-apple-darwin.tar.gz
LINUX_RELEASE_URL=https://github.com/sleipnir/cloudstate-cli/releases/download/$VERSION/cloudstate-cli-$VERSION-x86_64-unknown-linux-gnu.tar.gz

RELEASE_URL=$LINUX_RELEASE_URL

if [ $HOME ]; then
  HOME_DIR=$HOME
elif [ $( command -v getent ) ]; then
  HOME_DIR=$( getent passwd "$USER" | cut -d: -f6 )
else
  echo "Could not determine path to user's home directory"
  exit 1
fi

# Bash completion
INSTALL_COMPLETIONS=1
BASH_COMPLETION_DIR=/etc/bash_completion.d
skip_bash_completion() {
  INSTALL_COMPLETIONS=0
  if [ ! -z $1 ]; then
    echo $1
  fi
}

#TODO: Extract function
echo $OSTYPE
if [[ "$OSTYPE" == "linux-gnu" ]]; then
: #        # ...
elif [[ "$OSTYPE" == "darwin"* ]]; then
  # Mac OSX
  if [ $( command -v brew ) ]; then
    RELEASE_URL=$DARWIN_RELEASE_URL

    BREW_PREFIX=$( brew --prefix )
    BASH_COMPLETION_DIR=$BREW_PREFIX/etc/bash_completion.d
    if [[ ! -d "$BASH_COMPLETION_DIR" ]]; then
      skip_bash_completion "Could not determine the path to bash_completion.d.  Skipping install of completion tools"
    fi
  fi

#elif [[ "$OSTYPE" == "cygwin" ]]; then
#        # POSIX compatibility layer and Linux environment emulation for Windows
#elif [[ "$OSTYPE" == "msys" ]]; then
#        # Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
#elif [[ "$OSTYPE" == "win32" ]]; then
#        # I'm not sure this can happen.
#elif [[ "$OSTYPE" == "freebsd"* ]]; then
#        # ...
else
: #       # Unknown.
fi

CLOUDSTATE_CONF=$HOME_DIR/.cloudstate
if [ ! -d "$CLOUDSTATE_CONF" ]; then
    mkdir -p "$CLOUDSTATE_CONF"
fi

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

# Download binary
touch /tmp/cloudstate.tar.gz
#curl -vvv -H 'Cache-Control: no-cache' --url https://raw.githubusercontent.com/sleipnir/cloudstate-cli/master/bin/linux-x86/cloudstate --output /tmp/cloudstate
curl -vvv -H 'Cache-Control: no-cache' --url $RELEASE_URL --output /tmp/cloudstate.tar.gz

tar -zxvf /tmp/cloudstate.tar.gz --directory /usr/local/bin
#mv /tmp/cloudstate /usr/local/bin
chmod +x /usr/local/bin/cloudstate
rm -rf /tmp/cloudstate.tar.gz

# Install completions
cloudstate completions bash >> $BASH_COMPLETION_DIR/cloudstate.bash-completion
sed -i '$ d' $BASH_COMPLETION_DIR/cloudstate.bash-completion
source $BASH_COMPLETION_DIR/cloudstate.bash-completion

echo "Install Finish $(cloudstate --version) "
exit 0
