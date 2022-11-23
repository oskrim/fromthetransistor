#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall --top-module txfifotest txfifotest.cpp v/*.v && ./obj_dir/Vtxfifotest
verilator --cc --exe --build -j 0 -Wall --top-module cputest cputest.cpp v/*.v && ./obj_dir/Vcputest
