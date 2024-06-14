
# 创建用户
groupadd -g 60000 pgsql
useradd -u 60000 -g pgsql pgsql
echo "lhr" | passwd --stdin pgsql


# 创建目录

mkdir -p /postgresql/{archive,scripts,backup,pg16,soft}
mv ./postgresql-16.3.tar.gz /postgresql/soft/
chown -R pgsql:pgsql /postgresql
chmod -R 775 /postgresql





# 安装一些依赖包
yum install -y cmake make gcc zlib gcc-c++ perl readline readline-devel zlib zlib-devel \
perl python3 tcl openssl ncurses-devel openldap pam perl-IPC-Run libicu-devel


# 编译
su - pgsql
cd /postgresql/soft/
tar zxvf postgresql-16.03.tar.gz
cd postgresql-16.3
./configure --prefix=/postgresql/pg16
make -j 8 && make install
make world -j 8 && make install-world



# 配置环境变量
cat >>  ~/.bash_profile <<"EOF"
export LANG=en_US.UTF-8
export PS1="[\u@\h \W]\$ "
export PGPORT=5432
export PGDATA=/postgresql/pgdata
export PGHOME=/postgresql/pg16
export LD_LIBRARY_PATH=$PGHOME/lib:/lib64:/usr/lib64:/usr/local/lib64:/lib:/usr/lib:/usr/local/lib:$LD_LIBRARY_PATH
export PATH=$PGHOME/bin:$PATH:.
export DATE=`date +"%Y%m%d%H%M"`
export MANPATH=$PGHOME/share/man:$MANPATH
export PGHOST=$PGDATA
export PGUSER=postgres
export PGDATABASE=postgres
EOF

source  ~/.bash_profile



# 初始化 su - pgsql
/postgresql/pg16/bin/initdb -D /postgresql/pgdata -E UTF8 --locale=en_US.utf8 -U postgres --data-checksums



# 修改参数
cat >> /postgresql/pgdata/postgresql.conf <<"EOF"
listen_addresses = '*'
port=5432
unix_socket_directories='/postgresql/pgdata'
logging_collector = on
log_directory = 'pg_log'
log_filename = 'postgresql-%a.log'
log_truncate_on_rotation = on
EOF

cat   >> /postgresql/pgdata/pg_hba.conf << EOF
# TYPE  DATABASE    USER    ADDRESS       METHOD
local     all       all                    trust
host      all       all   127.0.0.1/32     trust
host      all       all    0.0.0.0/0        md5
host   replication  all    0.0.0.0/0        md5
EOF

# 启动 su - pgsql

pg_ctl start
pg_ctl status
pg_ctl stop

