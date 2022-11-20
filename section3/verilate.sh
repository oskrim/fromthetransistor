#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall cpu.cpp v/cpu.v v/*_uart.v v/fifo.v && ./obj_dir/Vcpu
