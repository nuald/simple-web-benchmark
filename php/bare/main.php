<?php

$pattern = '/\/greeting\/([a-z]+)/';

header('Content-Type', 'text/plain; charset=utf-8');
$uri = $_SERVER["REQUEST_URI"];
if ($uri == '/') {
    echo 'Hello World!';
} else if (preg_match($pattern, $uri, $matches)) {
    echo 'Hello, ' . $matches[1];
} else {
    return false;
}
