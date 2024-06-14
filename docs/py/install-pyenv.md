# Pyenv

Install Multiple Python Versions for Specific Project

- Home project: https://github.com/pyenv/pyenv

- Reference to: https://www.tecmint.com/pyenv-install-and-manage-multiple-python-versions-in-linux/

## 1. Install pyenv in Linux

### 1.1. Install all the required packages

```shell
# On Debian/Ubuntu/Linux Mint ------------ 
sudo apt install curl git-core gcc make zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev libssl-dev

# On CentOS/RHEL ------------
sudo yum -y install epel-release
sudo yum -y install git gcc zlib-devel bzip2-devel readline-devel sqlite-devel openssl-devel

# On Fedora 22+ ------------
sudo yum -y install git gcc zlib-devel bzip2-devel readline-devel sqlite-devel openssl-devel
```

### 1.2. Grab the the latest **pyenv** source tree from its Github repository

```shell
git clone https://github.com/pyenv/pyenv.git $HOME/.pyenv
```

### 1.3. Set the environment variable **PYENV_ROOT**

```shell
vim $HOME/.bashrc
```

```bash
## pyenv configs
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"

if command -v pyenv 1>/dev/null 2>&1; then
  eval "$(pyenv init -)"
fi
```

### 1.4. source **$HOME/.bashrc** file or restart the shell

```shell
source $HOME/.bashrc
# or:
exec "$SHELL"
```

## 2. How to install Multiple Python Versions in Linux

```shell
# View all available versions with this command.
pyenv install -l

# You can now install multiple Python version via pyenv, for example.
pyenv install 3.6.4
pyenv install 3.6.5

# List all Python versions available to pyenv
pyenv versions

# Check the global Python version
pyenv global

# Set the global python version using the pyenv command
pyenv global 3.6.5
pyenv global

# Set the local Python version on per-project basis
# For instance, if you have a project located in $HOME/python_projects/test,
# you can set the Python version of it using following command.
cd python_projects/test
pyenv local 3.6.5
pyenv version		#view local python version for a specific project, or:
pyenv versions
```

## 3. Extra:

Pyenv manages virtual environments via the **pyenv-virtualenv plugin** which automates management of **virtualenvs** and **conda** environments for Python on Linux and other UNIX-like systems.

### 3.1. Installing this plugin using following commands

```shell
git clone https://github.com/yyuu/pyenv-virtualenv.git $HOME/.pyenv/plugins/pyenv-virtualenv
source $HOME/.bashrc
```

### 3.1. Create a test virtual environment

called **venv_project1** under a project called **project1** as follows

```shell
cd python_projects
mkdir project1
cd project1
pyenv virtualenv 3.6.5 venv_project1
```

### operation

pyenv activate [project_env]
pyenv deactivate 切换到系统环境

``` 
# 创建两个虚拟环境
pyenv virtualenv 3.6.0 py36_Mao
pyenv virtualenv 2.7.13 py33_Mao
# 进入 3.5 环境进行工作
pyenv activate py35_Mao
# 进入 3.3 环境进行工作
pyenv activate py33_Mao
# 结束后离开虚拟环境
pyenv deactivate


```

删除操作

``` 

# 如果以后不再使用虚拟开发环境, 删除环境
rm -rf ~/.pyenv/versions/py35_Mao/
rm -rf ~/.pyenv/versions/3.5.2/envs/py35_Mao
rm -rf ~/.pyenv/versions/py33_Mao/
rm -rf ~/.pyenv/versions/3.3.2/envs/py33_Mao


```

install
``` 
apt install -y nasm zlib1g-dev libssl-dev libre2-dev libb64-dev locales libsm6 libxext6 libxrender-dev libgl1 python3-dev python3-pip git
apt-get install libbz2-dev
pip install git+https://github.com/pypdfium2-team/ctypesgen@pypdfium2
pip install -r requirements.txt -V

```

## issue


### ModuleNotFoundError: No module named '_bz'

find / -name _bz2.cpython-*-x86_64-linux-gnu.so

找到对应的_bz 文件，拷贝到对应的运行版本中。

find / -name lib-dynload

找到运行应用的python版本下的lib-dynload， 将so文件copy到此目录下面修改对应的python版本好

例如，python 37版本  “_bz2.cpython-37m-x86_64-linux-gnu.so” 改成 python 39 版本“_bz2.cpython-39-x86_64-linux-gnu.so”。

### ModuleNotFoundError: No module named '_sqlite3'

和上面一样的操作，找到_sqlite3 so 文件。