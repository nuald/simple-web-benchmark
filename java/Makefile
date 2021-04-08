sources=$(shell find src/ -type f -name '*.java')
main=hello.SampleController

all: target/library.jar

# contains jars of all dependencies (including transitive ones)
target/dependencies.list: Maven.list
	mkdir -p target
	coursier fetch $(shell sed '/^#/d' Maven.list) > $@

# concatenate dependencies to be parseable as a classpath argument
target/classpath.line: target/dependencies.list
	paste --serial --delimiters ':' $^ > $@

# compile scala source files (note that a stamp file is used, since
# many classfiles may be created per source file)
target/classfiles.stamp: $(sources) target/classpath.line
	mkdir -p target/classfiles
	javac \
	  -d target/classfiles \
	  -cp $(shell cat target/classpath.line) \
	$(sources)
	touch target/classfiles.stamp

# bundle classfiles into a jar
target/library.jar: target/classfiles.stamp
	jar cf $@ -C target/classfiles .  -C src/main/resources .

# run the application
run: target/library.jar
	java -cp $<:$(shell cat target/classpath.line) -Dserver.port=3000 $(main)

clean:
	rm -rf target

.PHONY: clean all run
