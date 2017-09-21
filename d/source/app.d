import vibe.d;
import std.regex;

auto reg = ctRegex!"^/greeting/([a-z]+)$";

void main()
{
    runWorkerTaskDist({
        listenHTTP("0.0.0.0:3000", &handleRequest);
    });
    runApplication();
}

void handleRequest(HTTPServerRequest req,
                    HTTPServerResponse res)
{
    if (req.path == "/")
        res.writeBody("Hello, World!", "text/plain");
    else if (auto m = matchFirst(req.path, reg))
        res.writeBody("Hello, " ~ m[1], "text/plain");
}

