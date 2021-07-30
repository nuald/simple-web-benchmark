require 'bundler/inline'
require 'etc'
require 'optparse'

gemfile(true) do
  source 'https://rubygems.org'

  gem 'rails', '~> 6.1.4'
  gem 'puma', '~> 5.3.2'
end

require 'action_controller/railtie'

class App < Rails::Application
  routes.append do
    get '/' => 'greeting#index'
    get '/greeting/:name' => 'greeting#name'
  end

  config.action_controller.perform_caching = true
  config.api_only = true
  config.cache_classes = true
  config.consider_all_requests_local = true
  config.eager_load = true
  config.log_level = :warn
  config.logger = Logger.new('/dev/null')
  config.secret_key_base = '59484ad3-3d7e-4854-9137-5c7ea01a48cd'
end

class GreetingController < ActionController::API
  def index
    render plain: 'Hello, world!'
  end

  def name
    render plain: "Hello, #{params['name']}"
  end
end

options = { port: 3000 }
OptionParser.new do |opts|
  opts.on('-pPORT', '--port=PORT', Integer, 'server port') do |port|
    options[:port] = port
  end
end.parse!

pid = Process.pid
File.write('.pid', pid)
puts "Master #{pid} is running on port #{options[:port]}"

ENV['WEB_CONCURRENCY'] = Etc.nprocessors.to_s
App.initialize!
Rack::Server.new(app: App, Port: options[:port]).start
