#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall cpu.cpp v/cpu.v v/fifo.v v/*x*.v && ./obj_dir/Vcpu
