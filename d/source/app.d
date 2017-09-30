import vibe.d;
import std.regex;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

void main()
{
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

