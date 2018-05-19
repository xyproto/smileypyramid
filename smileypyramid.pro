TEMPLATE = app

CONFIG += c++17
CONFIG -= console
CONFIG -= app_bundle
CONFIG -= qt

SOURCES += main.cpp

LIBS += -ldocopt

INCLUDEPATH += /usr/include/docopt

QMAKE_CXXFLAGS += -std=c++17
