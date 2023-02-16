#!/usr/bin/python3

import argparse
import os
import re
from http.server import BaseHTTPRequestHandler, HTTPServer

hostName = "127.0.0.1"
parser = argparse.ArgumentParser()
parser.add_argument("--port", help="server port", default="3000")
args = parser.parse_args()
hostPort = int(args.port)
reg = re.compile("^/greeting/([a-z]+)$")


class MyServer(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        response = None
        if self.path == "/":
            response = "Hello World!"
        else:
            match = reg.match(self.path)
            if match:
                response = "Hello, %s" % match.group(1)
        if response:
            self.send_header("Content-type", "text/plain; charset=utf-8")
            self.end_headers()
            self.wfile.write(bytes(response, "utf-8"))
        else:
            self.send_error(404)

    def log_message(self, format, *args):
        return


myServer = HTTPServer((hostName, hostPort), MyServer)
pid = str(os.getpid())
with open(".pid", "w") as pidFile:
    pidFile.write(pid)
print("Master %s is running on port %d" % (pid, hostPort))

try:
    myServer.serve_forever()
except KeyboardInterrupt:
    pass

myServer.server_close()