# 修改密码 post
## 创建用户
#groupadd -g 60000 pgsql
#useradd -u 60000 -g pgsql pgsql
#echo "lhr" | passwd --stdin pgsql
#
#
## 创建目录
#
#mkdir -p /postgresql/{archive,scripts,backup,pg16,soft}
#mv ./postgresql-16.3.tar.gz /postgresql/soft/
#chown -R pgsql:pgsql /postgresql
#chmod -R 775 /postgresql
#
#
#
#
#
## 安装一些依赖包
#yum install -y cmake make gcc zlib gcc-c++ perl readline readline-devel zlib zlib-devel \
#perl python3 tcl openssl ncurses-devel openldap pam perl-IPC-Run libicu-devel
#
#
## 编译
#su - pgsql
#cd /postgresql/soft/
#tar zxvf postgresql-16.03.tar.gz
#cd postgresql-16.3
#./configure --prefix=/postgresql/pg16
#make -j 8 && make install
#make world -j 8 && make install-world
#
#
#
## 配置环境变量
#cat >>  ~/.bash_profile <<"EOF"
#export LANG=en_US.UTF-8
#export PS1="[\u@\h \W]\$ "
#export PGPORT=5432
#export PGDATA=/postgresql/pgdata
#export PGHOME=/postgresql/pg16
#export LD_LIBRARY_PATH=$PGHOME/lib:/lib64:/usr/lib64:/usr/local/lib64:/lib:/usr/lib:/usr/local/lib:$LD_LIBRARY_PATH
#export PATH=$PGHOME/bin:$PATH:.
#export DATE=`date +"%Y%m%d%H%M"`
#export MANPATH=$PGHOME/share/man:$MANPATH
#export PGHOST=$PGDATA
#export PGUSER=postgres
#export PGDATABASE=postgres
#EOF
#
#source  ~/.bash_profile
#
#
#
## 初始化 su - pgsql
#/postgresql/pg16/bin/initdb -D /postgresql/pgdata -E UTF8 --locale=en_US.utf8 -U postgres --data-checksums
#
#
#
## 修改参数
#cat >> /postgresql/pgdata/postgresql.conf <<"EOF"
#listen_addresses = '*'
#port=5432
#unix_socket_directories='/postgresql/pgdata'
#logging_collector = on
#log_directory = 'pg_log'
#log_filename = 'postgresql-%a.log'
#log_truncate_on_rotation = on
#EOF
#
#cat   >> /postgresql/pgdata/pg_hba.conf << EOF
## TYPE  DATABASE    USER    ADDRESS       METHOD
#local     all       all                    trust
#host      all       all   127.0.0.1/32     trust
#host      all       all    0.0.0.0/0        md5
#host   replication  all    0.0.0.0/0        md5
#EOF
#
## 启动 su - pgsql
#
#pg_ctl start
#pg_ctl status
#pg_ctl stop
#
## 修改密码 postgres/Postgres54321001011010
#pg_ctl start
#psql
#alter user postgres with  password 'Postgres54321001011010';
#exit
#
#
## 或：nohup /postgresql/pg13/bin/postgres -D /postgresql/pgdata > /postgresql/pg13/pglog.out 2>&1 &
#
#
## 配置系统服务
## 退出当前账户，切换到root
#exit
#cat > /etc/systemd/system/PG16.service <<"EOF"
#[Unit]
#Description=PostgreSQL database server
#Documentation=man:postgres(1)
#After=network.target
#
#[Service]
#Type=forking
#User=pgsql
#Group=pgsql
#Environment=PGPORT=5432
#Environment=PGDATA=/postgresql/pgdata
#OOMScoreAdjust=-1000
#ExecStart=/postgresql/pg16/bin/pg_ctl start -D ${PGDATA} -s -o "-p ${PGPORT}" -w -t 300
#ExecStop=/postgresql/pg16/bin/pg_ctl stop -D ${PGDATA} -s -m fast
#ExecReload=/postgresql/pg16/bin/pg_ctl reload -D ${PGDATA} -s
#KillMode=mixed
#KillSignal=SIGINT
#TimeoutSec=0
#
#[Install]
#WantedBy=multi-user.target
#EOF
#
#
#
#systemctl daemon-reload
#systemctl enable PG16
#systemctl restart PG16
#systemctl status PG16gres/Postgres54321001011010
pg_ctl start
psql
alter user postgres with  password 'Postgres54321001011010';
exit


# 或：nohup /postgresql/pg13/bin/postgres -D /postgresql/pgdata > /postgresql/pg13/pglog.out 2>&1 &


# 配置系统服务
# 退出当前账户，切换到root
exit
cat > /etc/systemd/system/PG16.service <<"EOF"
[Unit]
Description=PostgreSQL database server
Documentation=man:postgres(1)
After=network.target

[Service]
Type=forking
User=pgsql
Group=pgsql
Environment=PGPORT=5432
Environment=PGDATA=/postgresql/pgdata
OOMScoreAdjust=-1000
ExecStart=/postgresql/pg16/bin/pg_ctl start -D ${PGDATA} -s -o "-p ${PGPORT}" -w -t 300
ExecStop=/postgresql/pg16/bin/pg_ctl stop -D ${PGDATA} -s -m fast
ExecReload=/postgresql/pg16/bin/pg_ctl reload -D ${PGDATA} -s
KillMode=mixed
KillSignal=SIGINT
TimeoutSec=0

[Install]
WantedBy=multi-user.target
EOF



systemctl daemon-reload
systemctl enable PG16
systemctl restart PG16
systemctl status PG16
