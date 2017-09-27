require "http/server"

reg = %r(^/greeting/([a-z]+)$)

server = HTTP::Server.new(3000) do |context|  
  context.response.headers["Content-Type"] = "text/plain"  
  context.response.status_code = 200    
    
  if context.request.path == "/" 
    context.response.print "Hello world!"
  else 
    context.response.print "Hello, #{context.request.path.match(reg).not_nil![1]}"
  end
end

server.listen(reuse_port: true)
