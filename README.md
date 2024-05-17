# pyssh
Python bindings for libssh2 on top of rust-ssh2

## motivations
* learn pyo3, ssh, rust
* I need a high-concurrency SSH connection pool

## Features
* create a ssh session
* exec command by session

## Install
```shell
git clone https://github.com/xiaoniaoyouhuajiang/pyssh.git
cd pyssh
maturin develop
```

## Usage
### benchmark
Enter the `benchmakrs` directory and execute the corresponding Python file to compare the performance of pyssh and Paramiko.
* ~~authentication~~
* execution
* close
* read output

## Todo
* get exit code for `run_command`
* high level api for ssh connection
    * batch running a single command
    * batch running command

