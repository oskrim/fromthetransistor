#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall main.cpp v/uart.v v/*_uart.v && ./obj_dir/Vuart
