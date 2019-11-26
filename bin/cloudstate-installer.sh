#!/bin/bash
RELEASE_URL=https://api.github.com/repos/sleipnir/cloudstate-cli/releases/latest
RELEASE_TYPE=linux

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
    RELEASE_TYPE=osx

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
:    #        # ...
  else
    echo "Install docker"
    # command
fi

#TODO: Extract function
# Verify Kubectl
if [[ $(which kubectl) && $(kubectl version) ]]; then
:    #        # ...
  else
    echo "Install kubectl"
    # command
fi

#TODO: Extract function
# Verify Minikube
if [[ $(which minikube) && $(minikube status) ]]; then
:    #        # ...
  else
    echo "Install minikube"
    # command
fi

#TODO: Extract function
# Verify Curl
if [[ $(which curl) && $(curl --version) ]]; then
:    #        # ...
  else
    echo "Install curl"
    # command
fi

# Download binary
echo "Donwloading binaries..."
curl -H 'Cache-Control: no-cache' -s $RELEASE_URL \
  | grep browser_download_url \
  | grep $RELEASE_TYPE.tar.gz \
  | cut -d '"' -f 4 \
  | wget -qi -

tar -zxvf cloudstate*.tar.gz --directory /usr/local/bin
chmod +x /usr/local/bin/cloudstate
rm -rf cloudstate*.tar.gz

# Install completions
echo "Installing completions..."
cloudstate completions bash >> $BASH_COMPLETION_DIR/cloudstate.bash-completion
sed -i '$ d' $BASH_COMPLETION_DIR/cloudstate.bash-completion
source $BASH_COMPLETION_DIR/cloudstate.bash-completion

# Check dependencies
echo "Checking dependencies...."
cloudstate --check

echo "Install Finish $(cloudstate --version) "

exit 0
