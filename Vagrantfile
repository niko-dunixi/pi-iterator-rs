# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/bionic64"
  config.vm.synced_folder ".", "/home/vagrant/pi-iterator-rs"

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    set -ex
    sudo apt-get update
    sudo apt-get install -y \
      build-essential \
      clang \
      linux-headers-"$(uname -r)" \
      llvm \
      make
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    rustup toolchain install nightly
    rustup default nightly
    rustup component add --toolchain=nightly rust-src rustfmt
  SHELL
end
