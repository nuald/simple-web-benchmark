#!/usr/bin/python3

import argparse
import os
import re
from twisted.web import server, resource
from twisted.internet import reactor, endpoints
from twisted.web.pages import notFound

parser = argparse.ArgumentParser()
parser.add_argument("--port", help="server port", default="3000")
args = parser.parse_args()
hostPort = int(args.port)
reg = re.compile("^/greeting/([a-z]+)$")


class MyServer(resource.Resource):
    isLeaf = True

    def render_GET(self, request):
        response = None
        path = request.uri.decode("utf-8")
        if path == "/":
            response = "Hello World!"
        else:
            match = reg.match(path)
            if match:
                response = "Hello, %s" % match.group(1)
        if response:
            request.setHeader(b"Content-type", b"text/plain; charset=utf-8")
            return bytes(response, "utf-8")
        else:
            return notFound().render(request)


pid = str(os.getpid())
with open(".pid", "w") as pidFile:
    pidFile.write(pid)
print("Master %s is running on port %d" % (pid, hostPort))

endpoints.serverFromString(reactor, "tcp:%d" % hostPort).listen(server.Site(MyServer()))
reactor.run()
