require "http/server"

class Cluster
  def self.fork(env : Hash)
    env["FORKED"] = "1"
    return Process.fork {
      Process.run(PROGRAM_NAME, nil, env, true, false, true, true, true, nil)
    }
  end

  def self.isMaster
    (ENV["FORKED"] ||= "0") == "0"
  end

  def self.isSlave
    (ENV["FORKED"] ||= "0") == "1"
  end

  def self.getEnv(env : String)
    ENV[env] ||= ""
  end
end

reg = %r(^/greeting/([a-z]+)$)

numThread = System.cpu_count

if Cluster.isMaster
  puts "Using #{numThread} processes"
  numThread.times do |i|
    Cluster.fork({"i" => i.to_s})
  end

  server = HTTP::Server.new(3001) do |context|
    context.response.headers["Content-Type"] = "text/plain"
    context.response.status_code = 200

    path = context.request.path
    if path == "/kill"
      Process.exit
    else
      context.response.respond_with_error(message = "Not Found", code = 404)
    end
  end

  server.listen
else
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

  begin
    server.listen
  rescue
    server.listen(reuse_port = true)
  end
end
