[![Build, Test, & Release 🚀](https://github.com/bbeesley/push-fns/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/bbeesley/push-fns/actions/workflows/build-and-test.yml)

# push-fns
A simple library for packaging up serverless function code and uploading to a bucket for use in lambda or cloud functions

## Usage

**Command Overview:**

* [`push-fns`↴](#push-fns)
* [`push-fns aws`↴](#push-fns-aws)
* [`push-fns gcp`↴](#push-fns-gcp)

## `push-fns`

A simple tool to upload serverless function assets

**Usage:** `push-fns <COMMAND>`

###### **Subcommands:**

* `aws` — Zips up function assets and uploads them to AWS S3 for use in lambda functions. Optionally creates a file for a layer as well as a file for the function itself
* `gcp` — Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions



## `push-fns aws`

Zips up function assets and uploads them to AWS S3 for use in lambda functions. Optionally creates a file for a layer as well as a file for the function itself

**Usage:** `push-fns aws [OPTIONS] --buckets <BUCKETS> --regions <REGIONS> --function-key <FUNCTION_KEY>`

###### **Options:**

* `-i`, `--include <INCLUDE>` — An array of globs defining what to bundle

  Default value: `**`
* `-e`, `--exclude <EXCLUDE>` — An array of globs defining what not to bundle
* `-b`, `--buckets <BUCKETS>` — A list of buckets to upload to (same order as the regions please)
* `-r`, `--regions <REGIONS>` — A list of regions to upload the assets to
* `-f`, `--function-key <FUNCTION_KEY>` — The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
* `-p`, `--input-path <INPUT_PATH>` — The path to the lambda code and node_modules (default ".")

  Default value: `.`
* `-l`, `--layer-key <LAYER_KEY>` — Tells the module to split out the node modules into a zip that you can create a lambda layer from
* `--layer-globs <LAYER_GLOBS>` — An array of globs defining what to include in the layer zip

  Default value: `node_modules/**`
* `-v`, `--version-suffix <VERSION_SUFFIX>` — An optional string to append to layer and function keys to use as a version indicator
* `--root-dir <ROOT_DIR>` — An optional path within the zip to save the files to
* `-s`, `--symlink-node-modules` — Should we create a symlink from the function directory to the layer node_modules?

  Default value: `false`



## `push-fns gcp`

Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions

**Usage:** `push-fns gcp [OPTIONS] --buckets <BUCKETS> --function-key <FUNCTION_KEY>`

###### **Options:**

* `-i`, `--include <INCLUDE>` — An array of globs defining what to bundle

  Default value: `**`
* `-e`, `--exclude <EXCLUDE>` — An array of globs defining what not to bundle
* `-b`, `--buckets <BUCKETS>` — A list of buckets to upload to (same order as the regions please)
* `-f`, `--function-key <FUNCTION_KEY>` — The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
* `-p`, `--input-path <INPUT_PATH>` — The path to the lambda code and node_modules (default ".")

  Default value: `.`
* `-v`, `--version-suffix <VERSION_SUFFIX>` — An optional string to append to layer and function keys to use as a version indicator
* `--root-dir <ROOT_DIR>` — An optional path within the zip to save the files to



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
