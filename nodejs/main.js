"use strict";
const cluster = require('cluster');
const http = require('http');
const numCPUs = require('os').cpus().length;

var greetingRe = new RegExp("^\/greeting\/([a-z]+)$", "i");
var portRe = new RegExp("^--port=(\\d+)$", "i");

var port = 3000;
process.argv.forEach((val, index) => {
  var match = portRe.exec(val);
  if (match) {
    port = parseInt(match[1]);
  }
});

if (cluster.isMaster) {
  console.log(`Master ${process.pid} is running on port ${port}`);

  // Fork workers.
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }

  cluster.on('exit', (worker, code, signal) => {
    console.log(`Worker ${worker.process.pid} terminated`);
  });
} else {
  // Workers can share any TCP connection
  // In this case it is an HTTP server
  http.createServer((req, res) => {
    var match;

    switch (req.url) {
        case "/": {
            res.statusCode = 200;
            res.statusMessage = 'OK';
            res.write("Hello World!");
            break;
        }

        default: {
            match = greetingRe.exec(req.url);
            if (match) {
              res.statusCode = 200;
              res.statusMessage = 'OK';
              res.write("Hello, " + match[1]);
            } else {
              res.statusCode = 404;
              res.write('Not found');
            }
        }
    }

    res.end();
  }).listen(port);

  console.log(`Worker ${process.pid} started`);
}
