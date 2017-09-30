require "http/server"

reg = %r(^/greeting/([a-z]+)$)

numThread = System.cpu_count
children = [] of Process

uname = Process.run("uname", {"-or"}) do |proc|
  proc.output.gets_to_end
end
isWSL = uname =~ /.*-Microsoft GNU\/Linux/

puts "Using #{numThread} processes"

numThread.times do |i|
  children << fork do

    server = HTTP::Server.new(3000) do |context|
      context.response.headers["Content-Type"] = "text/plain"
      context.response.status_code = 200

      path = context.request.path
      if path == "/"
        context.response.print "Hello world!"
      elsif match = path.match(reg)
        msg = match.not_nil![1]
        context.response.print "Hello, #{msg}"
      else
        context.response.respond_with_error(message = "Not Found", code = 404)
      end
    end

    if isWSL
      server.listen
    else
      server.listen(reuse_port = true)
    end

  end
end

server = HTTP::Server.new(3001) do |context|
  context.response.headers["Content-Type"] = "text/plain"
  context.response.status_code = 200

  path = context.request.path
  if path == "/kill"
    children.each do |p|
      puts p.pid
      if p.exists?
        p.kill
      end
    end
    Process.exit
  else
    context.response.respond_with_error(message = "Not Found", code = 404)
  end
end

server.listen
