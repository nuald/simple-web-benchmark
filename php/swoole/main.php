<?php
use OpenSwoole\Http\Server;
use OpenSwoole\Http\Request;
use OpenSwoole\Http\Response;

$options = getopt("", ["port::"]);
$port = array_key_exists("port", $options) ? intval($options["port"]) : 3000;

$http = new OpenSwoole\HTTP\Server('127.0.0.1', $port);
$pattern = '/\/greeting\/([a-z]+)/';

$http->on('request', function (Request $request, Response $response) use ($pattern) {
    $response->header('Content-Type', 'text/plain; charset=utf-8');
    $uri = $request->server['request_uri'];
    if ($uri == '/') {
        $response->end('Hello World!');
    } else if (preg_match($pattern, $uri, $matches)) {
        $response->end('Hello, ' . $matches[1]);
    } else {
        $response->status(404);
        $response->end('404 Not Found');
    }
});

$pid = getmypid();
echo "Master $pid is running on port $port", PHP_EOL;
file_put_contents(".pid", $pid);

$http->start();
