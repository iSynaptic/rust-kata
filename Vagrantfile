# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "bento/debian-8.7"
  config.vm.box_version = "= 2.3.5"

  config.vm.provider "virtualbox" do |vb|
     vb.memory = "4096"
  end

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    if ! which rustup >> /dev/null; then
      curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
      rustup install nightly-2017-04-28-x86_64-unknown-linux-gnu
      rustup default nightly-2017-04-28-x86_64-unknown-linux-gnu
    fi
  SHELL
end
