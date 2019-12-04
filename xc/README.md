# Cross Compile Rust Project
This directory contains a script and Dockerfiles that allow this
project to be cross compiled if docker is installed.

To cross-compile to known target architectures, simply run the
`rust-ports.sh` script. It will create a rust-ports directory
in the root directory with the desired release executables.

The format of the Dockerfile names is Dockerfile.`OsName`-`OsVersion`.
