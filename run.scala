import java.io.File
import scala.sys.process._

object RunApp {
  val usage = """
    Usage: scala run.scala <list of languages>

    Run the tests for the specified languages (* wildcard is supported for all).
"""

  val GOPATH = sys.env("GOPATH")
  val LANGS = List(
    "go",
    "rust",
    "scala",
    "nodejs",
    "d"
  )
  val PID_REGEX = raw"""p(\d+)""".r

  def runHey(lang: String, isIndex: Boolean): Unit = {
    val url = "http://127.0.0.1:3000/" + (if (isIndex) "" else "greeting/hello")
    val suffix = if (isIndex) "index" else "regex"
    val cmd = s"$GOPATH/bin/hey -n 50000 -c 256 -t 10"
    val csvCmd = s"$cmd -o csv $url"
    // First run, for JIT
    csvCmd.!!
    // Second run, for UI
    val runCmd = s"$cmd $url"
    println(s"[$lang] $url")
    runCmd.!
    // Third run, for stats
    csvCmd #> new File(s"stats/$lang-$suffix.csv")
  }

  def run(langs: Array[String]): Unit = {
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

      runHey(lang, true)
      runHey(lang, false)

      proc.destroy
      Thread.sleep(5000)
    }
  }

  def main(args: Array[String]): Unit = {
    if (args.length > 0) {
      var list = args.map(_ match {
        case "*" => LANGS
        case x: String => List(x)
      }).flatten.filter(LANGS.contains)
      println("Run tests for: " + list.mkString(" "))
      run(list)
    } else {
      println(usage)
    }
  }
}
