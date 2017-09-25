"use strict";
const cluster = require('cluster');
const http = require('http');
const numCPUs = require('os').cpus().length;

var greetingRe = new RegExp("^\/greeting\/([a-z]+)$", "i");

if (cluster.isMaster) {
  console.log(`Master ${process.pid} is running`);

  // Fork workers.
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }

  cluster.on('exit', (worker, code, signal) => {
    console.log(`worker ${worker.process.pid} died`);
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
            res.statusCode = 200;
            res.statusMessage = 'OK';
            res.write("Hello, " + match[1]);
        }
    }

    res.end();
  }).listen(3000);

  console.log(`Worker ${process.pid} started`);
}
