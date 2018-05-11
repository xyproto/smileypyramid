TEMPLATE = app
#CONFIG += console
CONFIG += c++17
CONFIG -= app_bundle
CONFIG -= qt

INCLUDEPATH += /usr/include/docopt
LIBS += /usr/lib/libdocopt.so

QMAKE_CXXFLAGS += -std=c++17 -ldocopt

SOURCES += main.cpp
