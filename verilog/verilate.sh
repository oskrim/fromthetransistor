#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall uart.cpp v/uart.v v/*_uart.v && ./obj_dir/Vuart
