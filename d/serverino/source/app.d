module app;

import std.getopt;
import std.process: thisProcessID;
import std.regex: ctRegex, matchFirst;
import std.stdio: File, writeln;
import std.parallelism: totalCPUs;

import serverino;

auto ctr = ctRegex!("/greeting/([a-z]+)$");

mixin ServerinoMain;
ushort port = 3000;

@onDaemonStart save_pid() {
    auto f = File(".pid", "w+");
    f.write(thisProcessID);
    writeln(i"Master $(thisProcessID) is running on port $(port)");
}

@onServerInit ServerinoConfig configure(string[] args) {
    auto info = getopt(args, "port", &port);

    return ServerinoConfig
        .create()
        .enableKeepAlive()
        .addListener("0.0.0.0", port)
        .setDaemonInstances(totalCPUs);
}

@endpoint void hello(Request req, Output output) {
    const path = req.path;
    if (path == "/")
        output ~= "Hello world!";
    else {
        auto ch = path.matchFirst(ctr);
        if (!ch.empty)
            output ~= "Hello, " ~ ch[1];
        else {
            output.status = 404;
            return;
        }
    }
}
