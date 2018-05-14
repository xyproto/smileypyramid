TEMPLATE = app

CONFIG += c++17
CONFIG -= app_bundle
CONFIG -= qt

SOURCES += main.cpp

LIBS += -L/usr/lib -ldocopt
INCLUDEPATH += /usr/include/docopt

QMAKE_CXXFLAGS += -std=c++17
