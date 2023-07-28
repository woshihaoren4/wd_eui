#!/bin/bash
function help() {
  echo "服务使用docker运行，请确保已经安装并启动docker"
  echo "coordinate使用etcd作为后端存储，请提前部署一个etcd集群，并将其中一个节点链接作为第二个入参，以便测试"
  echo "start run test server : ./cmd.sh start [ETCD_URL EX:http://127.0.0.1:2379]"
  echo "clean about:            ./cmd.sh clean"
}
function start_server() {
  if [ $# -lt 2 ]; then
      help
      exit 1
  fi
  echo "start make config.toml...
---> config.toml"
  cat > ./config.toml << EOF
[[proxy_sink]]
name = "coordinate"
addr = "coordinate:6666"

[etcd]
endpoints = ["$2"]
EOF
cat ./config.toml
echo "<--
config file generate success"

echo "NOTICE : 这里将coordinate和proxy服务的配置放在了一个文件中，也可以分开配置"

echo "start create docker network..."
docker network create test_coordinate_net

echo "start run coordinate server"
docker run -d --name coordinate  --net test_coordinate_net -v $(pwd)/config.toml:/root/config.toml -p 6666:6666  wdshihaoren/coordinate:v0.0.5 ./coordinate run -c ./config.toml

echo "start run rust-grpc-proxy proxy"
docker run -d --name rust-grpc-proxy --net test_coordinate_net -v $(pwd)/config.toml:/root/config.toml -p 6789:6789  wdshihaoren/rust-grpc-proxy:v0.0.6-s ./rust-grpc-proxy run -c ./config.toml

echo "
======> server start over
run cmd test ->./cmd.sh test"
}

function test() {
    echo "run test..."
    curl --location --request GET 'http://127.0.0.1:6789/api/v1/task/search'
}

case $1 in
start)
  start_server $*
  ;;
test)
  test
  ;;
clean)
  echo "stop coordinate container..."
  docker stop coordinate
  echo "stop rust-grpc-proxy container..."
  docker stop rust-grpc-proxy
  echo "start remove docker containers..."
  docker rm coordinate rust-grpc-proxy
  echo "clean bridge network..."
  docker network rm test_coordinate_net
  echo "remove config file..."
  rm ./config.toml
  echo "clean success"
  ;;
*)
  help
  ;;
esac

