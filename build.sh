mkdir protoc_tmp
cd protoc_tmp

wget https://github.com/protocolbuffers/protobuf/releases/download/v29.3/protoc-29.3-linux-x86_64.zip
unzip protoprotoc-29.3-linux-x86_64.zip

cd ..
PROTOC=$(realpath protoc_tmp/bin/protoc) cargo build --release
rm -rf protoc_tmp
