import vibe.d;
import std.regex;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

void main()
{
    runServer();
    runApplication();
}

void runServer()
{
    auto settings = new HTTPServerSettings;
    settings.port = 3000;
    settings.serverString = null;
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

