# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "bento/debian-8.7"
  config.vm.box_version = "= 2.3.5"

  config.vm.provider "virtualbox" do |vb|
     vb.memory = "4096"
  end

  config.vm.provision "shell", inline: <<-SHELL
    if ! which docker >> /dev/null; then
      apt-get update
      apt-get install -y linux-image-extra-$(uname -r) linux-image-extra-virtual
      apt-get install -y apt-transport-https ca-certificates curl gnupg2 software-properties-common
      curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
      add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable"
      apt-get update
      apt-get install -y docker-ce=17.03.1~ce-0~debian-jessie
      docker run --rm hello-world && sudo docker rmi hello-world

      sudo usermod -aG docker vagrant
    fi
  SHELL

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    if ! which rustup >> /dev/null; then
      curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
      rustup install nightly-2017-04-28-x86_64-unknown-linux-gnu
      rustup default nightly-2017-04-28-x86_64-unknown-linux-gnu
    fi
  SHELL


end
