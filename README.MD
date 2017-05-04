## Rust Kata ##

This repository contains an exercise in writing Rust code to search a document corpus by multiple methods and compare performance profiles. It also contains an exercise in documenting the high-level requirements, approach, design considerations, and assumptions.

- [Stated Requirements](#stated-requirements)
  * [High-Level](#high-level)
  * [Functional Requirements](#functional-requirements)
  * [Non-Functional Requirements](#non-functional-requirements)
- [Solution Approach](#solution-approach)
- [Design Considerations / Implementation Rationale](#design-considerations--implementation-rationale)
- [Assumptions](#assumptions)
- [Performance Benchmark Results and Observations](#performance-benchmark-results-and-observations)
- [Build / Usage](#build--usage)
  * [Docker Image](#docker-image)
  * [Vagrant Machine](#vagrant-machine)

## Stated Requirements ##

### High Level ###

Create a working program to search a set of documents for a user provided search term and return a list of document names that contain the term. The list should be ordered, descending, by relevance. Multiple search methods should be provided and a performance benchmark should be provided to compare their performance.

### Functional Requirements ###

Search relevance is defined as the number of times the exact term appears in the document.

Provide three methods for searching the documents, via:

- String Matching
- Regular Expression
- Pre-computed Index

The program should prompt the user to enter a search term (a single token), the desired search method, and then display the search results to the user. Each search results should contain the file name the term was found in, as well as how many occurrences of the term were found in the document (ie. [term frequency](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)).  Additionally, the program should display how long it took to complete the search, in milliseconds (ms), or if milliseconds doesn't provide sufficient resolution, in microseconds (μs).

#### Example: ####

	Enter the search term (press ENTER to exit): (?i)FTL

	Search Method: 1) String Match 2) Regular Expression 3) Pre-computed Index
	Enter method: 2

	Search results:

	  warp_drive.txt - 2 matches

	Elapsed time: 5 ms

The program should also allow the user to run a performance benchmark comparing the performance of each search method. The program should prompt the user to enter the number of searches to perform with each search method (iterations).  For each iteration, the program should select a search term at random and use that term to perform the search during the iteration.

Once all iterations run for each search method, the program should display the timing results for each method, in milliseconds (ms), or if milliseconds doesn't provide sufficient resolution, in microseconds (μs).

#### Example: ####

	How many iterations would you like to run: 100
	Results:

	  String Match: 956 μs
	  Regular Expression: 3 ms
	  Pre-computed Index: 5 ms

Sample documents are provided along with these requirements.

### Non-Functional Requirements ###

- Complete the working program in one week's time
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
	- Which search method is fastest and why?
	- What would you do re: the software or hardware to make the program scale to handle massive content and/or very large request volumes? (eg. 5000 request per second or more)

## Solution Approach ##

- A console application, written in the [Rust programming language](https://www.rust-lang.org), with an optional command line switch to run the performance benchmark.

- For the pre-computed index search method, the Tantivy search engine library will be used.

- All searching functionality is exposed via the `indexing` module. There is a `SearchMethod` enum and a `get_search_function` that exposes the functionality.

- All interaction with the Tantivy library is through the `DocumentIndex` struct/impl in the `indexing` module.

- All [CLI](https://en.wikipedia.org/wiki/Command-line_interface) interaction logic is in the "root" module.

- Unit testing follows the Rust convention of placing a `tests` sub-module next to the subject code being tested.

## Design Considerations / Implementation Rationale ##

- Since there were no functional or non-functional requirements dictating the use of a specific language or platform, I chose to use Rust.  Rust provides a fairly powerful type system, pattern matching, a novel approach to guaranteeing memory safety and concurrency without data races, and cross-platform, cross-compilation support.  All this in a "systems" programming language that can operate with "bare-metal" performance.

	My rationale for choosing Rust was that I had no prior exposure to using Rust (other than reading some documentation and blog posts) and I wanted to take advantage of a learning opportunity while meeting the requirements for the working program.  If there are aspects of the implementation that are not "idiomatic" Rust or does not take full advantage of language features and ergonomics, it is likely due to my limited experience with Rust.

- I chose to use the Tantivy library for the pre-computed index functionality because it appeared to be functional, well tested, and supported by a community. In fact, one of the lead engineers was very responsive to a question I had during my usage of the library.  That said, the internal workings of the library makes use of a standard TF-IDF score to select documents and order them by relevance.  The requirements stipulated that search results must be ordered by term-frequency (TF) alone, so the `DocumentIndex.search` function extracts the term-frequency from the indexes' [`Postings`](https://en.wikipedia.org/wiki/Inverted_index) list in-order to reorder the results as per the requirements.  Extracting the term-frequency for a single document from this postings list, although possible, is not a core-use case of the library and thus is not going to yield the best performance. 

	Additionally, it's possible that the TF-IDF score eliminates some documents that would have a high TF-based score, but a low TF-IDF score based on the search term.  This usually happens with words that frequently show up in most, if not all documents in the corpus.  Examples would be very common words like "the", "as", and "to".  Although, I did not confirm it, typically search engine libraries like Tantivy (eg. Lucene) eliminate [stop](https://en.wikipedia.org/wiki/Stop_words) and low scoring words.  This means for very common terms, the Tantivy search index based results may eliminate some results that the Simple String and Regex search methods would return.  This is technically a deviation from the requirements that relevance must be based on document term-frequency. That said, my recommendation in a real system would be to use a TF-IDF based relevance score (or some derivation) since ranking by term-frequency can often yield poor quality results.

- I could have used [ElasticSearch](https://www.elastic.co/) to index the document corpus and handle the search functionality. I chose not to for this exercise, since it is overkill for the sample document corpus, and not mandated by requirements.  However, at scale I would recommend the usage of ElasticSearch, or something similar.  It is ideal for handling massive load at scale by providing the ability to distribute load following the three dimensions (or axises) of the [Scale Cube](http://microservices.io/articles/scalecube.html).

	Along the Z axis, ElasticSearch supports dividing an index up into multiple [shards](https://en.wikipedia.org/wiki/Shard_(database_architecture)) and distributing them to separate nodes. Each shard has a fraction of the total index data, and as a result a search request sent to each shard can return its results more quickly and in parallel.

	Along the X axis, ElasticSearch supports creating [replicas](https://en.wikipedia.org/wiki/Replication_(computing)) (or copies) of each shard and placing them on separate nodes. Search request load can be shared among the nodes containing duplicate copies of the shard without needed one node to handle all requests targeting a single shard.

	Along the Y axis, ElasticSearch supports creating multiple indexes.  For data that is dissimilar or searchable independently, different indexes (and their shards and replicas) can be distributed to entirely different nodes, further distributing load across many nodes in parallel.

- Throughout the code, I wanted to make use of Rust's generics, enums, and traits to better understand how the type system works.  This includes the application ([pun intended](https://en.wikipedia.org/wiki/Function_application)) of [higher-order functions](https://en.wikipedia.org/wiki/Higher-order_function). The `get_search_function` in the `indexing` module returns a reference to another function that can be used to perform the searches using the search method specified by arguments passed into the `get_search_function`. The `time_search` and `time_work` functions in the "root" module will run a provided lambda (called "Closures" in Rust) and capture timing information.

- Rust has first-class support for the Maybe (`Option<T>`), Either (sometimes refereed as Error, `Result<T, E>`) and List (`Iterator`) "[monads](http://adit.io/posts/2013-04-17-functors,_applicatives,_and_monads_in_pictures.html)". The language strongly encourages the use of these types, often using them in Rust's standard library. But the real power of these concepts is the conciseness and readability of the code when making use of the supporting "combinator" functions. (eg. `map`, `filter`, `take`, `and_then`) Where appropriate, I tried to make use of these functions.

- There were a few places where the `.unwrap()` function is used to extract a value from a containing type. (eg. `Option<T>`)  Technically, this operation can cause a panic and the process to exit abruptly. Usually, this is used where the state of the application cannot get to this point, but I cannot guarantee this.  Given a little more time, I would like to continue to leverage the type system and libraries to eliminate any potentially dangerous `.unwrap()` calls.

- A command line argument parser library like [`docopt.rs`](https://github.com/docopt/docopt.rs) or [`clap-rs`](https://github.com/kbknapp/clap-rs) could have been used to handle argument parsing.  I chose not to since there was only one optional argument to parse and it's usage would have added another dependency and potentially slowed down completion of the program. 

- More functionality found in the "root" module could possibly be extracted into more testable forms, separated from UI logic.

## Assumptions ##

- A console application, instead of a Web API or Web Application, satisfies the requirements.

- Sample documents provided with requirements are sufficient alone to prove a working program.  It is not necessary to find or test with other collections of documents.

- All sample documents provided to this tool can easily fit into memory typically available on commodity PC hardware.

- There is no expectation that each search method returns the exact same results for a given search term.

- There is no expectation that the program must handle multi-word/term searches ([*n*-grams](https://en.wikipedia.org/wiki/N-gram))

- There is no expectation that the pre-computed index functionality be directly implemented, but can be deferred to an existing library.

- Not all code needs to be unit tested, nor does usage of code coverage analyses tools need to be demonstrated.


## Performance Benchmark Results and Observations ##

The performance benchmark pre-loads all sample documents into memory and creates the Tantivy index before prompting the user for the number of iterations to run.  From this point, all search execution goes against in-memory data structures.  Performance characteristics of [information retrieval](https://en.wikipedia.org/wiki/Information_retrieval) ("search") algorithms vary greatly based on search patterns, data structures, and storage media characteristics (let's hope technologies like [Intel Optane](http://www.intel.com/content/www/us/en/architecture-and-technology/intel-optane-technology.html) because an available and prevalent option). 

A performance benchmark run of 2 million iterations, running on an 3.2Ghz Windows 10 x64 machine, yielded the following output from the Rust Kata program:

	(                         )
	)\ )              )    ( /(          )
	(()/(   (       ( /(    )\())   )  ( /(    )
	/(_)) ))\  (   )\()) |((_)\ ( /(  )\())( /(
	(_))  /((_) )\ (_))/  |_ ((_))(_))(_))/ )(_))
	| _ \(_))( ((_)| |_   | |/ /((_)_ | |_ ((_)_
	|   /| || |(_-<|  _|  | ' < / _` ||  _|/ _` |
	|_|_\ \_,_|/__/ \__|  |_|\_\\__,_| \__|\__,_|
	
	Building in-memory index of sample input files...
	
	Executing naive performance benchmark.
	
	How many iterations would you like to run: 2000000
	Results:
	
		String Match: 13742 ms
		Regular Expression: 64036 ms
		Tantivy Index: 23967 ms
	
	Thank you, come again!

The best performing solution in this case is the String Match. Because the corpus is so small, it's not surprising that it performs well.  However, as the corpus grows in size, so will the [time-complexity](https://en.wikipedia.org/wiki/Time_complexity) of a String Match approach, and not in a favorable way.

Regular Expression is the worst of the three options and likely would remain so.  I would not recommend this approach for many reasons. For each search invocation, the regular expression pattern needs to be parsed into a pattern-matching state machine. This is costly from a time-complexity and [space-complexity](https://en.wikipedia.org/wiki/DSPACE) perspective since memory needs to be allocated for the resulting state machine. Adding to that, the time-complexity of this approach is hard to quantify because the user provided regular expression can drastically change the runtime characteristics.  This is even more concerning when looked at from a security perspective: making use of user-provided regular expressions is a [known attack vector](https://en.wikipedia.org/wiki/ReDoS).  Often it is trivial to craft an regular expression that takes a large amount of processing time and can be used as a [DoS attack](https://en.wikipedia.org/wiki/Denial-of-service_attack). Finally, regular expressions are very difficult for end-users (sometimes for engineers) to author and read - which lead to the engineering tribal wisdom that you've just [added another problem](https://xkcd.com/1171/)! 

The Tantivy index approach, in this run, is in the middle of the pack, performance wise, and I would venture that is due to overhead related to retrieving term-frequency for each document and resorting the results.  Additionally, I would venture that a moderately larger document corpus would find the current implementation of Tantivy Index searching the front runner.  The underlying data structure, an [inverted index](https://en.wikipedia.org/wiki/Inverted_index) is designed to have it's time-complexity grow sub-linearly as the corpus grows.

Most, if not all, Information Retrieval strategies are trade-offs between time-complexity and space-complexity - in other words, trading CPU time for memory/disk space.  Designing your system to operate with acceptable performance characteristics at scale often involves tuning your application along the three axises of the [Scale Cube](http://microservices.io/articles/scalecube.html) and the finding an appropriate trade-off between time and space, CPU and storage.

## Build / Usage ##

In order to build and run this project you need to have the following prerequisites installed and accessible via the PATH environment variable (tested on `Debian x64 8.7` patched as of `2017-04-24`):

- [Rust 1.18 Nightly](https://github.com/rust-lang-nursery/rustup.rs) (tested with version `128aa262e 2017-04-28`)

Clone this repository into a directory on your local machine. In a terminal, navigate into the new project directory and run the `cargo build --release`.  This should compile the `rustkata` program.  NOTE: The first time this is done, it may take a while - the dependent packages ("crates") are being downloaded and compiled.  

The executable should now be in the `target/release` directory. Typing `./target/release/rustkata` (or on Windows `.\target\release\rustkata.exe`) should run the program for interactive searching. Adding the argument `--mode perf` will run the program for performance benchmarking.

### Docker Image ###
A [Docker](https://www.docker.com/) image has been built and published to the public Docker hub.  It's designed to be run interactively.  To run the image, you must have the following prerequisites installed and and accessible via the PATH environment variable:

- Docker 17.03.1-ce

At a terminal, type `docker run --rm -it jordanterrell/rustkata:0.1.0`.  This will run the Rust Kata application in a container and allow you to interact with the container.  If you would like to run the performance benchmark, type `docker run --rm -it jordanterrell/rustkata:0.1.0 --mode perf`.  On *nix based systems, you many need to preface each command with `sudo` since running containers is considers an administrative operation. For example, `sudo docker run --rm -it jordanterrell/rustkata:0.1.0`.

### Vagrant Machine ###

A [Vagrant](https://www.vagrantup.com/) machine has been included in the repository with an environment containing the prerequisites necessary to build and run the application already installed.  To make use of the Vagrant machine, make sure the following prerequisites are installed:

- Vagrant 1.9.2
- [VirtualBox](https://www.virtualbox.org/wiki/VirtualBox) 5.1.6
	- NOTE: VirtualBox is known to have difficulties running at the same time Hyper-V is installed (on Windows) or when Parallels is running (on OS X).

After cloning this repository, navigate to the root of the repository and type `vagrant up`. This will bring up the Vagrant machine. NOTE: This may take a few minutes, especially the first time.  Once the command completes, type `vagrant ssh`. This will connect your terminal to the running Vagrant machine.  Navigate to the `/vagrant` directory and now you will be in the root of the project repository.  Use the instructions above to build and run the `rustkata` program.  Any changes made to the source code (on the host machine or in the Vagrant machine) can be re-compiled and run from the VM.

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