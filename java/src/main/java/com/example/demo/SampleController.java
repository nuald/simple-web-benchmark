package hello;

import org.springframework.boot.*;
import org.springframework.boot.autoconfigure.*;
import org.springframework.stereotype.*;
import org.springframework.web.bind.annotation.*;
import java.io.*;

@Controller
@EnableAutoConfiguration
public class SampleController {

    @RequestMapping("/")
    @ResponseBody
    String home() {
        return "Hello World!";
    }

    @RequestMapping("/greeting/{name}")
    @ResponseBody
    String greeting(@PathVariable("name") String name) {
        return "Hello, " + name;
    }

    public static void main(String[] args) throws Exception {
        final var pid = String.valueOf(ProcessHandle.current().pid());
        try (final var writer = new BufferedWriter(new FileWriter(".pid"))) {
            writer.write(pid);
        }
        System.out.println("Master " + pid + " is running");
        SpringApplication.run(SampleController.class, args);
    }
}
