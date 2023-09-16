gen:
	protoc -I=./protocol --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ./protocol/api/api.proto
