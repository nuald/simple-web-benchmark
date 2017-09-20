package lite

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import akka.http.scaladsl.model._
import akka.http.scaladsl.server.Directives._
import akka.stream.ActorMaterializer
import scala.io.StdIn

object WebServer {
  def main(args: Array[String]) {

    implicit val system = ActorSystem("my-system")
    implicit val materializer = ActorMaterializer()
    // needed for the future flatMap/onComplete in the end
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

    val bindingFuture = Http().bindAndHandle(route, "localhost", 3000)

    println(s"Server online at http://localhost:3000/\nPress RETURN to stop...")
    StdIn.readLine() // let it run until user presses return
    bindingFuture
      .flatMap(_.unbind()) // trigger unbinding from the port
      .onComplete(_ => system.terminate()) // and shutdown when done
  }
}

