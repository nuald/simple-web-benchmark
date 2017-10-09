#!/usr/bin/env scalas

/***
scalaVersion := "2.12.3"
scalacOptions ++= Seq("-deprecation", "-feature")
libraryDependencies += "com.github.scopt" %% "scopt" % "3.7.0"
*/

import java.io._
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

val DhcpcdLocal = """
  - name: dhcpcd
    image: linuxkit/dhcpcd:d4408777ed6b6e6e562a5d4938fd09804324b33e
    command: ["/sbin/dhcpcd", "--nobackground", "-f", "/dhcpcd.conf", "-1"]
"""
val DhcpcdWlan0 = """
  - name: dhcpcd
    image: linuxkit/dhcpcd:d4408777ed6b6e6e562a5d4938fd09804324b33e
    command: ["/sbin/dhcpcd", "wlan0"]
"""

def entryPoint(args: Array[String]): Unit = {
  parser.parse(args, Config()) match {
    case Some(config) => {
      new File("build").mkdir()
      val root = if (config.usb) "root=/dev/sda1" else ""
      val dhcpcdLocal = if (config.usb) "" else DhcpcdLocal
      val dhcpcdWlan0 = if (config.usb) DhcpcdWlan0 else ""
      val src = scala.io.Source.fromFile("linuxkit.yml.in").mkString
      val dst = src.replace("%%ROOT%%", root).
        replace("%%SSID%%", config.ssid).
        replace("%%PASSWORD%%", config.password).
        replace("%%DHCPCD-LOCAL%%", dhcpcdLocal).
        replace("%%DHCPCD-WLAN0%%", dhcpcdWlan0)
      new PrintWriter("build/linuxkit.yml") { write(dst); close }
      "docker build . -t swb".!
      "moby build -format iso-bios -dir build build/linuxkit.yml".!
    }
    case None =>
    // arguments are bad, error message will have been displayed
  }
}

entryPoint(args)
