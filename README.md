# waking-git
Git world generator. `waking-git` uses the code structure, and evolution of
your git repository to generate a world for you to explore.

The program will scan your repository, and extract from its directory and code structure
revelant data that can be used to generate a 2D/3D playable world/game. The structure
of your code (Intefaces, Classes, functions etc...) can be used to create living creatures.
Those creatures can be mobs/enemies depending on how well their structure match well known
code smell, in the programming language the code is written on.

## Development
The project is build using the `rust` programming language, and uses git as a strong dependency.

### Dependencies
- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads)
- [Docker](https://docs.docker.com/engine/install/)
- [GNU make](https://www.gnu.org/software/make/)

### Architecture

Please read the draft of the [architecture](https://github.com/elhmn/waking-git/blob/main/docs/Architecture.md).

### How to run ?

In order to run `waking-git` you need to [install](https://www.rust-lang.org/tools/install) the rust tool chain.

**How to scan a repository ?**

```console
$ cargo run scan https://github.com/elhmn/ckp
```

Or use `bevy` dynamic library
```console
$ cargo run --features bevy/dynamic scan https://github.com/elhmn/ckp
```

**How to play ?** ***(To edit as the project evolve)***

```console
$ cargo run play
```

Or use the bevy dynamic library
```console
$ cargo run --features bevy/dynamic play
```

### How to test?

Run the entire test suite using,

```console
$ make test
```

Check linting
```console
$ make lint
```

Check code format
```console
$ make fmt
```

Setup git local hooks

```console
make install-hooks
```

### More

You can run `make` OR `make help` to find out more commands
```console
$ make
 You can build `wake` using `make build`
 or run it using `cargo run scan https://github.com/elhmn/ckp`

 Choose a command...
  build           build application binary.
  test            run tests
  lint            run linter over the entire code base
  fmt             check your code format
  install-hooks   install local git hooks
```

## Resources

Building this project require to know what is the of state code scanning and data visualisation researches.
Here is a list of research papers and code visualisers that will be useful to work on this project:

- [codeology](https://demo.marpi.pl/codeology/) brings life to your source code, by generating a creature that represents your code structure
- [Gource](https://github.com/acaudwell/Gource) is a git history visualiser
- [Visual softare analytics](https://home.uni-leipzig.de/svis) (VSA) is a research group that explore different ways to represent complex software systems
- [getaviz](https://home.uni-leipzig.de/svis/getaviz/index.php?setup=web/RD%20C&model=RD%20C%20busybox&aframe=true) is a [tool](https://github.com/softvis-research/Getaviz) built by the VSA, to visualise code structure
- [static code analysers](https://github.com/analysis-tools-dev/static-analysis)
- [dynamic code analysers](https://github.com/analysis-tools-dev/dynamic-analysis)
- [Gephi](https://gephi.org)
- [Rust code analysis](https://github.com/mozilla/rust-code-analysis)
- Some research papers:
	- [1](https://www.researchgate.net/publication/328282991_Towards_an_Open_Source_Stack_to_Create_a_Unified_Data_Source_for_Software_Analysis_and_Visualization)
	- [2](https://www.researchgate.net/publication/328019593_The_Recursive_Disk_Metaphor_-_A_Glyph-based_Approach_for_Software_Visualization)
	- [3](https://www.researchgate.net/publication/328019663_Past_Present_and_Future_of_3D_Software_Visualization_-_A_Systematic_Literature_Analysis)
	- [4](https://www.researchgate.net/publication/328019394_A_Structured_Approach_for_Conducting_a_Series_of_Controlled_Experiments_in_Software_Visualization)
	- [5](https://www.researchgate.net/publication/320083290_GETAVIZ_Generating_Structural_Behavioral_and_Evolutionary_Views_of_Software_Systems_for_Empirical_Evaluation)
	- [6](https://www.researchgate.net/publication/318570435_Generative_Software_Visualization_Automatic_Generation_of_User-Specific_Visualizations)
	- [7](https://www.researchgate.net/publication/265428652_MSE_and_FAMIX_30_an_Interexchange_Format_and_Source_Code_Model_Family)
	- [8](https://www.researchgate.net/publication/281743434_How_to_Master_Challenges_in_Experimental_Evaluation_of_2D_versus_3D_Software_Visualizations)
	- [9](https://www.researchgate.net/publication/220818819_A_Visual_Analytics_Tool_for_Software_Project_Structure_and_Relationships_among_Classes)
	- [10](https://opus-htw-aalen.bsz-bw.de/frontdoor/deliver/index/docId/658/file/ICCSE16-SEE.pdf)
	- [11](https://blog.ndepend.com/visualize-code-with-software-architecture-diagrams/)
	- [12](https://www.researchgate.net/publication/347700460_rust-code-analysis_A_Rust_library_to_analyze_and_extract_maintainability_information_from_source_codes)
