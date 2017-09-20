package lite

import akka.actor.ActorSystem
import akka.http.scaladsl.Http
import akka.http.scaladsl.model._
import akka.http.scaladsl.model.Uri.Path._
import akka.http.scaladsl.server.{Route, RouteResult}
import akka.stream.ActorMaterializer

import scala.concurrent.Future
import scala.io.StdIn

object WebServer {
  def main(args: Array[String]) {

    implicit val system = ActorSystem("my-system")
    implicit val materializer = ActorMaterializer()
    // needed for the future flatMap/onComplete in the end
    import system.dispatcher

    def response(s: String) = Future.successful{RouteResult.Complete{HttpResponse(
      status = StatusCodes.OK,
      entity = HttpEntity(ContentTypes.`text/html(UTF-8)`, s)
    )}}

    val route: Route = request => response(request.unmatchedPath match {
      case SingleSlash => "Hello World!"
      case Slash(Segment("greeting", Slash(Segment(user, Empty)))) => "Hello, " + user
    })

    val bindingFuture = Http().bindAndHandle(route, "localhost", 3000)

    println(s"Server online at http://localhost:3000/\nPress RETURN to stop...")
    StdIn.readLine() // let it run until user presses return
    bindingFuture
      .flatMap(_.unbind()) // trigger unbinding from the port
      .onComplete(_ => system.terminate()) // and shutdown when done
  }
}

