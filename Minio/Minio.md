# 图片服务器使用[MinIO](https://min.io/download)

MinIO 是一个基于Apache License v2.0开源协议的对象存储服务。它兼容亚马逊S3云存储服务接口，非常适合于存储大容量非结构化的数据，例如图片、视频、日志文件、备份数据和容器/虚拟机镜像等，而一个对象文件可以是任意大小，从几kb到最大5T不等。

可以应用这个搭建了简单的文件服务，进行论文和图片的存储

[在线文档](https://docs.min.io/cn/minio-quickstart-guide.html)


## 安装和试用

* 下载

[https://min.io/download](https://min.io/download)

`MINIO SERVER`和`MINIO CLIENT`都需要下载

* 启动
```bash
minio.exe server minio
# COMMANDS:
#   server   start object storage server
#   minio    保存数据用的目录

# win可以这样
# start "minioadmin" .\minio.exe server .\data
```

* 登录
```bash
# http://ip:9000/minio/login
# 默认使用如下登录
minioadmin
minioadmin
```

* 测试
1. 点击右下角创建bucket
2. 点击右下角upload

**注意配置php和minio的最大文件限制**

* 配置文件服务器公开访问

[完全体文档](https://docs.minio.io/cn/minio-client-complete-guide.html)

Manage anonymous bucket policies to a bucket and its contents
```bash
USAGE:
  mc policy [FLAGS] set PERMISSION TARGET
  mc policy [FLAGS] set-json FILE TARGET
  mc policy [FLAGS] get TARGET
  mc policy [FLAGS] get-json TARGET
  mc policy [FLAGS] list TARGET

PERMISSION:
  Allowed policies are: [none, download, upload, public].

FILE:
  A valid S3 policy JSON filepath.

FLAGS:
  --help, -h                       show help
```

Set anonymous bucket policy for mybucket/myphotos/2020/ sub-directory and its objects to download only. Now, objects under the sub-directory are publicly accessible. e.g mybucket/myphotos/2020/yourobjectnameis available at https://play.min.io:9000/mybucket/myphotos/2020/yourobjectname


```bash
# 管理配置文件
mc config host add minioadmin http://127.0.0.1:9000 minioadmin minioadmin

# 显示当前匿名存储桶策略
.\mc.exe policy list minioadmin/minioadmin

# 设置可下载的匿名存储桶策略
.\mc.exe policy set download minioadmin/minioadmin/

.\mc.exe policy set public minioadmin/minioadmin/
```


## 配置minio的laravel插件

不一定用得上，因为已经安装好了

```bash
# 查看当前镜像源
composer config -l

# 更新
composer self-update -vvv

# 设置Packagist中国全量镜像
composer config repo.packagist composer https://packagist.phpcomposer.com
# 或者这个
composer config repo.packagist composer https://mirrors.aliyun.com/composer/

# 下载插件
composer require aws/aws-sdk-php-laravel ~3.0

## 指定PHP版本 指定composer 指定载入包
# /usr/local/php7/bin/php composer /usr/local/bin/require james.xue/login-captcha
```

## minio初始化
<!-- export MINIO_ACCESS_KEY=username
export MINIO_SECRET_KEY=password
nohup sudo /usr/local/minio/minio server --address=0.0.0.0:9000 --config-dir /etc/minio /data/minioData > /usr/local/minio/minio.log 2>&1& -->

**用户名、密码、IP，需要和config\aws.php中的一致**

```powershell
# 设置用户名和密码，仅一次有效；*nix用export吧
set MINIO_ACCESS_KEY=minioadmin
set MINIO_SECRET_KEY=minioadmin

# 启动服务
start "文件服务器 - minio" .\minio.exe server miniofiles

# 配置，

.\mc.exe config host add minioadmin http://127.0.0.1:9000 minioadmin minioadmin

.\mc.exe mb minioadmin/images
.\mc.exe mb minioadmin/papers

.\mc.exe policy list minioadmin/images
.\mc.exe policy set download minioadmin/images
.\mc.exe policy set public minioadmin/images

.\mc.exe policy list minioadmin/papers
.\mc.exe policy set download minioadmin/papers
.\mc.exe policy set public minioadmin/papers
```

## minio启动
```powershell
# 设置用户名和密码，仅一次有效；*nix用export吧
set MINIO_ACCESS_KEY=minioadmin
set MINIO_SECRET_KEY=minioadmin

# 启动服务
start "文件服务器 - minio" .\minio.exe server miniofiles
```
