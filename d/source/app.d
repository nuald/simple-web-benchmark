import core.thread;
import std.conv;
import std.getopt;
import std.regex;
import std.stdio;
import vibe.d;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

void main(string[] args)
{
    ushort port = 3000;
    getopt(args, "port",  &port);

    auto pid = to!string(getpid());
    auto file = File(".pid", "w");
    file.write(pid);
    file.close();
    writefln("Master %s is running on port %d", pid, port);
    setupWorkerThreads(logicalProcessorCount + 1);
    runWorkerTaskDist(&runServer, port);
    runApplication();
}

void runServer(ushort port) {
    auto settings = new HTTPServerSettings;
    settings.options |= HTTPServerOption.reusePort;
    settings.port = port;
    settings.bindAddresses = ["127.0.0.1"];
    listenHTTP(settings, &handleRequest);
}

void handleRequest(HTTPServerRequest req,
                   HTTPServerResponse res)
{
    if (req.path == "/")
        res.writeBody("Hello, World!", "text/plain");
    else if (auto m = matchFirst(req.path, reg))
        res.writeBody("Hello, " ~ m[1], "text/plain");
}
