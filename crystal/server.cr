require "http/server"
require "option_parser"

port = 3000
OptionParser.parse do |parser|
  parser.on("--port=PORT", "server port") { |p| port = p.to_i }
end

reg = %r(^/greeting/([a-z]+)$)

pid = Process.pid
File.open(".pid", "w") do |io|
  io.print pid
end

puts "Master #{pid} is running on port #{port}"

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

server.listen(port, true)
