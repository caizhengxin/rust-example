# -*- coding: utf-8 -*-
# @Author: jankincai
# @Date:   2022-07-08 14:18:21
# @Last Modified by:   jankincai
# @Last Modified time: 2022-07-08 14:18:36
import grpc

import helloworld_pb2
import helloworld_pb2_grpc


def run():
    with grpc.insecure_channel('127.0.0.1:50051') as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)

        req = helloworld_pb2.HelloRequest(name="foo")
        resp = stub.SayHello(req)

        print(resp)
        print(resp.message)

if __name__ == "__main__":
    run()
