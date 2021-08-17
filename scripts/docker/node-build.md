这部分教程包含的内容主要包含

1.用dockerfile构建一个新的镜像

2.用docker-compose启动容器，包括参数的配置

下载代码：

```shell
git clone https://github.com/jianlexia/assets_nft.git
```



编译代码，注意这里编译的环境必须要和镜像的系统版本一致。

```sh
cd assets_nft

cargo build --release
```



用编译好的可执行文件来构建镜像，Dockerfile文件在项目根目录。镜像初始文件较大，需要网络较好的环境。

```shell
docker build -t xiajianle/gwinode:v1.0.1 .
```



推送镜像到远程仓库

```
docker login

docker push  xiajianle/gwinode:v2.0
```



其他服务器上需要运行节点，则通过/scripts/docker/目录下的docker-compose.yml文件来启动容器。需要做节点的定制信息配置。

其中--name、node-key需要进行配置。

修改环境变量 ，拷贝并修改文件内容

```
cp example.env .env
```



启动容器命令如下

```sh
docker-compose up gwinode
```



成功启动之后，需要再注入aura和gran的sessionKey。

在/scripts/docker/ 目录下配置好aura.json和gran.json两个文件。

然后执行命令进行sessionKey的注入

```shell
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@aura.json";

curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@gran.json"
```



节点启动之后，可以在监控应用上统一查看

https://telemetry.polkadot.io/#/GWI%20live%20net

