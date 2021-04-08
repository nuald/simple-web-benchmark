
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <boost/asio.hpp>
#include <boost/format.hpp>
#include <chrono>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <list>
#include <memory>
#include <string>
#include <thread>
#include <unistd.h>
#include <fstream>

namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using format = boost::format;
using tcp = boost::asio::ip::tcp;

class http_worker {

public:
  http_worker(http_worker const&) = delete;
  http_worker& operator=(http_worker const&) = delete;

  http_worker(tcp::acceptor& acceptor): acceptor_(acceptor) {}

  void start() {
    accept();
    check_deadline();
  }

private:
  using request_body_t = http::string_body;

    // The acceptor used to listen for incoming connections.
  tcp::acceptor& acceptor_;

    // The socket for the currently connected client.
  tcp::socket socket_{acceptor_.get_executor()};

    // The buffer for performing reads
  beast::flat_static_buffer<8192> buffer_;

    // The parser for reading the requests
  boost::optional<http::request_parser<request_body_t>> parser_;

    // The timer putting a time limit on requests.
  net::steady_timer request_deadline_{
    acceptor_.get_executor(), (std::chrono::steady_clock::time_point::max)()
  };

  // The string-based response message.
  boost::optional<http::response<http::string_body>> string_response_;

  // The string-based response serializer.
  boost::optional<http::response_serializer<http::string_body>> string_serializer_;

  void accept() {
      // Clean up any previous connection.
    beast::error_code ec;
    socket_.close(ec);
    buffer_.consume(buffer_.size());

    acceptor_.async_accept(
      socket_,
      [this](beast::error_code ec) {
        if (ec) {
          accept();
        } else {
          // Request must be fully processed within 60 seconds.
          request_deadline_.expires_after(std::chrono::seconds(60));
          read_request();
        }
      });
  }

  void read_request() {
    // On each read the parser needs to be destroyed and
    // recreated. We store it in a boost::optional to
    // achieve that.
    //
    // Arguments passed to the parser constructor are
    // forwarded to the message object. A single argument
    // is forwarded to the body constructor.
    //
    // We construct the dynamic body with a 1MB limit
    // to prevent vulnerability to buffer attacks.
    //
    parser_.emplace(std::piecewise_construct, std::make_tuple());

    http::async_read(
      socket_,
      buffer_,
      *parser_,
      [this](beast::error_code ec, std::size_t) {
        if (ec) {
          accept();
        } else {
          process_request(parser_->get());
        }
      });
  }

  void process_request(http::request<request_body_t> const& req) {
    switch (req.method()) {
      case http::verb::get:
      send_response(http::status::ok, "Hello World!");
      break;

      default:
      // We return responses indicating an error if
      // we do not recognize the request method.
      send_response(
        http::status::bad_request,
        "Invalid request-method '" + std::string(req.method_string()) + "'\r\n");
      break;
    }
  }

  void send_response(http::status status, std::string const& body) {
    string_response_.emplace(std::piecewise_construct, std::make_tuple());

    string_response_->result(status);
    string_response_->keep_alive(false);
    string_response_->set(http::field::server, "Beast");
    string_response_->set(http::field::content_type, "text/plain");
    string_response_->body() = body;
    string_response_->prepare_payload();

    string_serializer_.emplace(*string_response_);

    http::async_write(
      socket_,
      *string_serializer_,
      [this](beast::error_code ec, std::size_t) {
        socket_.shutdown(tcp::socket::shutdown_send, ec);
        string_serializer_.reset();
        string_response_.reset();
        accept();
      });
  }

  void check_deadline() {
    // The deadline may have moved, so check it has really passed.
    if (request_deadline_.expiry() <= std::chrono::steady_clock::now()) {
      // Close socket to cancel any outstanding operation.
      beast::error_code ec;
      socket_.close();

      // Sleep indefinitely until we're given a new deadline.
      request_deadline_.expires_at(std::chrono::steady_clock::time_point::max());
    }

    request_deadline_.async_wait(
      [this](beast::error_code) {
        check_deadline();
      });
  }
};

int main() {
  try {
    auto const address = net::ip::make_address("0.0.0.0");
    unsigned short port = 3000;
    int num_workers = std::thread::hardware_concurrency();
    auto pid = getpid();
    {
      std::ofstream pid_file(".pid");
      pid_file << pid;
    }
    std::cout << format("Master %d is running on port %d\n") % pid % port;

    net::io_context ioc{1};
    tcp::acceptor acceptor{ioc, {address, port}};

    std::list<http_worker> workers;
    for (int i = 0; i < num_workers; ++i) {
      workers.emplace_back(acceptor);
      workers.back().start();
    }

    ioc.run();
  } catch (const std::exception& e) {
    std::cerr << "Error: " << e.what() << std::endl;
    return EXIT_FAILURE;
  }
}
