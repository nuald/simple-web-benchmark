#!/usr/bin/env scalas

/***
scalaVersion := "2.12.8"
scalacOptions ++= Seq("-deprecation", "-feature")
libraryDependencies += "org.jfree" % "jfreechart" % "1.5.0"
libraryDependencies += "com.github.jnr" % "jnr-posix" % "3.0.50"
libraryDependencies += "com.github.scopt" %% "scopt" % "3.7.1"
*/

import java.io.File
import java.text.SimpleDateFormat
import java.util.{ArrayList, Calendar}
import jnr.posix.POSIXFactory
import org.jfree.chart._
import org.jfree.chart.axis._
import org.jfree.chart.labels._
import org.jfree.chart.plot._
import org.jfree.chart.renderer.category._
import org.jfree.data.statistics._
import scala.sys.process._
import scala.collection.JavaConverters._

val IsWindows = sys.props("os.name").startsWith("Windows")
val Ext = if (IsWindows) ".exe" else ""
val ShellPrefix: Array[String] = if (IsWindows) Array("cmd", "/C") else Array()
val Posix = POSIXFactory.getPOSIX()

case class Cmd(
  cmd: Array[String],
  title: String,
  preRun: Option[Array[String]])

val LangCmds = Map(
  "go" -> Cmd(
    Array(s"go/build/main${Ext}"),
    "Go",
    Some(Array("go", "build", "-o", s"go/build/main${Ext}", "go/main.go"))),
  "rust_hyper" -> Cmd(
    Array(s"rust/hyper/target/release/simple-web-server${Ext}"),
    "Rust/hyper",
    Some(Array("cargo", "build", "--manifest-path rust/hyper/Cargo.toml", "--release"))),
  "rust_rocket" -> Cmd(
    Array(s"rust/rocket/target/release/rust-rocket${Ext}"),
    "Rust/rocket",
    Some(Array("cargo", "build", "--manifest-path rust/rocket/Cargo.toml", "--release"))),
  "scala" -> Cmd(
    ShellPrefix ++ Array("gradle", "-p", "scala", "run"),
    "Scala/Akka",
    Some(ShellPrefix ++ Array("gradle", "-p", "scala", "build"))),
  "java" -> Cmd(
    ShellPrefix ++ Array("java", "-jar", "-Dserver.port=3000",
      "java/build/libs/java-0.0.1-SNAPSHOT.jar"),
    "Java/Spring Boot",
    Some(ShellPrefix ++ Array("gradle", "-p", "java", "build"))),
  "nodejs" -> Cmd(
    Array("node", "nodejs/main.js"),
    "Node.js",
    None),
  "python" -> Cmd(
    Array("pypy3", "python/twist.py"),
    "PyPy3/Twisted",
    None),
  "ldc2" -> Cmd(
    Array(s"d/build/ldc/vibedtest${Ext}"),
    "D (LDC/vibe.d)",
    Some(Array("dub", "build", "--root=d", "--compiler=ldc2", "--build=release", "--config=ldc"))),
  "dmd" -> Cmd(
    Array(s"d/build/dmd/vibedtest${Ext}"),
    "D (DMD/vibe.d)",
    Some(Array("dub", "build", "--root=d", "--compiler=dmd", "--build=release", "--config=dmd")))
) ++ (if (IsWindows) Map() else Map(
  "crystal" -> Cmd(
    Array(s"./crystal/server${Ext}"),
    "Crystal",
    Some(Array("crystal", "build", "--release", "--no-debug", "-o", s"crystal/server${Ext}", "crystal/server.cr"))),
  "php" -> Cmd(
    Array("php", "-c", "php/swoole/php.ini", "php/swoole/main.php"),
    "PHP/Swoole",
    None)
))

val LsofPattern = raw"""p(\d+)""".r
val NetstatPattern = raw"""\s+\w+\s+[\d\.]+:3000\s+[\d\.]+:\d+\s+\w+\s+(\d+)""".r
val CsvPattern = raw"""([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+),(\d+),([\d\.]+)""".r
val DefaultImg = "result.png"
val Attempts = 30

def log(msg: String): Unit = {
  val now = Calendar.getInstance.getTime
  val fmt = new SimpleDateFormat("hh:mm:ss")
  println(s"[${ fmt.format(now) }] $msg")
}

def runHey(lang: String, isIndex: Boolean): List[Double] = {
  val url = "http://127.0.0.1:3000/" + (if (isIndex) "" else "greeting/hello")
  val suffix = if (isIndex) "index" else "regex"
  val cmd = "hey -n 50000 -c 256 -t 10"
  val csvCmd = s"$cmd -o csv $url"
  // First run, for JIT
  csvCmd ! ProcessLogger(_ => ())
  // Second run, for UI
  val runCmd = s"$cmd $url"
  log(s"[$lang] $url")
  runCmd.!
  // Third run, for stats
  val stream = csvCmd lineStream_! ProcessLogger(line => ())
  val values = stream.flatMap { (line) => line match {
      case CsvPattern(responseTime, dnsLookup, dns, requestWrite, responseDelay, responseRead, statusCode, offset) => {
        Some(responseTime.toDouble * 1000)
      }
      case _ => None
    }
  }
  values.toList
}

