module app;

import vibe.core.core : runApplication, runWorkerTaskDist, setupWorkerThreads;
import vibe.http.server;
import vibe.http.router;

import std.getopt;
import std.process: thisProcessID;
import std.regex: ctRegex, matchAll;
import std.stdio: File, writeln;
import std.parallelism: totalCPUs;

auto ctr = ctRegex!("/greeting/([a-z]+)$");

ushort port = 3000;

void save_pid() {
    auto f = File(".pid", "w+");
    f.write(thisProcessID);
    writeln(i"Master $(thisProcessID) is running on port $(port)");
}

void main(string[] args)
{
    save_pid();
    auto info = getopt(args, "port", &port);

    setupWorkerThreads(totalCPUs);
    runWorkerTaskDist(() nothrow {
		try {
		    auto settings = new HTTPServerSettings;
                settings.port = port;
                settings.bindAddresses = ["0.0.0.0"];
                settings.options |= HTTPServerOption.reusePort;

                auto router = new URLRouter;

                router
                    .get("/greeting/:id", &handleRegex)
                    .get("/", &handleRoot);

                router.rebuild();
                listenHTTP(settings, router);
		} catch (Exception e) assert(false, e.msg);
	});
	runApplication();
}

void handleRegex(scope HTTPServerRequest req, scope HTTPServerResponse res)
{
    if ((req.requestURI).matchFirst(ctr)) {
        res.writeBody("Hello " ~ req.params["id"],"text/plain");
    }
    else {
        res.statusCode = HTTPStatus.notFound;
        res.writeBody("404 Not Found");
    }
}


void handleRoot(scope HTTPServerRequest req, scope HTTPServerResponse res)
{
    res.writeBody("Hello world!", "text/plain");
}
