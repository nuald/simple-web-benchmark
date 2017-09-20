import vibe.d;
import std.regex;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

shared static this()
{
    auto settings = new HTTPServerSettings;
    settings.port = 3000;
    settings.bindAddresses = ["0.0.0.0"];

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
