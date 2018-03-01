<?php

$options = getopt("", ["port::"]);
$port = array_key_exists("port", $options) ? intval($options["port"]) : 3000;

$http = new swoole_http_server('127.0.0.1', $port);
$pattern = '/\/greeting\/([a-z]+)/';

$http->on('request', function ($request, $response) use ($pattern) {
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
echo "Master $pid is running on port $port", PHP_EOL;

$http->start();
