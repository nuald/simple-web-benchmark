require "http/server"

reg = %r(^/greeting/([a-z]+)$)

numThread = System.cpu_count

uname = Process.run("uname", {"-or"}) do |proc|
  proc.output.gets_to_end
end

pid = Process.pid
File.open(".pid", "w") do |io|
  io.print pid
end

puts "Master #{pid} is running"

numThread.times do |i|
  Process.fork do
    puts "Worker #{Process.pid} started"

    server = HTTP::Server.new do |context|
      context.response.headers["Content-Type"] = "text/plain"
      context.response.status_code = 200

      path = context.request.path
      if path == "/"
        context.response.print "Hello world!"
      elsif match = path.match(reg)
        msg = match.not_nil![1]
        context.response.print "Hello, #{msg}"
      else
        context.response.respond_with_status(HTTP::Status::NOT_FOUND, "Not Found")
      end
    end

    server.listen(port = 3000, reuse_port = true)
  end
end

sleep
