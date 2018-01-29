<?php

$http = new swoole_http_server('127.0.0.1', 3000);

$http->on('request', function ($request, $response) {
    $pattern = "/\/greeting\/([a-z]+)/";
    $response->header('Content-Type', 'text/plain; charset=utf-8');
    $uri = $request->server['request_uri'];
    if ($uri == '/') {
        $response->end('Hello World!');
    } else if (preg_match($pattern, $uri, $matches)) {
        $response->end('Hello, ' . $matches[1]);
    } else {
        $response->status(404);
        $response->end();
    }
});

$pid = getmypid();
echo "Master $pid is running", PHP_EOL;

$http->start();
