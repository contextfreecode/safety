JAKT_INSTALL=$HOME/projects/jakt/install
$JAKT_INSTALL/bin/jakt -R $JAKT_INSTALL/include/runtime $1.jakt && \
time ./build/$1
# time valgrind ./build/$1
