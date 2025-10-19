module app;

import std.getopt;
import std.process: thisProcessID;
import std.regex: ctRegex, matchFirst;
import std.stdio: File, writeln;
import std.parallelism: totalCPUs;
import std.array: empty;

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

@endpoint 
@route!("/")
void hello(Output output) {
    output ~= "Hello world!";
}

@endpoint
@route!(req => !(req.path).matchFirst(ctr).empty)
void greetings(Request req, Output output) {
    output ~= "Hello, " ~ req.path[10..$]; 
}

@endpoint @priority(-1)
void page404(Output output)
{
	output.status = 404;
	output.addHeader("Content-Type", "text/plain");

	output.write("Page not found!");
}
