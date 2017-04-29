## Rust Kata ##

This repository contains an exercise in writing Rust code to search a document corpus by multiple methods and compare performance profiles. It also contains an exercise in documenting the high-level requirements, approach, design considerations, and assumptions.

- [Build / Usage](#build--usage)
  * [Vagrant Machine](#vagrant-machine)

## Build / Usage ##

In order to build and run this project you need to have the following prerequisites installed and accessible via the PATH environment variable (tested on `Debian x64 8.7` patched as of `2017-04-24`):

- Rust 1.18 Nightly (tested with version `128aa262e 2017-04-28`)

Clone this repository into a directory on your local machine. Navigate into the new project directory and run the `cargo build --release`.  This should compile the `rustkata` program.  NOTE: The first time this is done, it may take a while - the dependent packages ("crates") are being downloaded and compiled.  The executable should be in the `target/release` directory. Typing `./target/release/rustkata` should run the program.

### Vagrant Machine ###

A [Vagrant](https://www.vagrantup.com/) machine has been included in the repository with an environment containing the prerequisites necessary to build and run the application already installed.  To make use of the Vagrant machine, make sure the following prerequisites are installed:

- Vagrant 1.9.2
- VirtualBox 5.1.6
	- NOTE: VirtualBox is known to have difficulties running at the same time Hyper-V is installed (on Windows) or when Parallels is running (on OS X).

At the root of the repository, type `vagrant up`. This will bring up the Vagrant machine. NOTE: This may take a few minutes, especially the first time.  Once the command completes, type `vagrant ssh`. This will connect your terminal to the running Vagrant machine.  Navigate to the `/vagrant` directory and now you will be in the root of the project repository.  Use the instructions above to build and run the `rustkata` program.  Any changes made to the source code (on the host machine or in the Vagrant machine) can be re-compiled and run from the VM.

	vagrant up
	# wait a few minutes
	vagrant ssh
	cd /vagrant
	cargo build --release
	# rustkata program will be compiled
	./target/release/rustkata


Once you are done, type `exit` into the shell. This will disconnect your terminal from the running Vagrant machine. Type `vagrant destroy -f` to terminate the Vagrant machine and wipe it from your system. NOTE: The base Debian image that was downloaded the first time `vagrant up` was run will still remain on your system. If you wish to remove this image, follow the documentation on the [Vagrant](https://www.vagrantup.com/docs/cli/box.html/) website.

## License ##

Copyright © 2017, [Jordan Terrell](https://github.com/iSynaptic).
Released under the [MIT license](https://github.com/iSynaptic/rust-kata/blob/master/LICENSE).