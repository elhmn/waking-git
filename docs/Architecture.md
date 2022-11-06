# Waking-git architecture (draft)

This is a rough description of what the program architecture is supposed to look like.


## How does it work ?

- We first pass to the program a `git` repository `http` or `https` url.
	The repository url can come from github.com, gitlab.com or any other source.
- The repository will then be scanned and a set of relevant data will be extracted from the repository tree and source code.
	This data will be stored in a directory named after a slug of your repository `owner/name`.
- The Player will use data extracted from your repository to create a world to explore.


### Code scanner:

The code scanner receives an https url and an optional commit `reference`
to download the repository content, if the reference is not specified
the scanner will use the main branch as default reference and download.

```console
$ wake scan https://github.com/elhmn/ckp <reference>
```

The code scanner will analyse your repository tree structure and your source file,
and generate a set of files that will later be used by the world generator to create
a playable 2D or 3D world.


### The player:

The player uses data extracted by the `code scanner` to generate a playable world.

```console
$ wake play --dir <path_to_the_directory_generated_by_the_code_scanner>
```

you can use the `--dir` flag to set the directory that should be used to generate the world
use to create the world to explore.

The player can also use the `https` url of your source code. This will automatically call
the scanner and create the playable world created by the scanner.

```console
$ wake play <the_https_url_of_your_source_code>
```
