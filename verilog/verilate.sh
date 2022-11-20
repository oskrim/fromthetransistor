#!/usr/bin/env bash
verilator --cc --exe --build -j 0 -Wall uart.cpp v/uart.v v/*_uart.v && ./obj_dir/Vuart
verilator --cc --exe --build -j 0 -Wall uart_fifo.cpp v/uart_fifo.v v/*_uart.v v/fifo.v && ./obj_dir/Vuart_fifo