def calculateStats(lazyValues: List[Double]): BoxAndWhiskerItem = {
  // Lazy evaluation is too slow, need to materialize
  val values = new ArrayList(lazyValues.asJava)
  val item = BoxAndWhiskerCalculator.calculateBoxAndWhiskerStatistics(values)
  val mean = item.getMean()
  val median = item.getMedian()
  val q1 = item.getQ1()
  val q3 = item.getQ3()
  val minRegularValue = item.getMinRegularValue()
  val maxRegularValue = item.getMaxRegularValue()
  // ignore outliers
  new BoxAndWhiskerItem(mean, median, q1, q3, minRegularValue, maxRegularValue,
    null, null, null)
}

def kill(pid: Long): Unit = {
  if (IsWindows) {
    Seq("taskkill", "/t", "/f", "/pid", pid.toString).!
  } else {
    Posix.kill(-pid, 9)
    // process group kill doesn't always work
    Posix.kill(pid, 9)
  }
}

def killProcesses(): Boolean = {
  if (IsWindows) {
    val netstat = Seq("netstat", "-ona")
    netstat.lineStream_!.map { (line) => line match {
        case NetstatPattern(pid) if pid != "0" => {
          kill(pid.toLong)
          true
        }
        case _ => false
      }
    }.contains(true)
  } else {
    val lsof = Seq("lsof", "-Fp", "-i", ":3000")
    lsof.lineStream_!.map { (line) => line match {
        case LsofPattern(pid) => {
          kill(pid.toLong)
          true
        }
        case _ => false
      }
    }.contains(true)
  }
}

def getProcessId(procCmd: Array[String]): Option[Long] = {
  val pidFile = new java.io.File(".pid")
  pidFile.delete()
  Runtime.getRuntime.exec(procCmd)
  print("Waiting")
  for (i <- 1 to Attempts) {
    if (pidFile.exists) {
      val source = scala.io.Source.fromFile(pidFile)
      val content = try source.mkString finally source.close()
      return Some(content.toInt)
    }
    Thread.sleep(1000)
    print(".")
  }
  None
}

def run(langs: Seq[String], verbose: Boolean): BoxAndWhiskerCategoryDataset = {
  val dataset = new DefaultBoxAndWhiskerCategoryDataset()

  for (lang <- langs) {
    while (killProcesses()) {
      // kill until all died
    }
    val langCmd = LangCmds(lang)
    langCmd.preRun match {
      case Some(x) => {
        log(x.mkString(" "))
        Runtime.getRuntime.exec(x).waitFor
      }
      case None =>
    }
    val procCmd = langCmd.cmd
    log(procCmd.mkString(" "))
    getProcessId(procCmd) match {
      case Some(processId) => {
        val indexValues = runHey(lang, true)
        val langTitle = lang.capitalize
        dataset.add(
          calculateStats(indexValues), "Index URL Request", langCmd.title)
        val patternValues = runHey(lang, false)
        dataset.add(
          calculateStats(patternValues), "Pattern URL Request", langCmd.title)

        if (verbose) {
          log(s"Killing $processId process tree...")
        }
        kill(processId)
      }
      case None => log(s"$lang test failed!")
    }
  }

  dataset
}

def writeStats(dataset: BoxAndWhiskerCategoryDataset, out: File): Unit = {
  val langAxis = new CategoryAxis("Language")
  val responseAxis = new NumberAxis("Response, ms")
  responseAxis.setAutoRangeIncludesZero(true)
  val renderer = new BoxAndWhiskerRenderer()
  renderer.setFillBox(false)
  renderer.setMeanVisible(false)
  renderer.setWhiskerWidth(0.5)
  val plot = new CategoryPlot(dataset, langAxis, responseAxis, renderer)
  plot.setOrientation(PlotOrientation.HORIZONTAL)

  val chart = new JFreeChart(plot)
  ChartUtils.saveChartAsPNG(out, chart, 480, 640)
}

case class Config(
  out: File = new File(DefaultImg),
  verbose: Boolean = false,
  langs: Seq[String] = Seq())

val parser = new scopt.OptionParser[Config]("scalas suite/run.scala") {
  opt[File]('o', "out").optional().valueName("<file>").
    action( (x, c) => c.copy(out = x) ).
    text(s"image file to generate ($DefaultImg by default)")

  opt[Unit]("verbose").action( (_, c) =>
    c.copy(verbose = true) ).text("verbose execution output")

  arg[String]("<lang>...").unbounded().required().action( (x, c) =>
    c.copy(langs = c.langs :+ x) ).text("languages to test ('all' for all)")

  note(s"""
The following languages are supported: ${ LangCmds.keys.mkString(", ") }.""")
}

def entryPoint(args: Array[String]): Unit = {
  parser.parse(args, Config()) match {
    case Some(config) => {
      var list = config.langs.map(_ match {
        case "all" => LangCmds.keys
        case x: String => List(x)
      }).flatten.filter(LangCmds.contains)
      log("Run tests for: " + list.mkString(" "))
      val ds = run(list, config.verbose)
      writeStats(ds, config.out)
    }
    case None =>
    // arguments are bad, error message will have been displayed
  }
}

entryPoint(args)
