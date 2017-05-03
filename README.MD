## Rust Kata ##

This repository contains an exercise in writing Rust code to search a document corpus by multiple methods and compare performance profiles. It also contains an exercise in documenting the high-level requirements, approach, design considerations, and assumptions.

- [Stated Requirements](#stated-requirements)
  * [High-Level](#high-level)
  * [Functional Requirements](#functional-requirements)
  * [Non-Functional Requirements](#non-functional-requirements)
- [Solution Approach](#solution-approach)
- [Performance Benchmark Results and Observations](#performance-benchmark-results-and-observations)
- [Design Considerations / Implementation Rationale](#design-considerations--implementation-rationale)
- [Assumptions](#assumptions)
- [Build / Usage](#build--usage)
  * [Docker Image](#docker-image)
  * [Vagrant Machine](#vagrant-machine)

## Stated Requirements ##

### High Level ###

Create a working program to search a set of documents for a user provided search term and return a list of document names that contain the term. The list should be ordered, descending, by relevance. Multiple search methods should be provided and their performance should be comparable.

### Functional Requirements ###

Search relevance is defined as the number of times the exact term appears in the document.

Provide three methods for searching the documents, via:

- String Matching
- Regular Expression
- Precomputed Index

The program should prompt the user to enter a search term (a single token), the desired search method, and then display the search results to the user. Each search results should contain the file name the term was found in, as well as how many occurances of the term were found in the document (ie. [term frequency](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)).  Additionally, the program display how long it took to complete the search, in milliseconds (ms), or if milliseconds doesn't provide sufficient resolution, in microseconds (μs).

#### Example: ####

	Enter the search term (press ENTER to exit): (?i)FTL

	Search Method: 1) String Match 2) Regular Expression 3) Precomputed Index
	Enter method: 2

	Search results:

	  warp_drive.txt - 2 matches

	Elapsed time: 5 ms

The program should also allow the user to run a performance benchmark comparing the performance of each search method. The program should prompt the user to enter the number of searches to perform with each search method (iterations).  For each iteration, the program should select a search term at random and use that term to perform the search during the iteration.

Once all iterations run for each search method, the program should display the timing results for each method, in milliseconds (ms), or if milliseconds doesn't provide sufficient resolution, in microseconds (μs).

#### Example: ####

	How many iterations would you like to run: 2000000
	Results:

	  String Match: 13903 ms
	  Regular Expression: 61830 ms
	  Precomputed Index: 24088 ms	

### Non-Functional Requirements ###

- Any technical stack can be used
	- If appropriate, use a 3rd-party data store
  - Any 3rd-party frameworks/libraries can be used
- Provide evidence of a working program. One of the following:
	- Unit test results or other output
	- Hosted instance of the working program
	- Runnable instance on a computer
- Must provide source code
- Must implement a few executable tests
- Provide documentation on how to run and test the application, as well as any other relevant details
- Document the thought process and answer the following questions:
	- Which seach method is fastest and why?
	- What would you do re: the software or hardware to make the program scale to handle massive content and/or very large request volumes? (eg. 5000 request per second or more)

## Build / Usage ##

In order to build and run this project you need to have the following prerequisites installed and accessible via the PATH environment variable (tested on `Debian x64 8.7` patched as of `2017-04-24`):

- [Rust 1.18 Nightly](https://github.com/rust-lang-nursery/rustup.rs) (tested with version `128aa262e 2017-04-28`)

Clone this repository into a directory on your local machine. In a terminal, navigate into the new project directory and run the `cargo build --release`.  This should compile the `rustkata` program.  NOTE: The first time this is done, it may take a while - the dependent packages ("crates") are being downloaded and compiled.  The executable should be in the `target/release` directory. Typing `./target/release/rustkata` (or on Windows `.\target\release\rustkata.exe`) should run the program.

### Docker Image ###
A Docker image has been built and uploaded to the public Docker hub.  It's designed to be run interactively.  To run the image, you must have the following prerequisites installed:

- Docker 17.03.1-ce

At a terminal prompt, type `docker run --rm -it jordanterrell/rustkata:0.1.0-rc.2`.  This will run the Rust Kata application in a container and allow you to interact with the container.  If you would like to run the performance benchmark, type `docker run --rm -it jordanterrell/rustkata:0.1.0-rc.2 --mode perf`.

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