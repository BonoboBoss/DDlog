JAR=ddlogapi.jar
# There are other *.class files generated, but
# the remaining ones are for inner classes of these
# so they are always generated together.
CLASS = ddlogapi/DDlogAPI.class \
		ddlogapi/DDlogCommand.class \
		ddlogapi/DDlogRecord.class \
		ddlogapi/DDlogRecCommand.class \
		ddlogapi/DDlogException.class \
		ddlogapi/DDlogConfig.class

HEADERS = ddlogapi_DDlogAPI.h ddlogapi_DDlogAPI_DDlogCommandVector.h

all: ${JAR} ${HEADERS}

${JAR}: ${CLASS}
	jar cvf ${JAR} ddlogapi/*.class

%.class: %.java
	javac $<

${HEADERS}: ddlogapi/DDlogAPI.java
	javac -h . $<

clean:
	rm -f *.class ddlogapi/*.class ${HEADERS}
