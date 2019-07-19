import core.thread;
import vibe.d;
import std.regex;
import std.stdio;
import std.conv;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

void main()
{
    auto pid = to!string(getpid());
    auto file = File(".pid", "w");
    file.write(pid);
    file.close();
    writefln("Master %s is running", pid);
    setupWorkerThreads(logicalProcessorCount + 1);
    runWorkerTaskDist(&runServer);
    runApplication();
}

void runServer()
{
    auto settings = new HTTPServerSettings;
    settings.options |= HTTPServerOption.reusePort;
    settings.port = 3000;
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

