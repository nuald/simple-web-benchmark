package lite

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import akka.http.scaladsl.model._
import akka.http.scaladsl.server.Directives._
import akka.stream.ActorMaterializer

object WebServer {
  def main(args: Array[String]): Unit = {
    val pid = ProcessHandle.current().pid().toString
    new java.io.PrintWriter(".pid") { write(pid); close() }
    val port = sys.env.get("PORT").getOrElse("3000").toInt
    println(s"Master ${ pid } is running on port ${ port }")

    implicit val system = ActorSystem("my-system")
    implicit val executionContext = system.dispatcher

    val route =
      path("greeting" / Segment) { user =>
        get {
          complete(HttpEntity(ContentTypes.`text/html(UTF-8)`, "Hello, " + user))
        }
      } ~
      path("") {
        get {
          complete(HttpEntity(ContentTypes.`text/html(UTF-8)`, "Hello World!"))
        }
      }

    Http().bindAndHandle(route, "localhost", port)
  }
}
