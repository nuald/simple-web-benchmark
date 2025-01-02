module app;

import serverino;
import std.getopt;
import std.process: thisProcessID;
import std.regex: ctRegex, matchFirst;
import std.stdio: File, writeln;

auto ctr= ctRegex!("/greeting/([a-z]+)");

mixin ServerinoMain;
ushort port;

@onDaemonStart save_pid() {
    auto f = File(".pid", "w+");
    f.writeln(thisProcessID);
    writeln(i"Master $(thisProcessID) is running on port $(port)");
}

@onServerInit ServerinoConfig configure(string[] args) {
    auto info = getopt(args, "port", &port);

    return ServerinoConfig
        .create()
        .enableKeepAlive()
        .addListener("0.0.0.0", port)
        .setWorkers(25);
}

@endpoint void hello(Request req, Output output) {
    if (req.uri == "/")
        output ~= "Hello world!";
    else {
        auto ch = req.uri.matchFirst(ctr);
        if (!ch.empty)
            output ~= ch[1];
        else {
            output.status = 404;
            return;
        }
    }
}
