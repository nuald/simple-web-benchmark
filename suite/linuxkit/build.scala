#!/usr/bin/env scalas

/***
scalaVersion := "2.12.3"
scalacOptions ++= Seq("-deprecation", "-feature")
libraryDependencies += "com.github.scopt" %% "scopt" % "3.7.0"
libraryDependencies += "org.clapper" %% "scalasti" % "3.0.1"
*/

import java.io._
import org.clapper.scalasti.ST
import scala.util.{Success, Failure}
import sys.process._

case class Config(
  usb: Boolean = false,
  ssid: String = "",
  password: String = ""
)

val parser = new scopt.OptionParser[Config]("run.scala") {
  opt[String]("ssid").optional().valueName("<SSID name>").
    action( (x, c) => c.copy(ssid = x) ).
    text(s"SSID name")

  opt[String]("password").optional().valueName("<SSID password>").
    action( (x, c) => c.copy(password = x) ).
    text(s"SSID password")

  opt[Unit]("usb").action( (_, c) =>
    c.copy(usb = true) ).text("generate USB bootable image")
}

def entryPoint(args: Array[String]): Unit = {
  parser.parse(args, Config()) match {
    case Some(config) => {
      new File("build").mkdir()
      val src = scala.io.Source.fromFile("linuxkit.yml.in").mkString
      val template = ST(src).add("config", config)
      template.render() match {
        case Success(dst) => {
          new PrintWriter("build/linuxkit.yml") { write(dst); close }
          "docker build . -t swb".!
          "moby build -format iso-bios -dir build build/linuxkit.yml".!
        }
        case Failure(ex) => println(ex)
      }
    }
    case None =>
    // arguments are bad, error message will have been displayed
  }
}

entryPoint(args)
