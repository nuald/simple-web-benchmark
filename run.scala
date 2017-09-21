#!/usr/bin/env scalas

/***
scalaVersion := "2.12.3"
scalacOptions += "-deprecation"
libraryDependencies += "org.jfree" % "jfreechart" % "1.0.19"
*/

import java.io.File
import java.util.ArrayList
import org.jfree.chart._
import org.jfree.chart.axis._
import org.jfree.chart.labels._
import org.jfree.chart.plot._
import org.jfree.chart.renderer.category._
import org.jfree.data.statistics._
import scala.sys.process._
import scala.collection.JavaConverters._

val LANGS = List(
  "go",
  "rust",
  "scala",
  "nodejs",
  "d"
)

val usage = s"""Usage: ./run.scala <list of languages>

Run the tests for the specified languages (* means all).
The following languages are supported: ${ LANGS.mkString(", ") }."""

val GOPATH = sys.env("GOPATH")
val PID_REGEX = raw"""p(\d+)""".r
val CSV_REGEX = raw"""([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+),([\d\.]+)""".r

def print(msg: String): Unit = {
  println(msg)
}

def runHey(lang: String, isIndex: Boolean): List[Double] = {
  val url = "http://127.0.0.1:3000/" + (if (isIndex) "" else "greeting/hello")
  val suffix = if (isIndex) "index" else "regex"
  val cmd = s"$GOPATH/bin/hey -n 50000 -c 256 -t 10"
  val csvCmd = s"$cmd -o csv $url"
  // First run, for JIT
  csvCmd.!!
  // Second run, for UI
  val runCmd = s"$cmd $url"
  print(s"[$lang] $url")
  runCmd.!
  // Third run, for stats
  val values = csvCmd.lineStream_!.flatMap { (line) => line match {
      case CSV_REGEX(responseTime, dnsLookup, dns, requestWrite, responseDelay, responseRead) => {
        Some(responseTime.toDouble * 1000)
      }
      case _ => None
    }
  }
  values.toList
}

def calculateStats(values: List[Double]): BoxAndWhiskerItem = {
  // Lazy evaluation is too slow, need to materialize
  val item = BoxAndWhiskerCalculator.calculateBoxAndWhiskerStatistics(
    new ArrayList(values.asJava))
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

def run(langs: Array[String]): BoxAndWhiskerCategoryDataset = {
  val dataset = new DefaultBoxAndWhiskerCategoryDataset()

  for (lang <- langs) {

    val lsof = Seq("lsof", "-Fp", "-i", ":3000")
    lsof.lineStream_!.foreach { (line) => line match {
        case PID_REGEX(pid) => {
          Seq("kill", "-9", pid).!
        }
        case _ =>
      }
    }

    val cmd = lang match {
      case "go" => "go run main.go"
      case "rust" => "cargo run --release"
      case "scala" => "sbt run"
      case "nodejs" => "node main.js"
      case "d" => "dub run --compiler=ldc2 --build=release"
    }
    val proc = Process(cmd, new File(lang)).run
    Thread.sleep(10000)

    val indexValues = runHey(lang, true)
    val langTitle = lang.capitalize
    dataset.add(calculateStats(indexValues), "Index URL Request", langTitle)
    val patternValues = runHey(lang, false)
    dataset.add(calculateStats(patternValues), "Pattern URL Request", langTitle)

    proc.destroy
    Thread.sleep(5000)
  }

  dataset
}

def writeStats(dataset: BoxAndWhiskerCategoryDataset): Unit = {
  val xAxis = new CategoryAxis("Language")
  val yAxis = new NumberAxis("Response, ms")
  yAxis.setAutoRangeIncludesZero(false)
  val renderer = new BoxAndWhiskerRenderer()
  renderer.setFillBox(false)
  renderer.setMeanVisible(false)
  renderer.setWhiskerWidth(0.5)
  val plot = new CategoryPlot(dataset, xAxis, yAxis, renderer)

  val chart = new JFreeChart(plot)
  ChartUtilities.saveChartAsPNG(new File("result.png"), chart, 400, 300);
}

def entryPoint(args: Array[String]): Unit = {
  if (args.length > 0) {
    var list = args.map(_ match {
      case "*" => LANGS
      case x: String => List(x)
    }).flatten.filter(LANGS.contains)
    print("Run tests for: " + list.mkString(" "))
    val ds = run(list)
    writeStats(ds)
  } else {
    print(usage)
  }
}

entryPoint(args)
