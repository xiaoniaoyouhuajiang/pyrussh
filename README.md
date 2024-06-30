# pyrussh
Python bindings for libssh2 on top of rust-ssh2

## motivations
* learn pyo3
* I need a high-concurrency SSH connection pool

## Features
* create a ssh session
* exec command by session
* download/upload file

## Install & Test
from pypi
```
# python3 -m pip install pyrussh
pip install pyrussh
```

developing mode:
```shell
git clone https://github.com/xiaoniaoyouhuajiang/pyrussh.git
cd pyrussh
maturin develop
```

bench test paramiko & pyrussh 
```shell
# after installing pyrussh
python benchmakrs/benchmarks.py --host <your host> --port <port> --username <..> --password <..> 
```

## Usage
### benchmark
Enter the `benchmakrs` directory and execute the corresponding Python file to compare the performance of pyrussh and Paramiko.
* ~~authentication~~
* execution
* ~~upload file~~
* read remote file

### result
test on my pc:
|time/s|paramiko|pyrussh|
|--|--|--|
|authentication|0.316176|0.077625|
|upload file|0.825472|0.632277|

## Todo
* ~~get exit code for `run_command`~~
* ~~download/upload file~~
* high level api for ssh connection
    * batch running a single command
    * batch running command

