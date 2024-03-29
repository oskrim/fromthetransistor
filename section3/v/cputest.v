`timescale 1ns / 1ps

module cputest (
    input wire            clk,
    input wire            i_reset,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  wire rx_valid;
  wire wr_valid;
  wire cpu_running;
  wire [7:0] rx_data;
  // verilator lint_off UNUSEDSIGNAL
  wire [31:0] rd_data;
  // verilator lint_on UNUSEDSIGNAL
  // verilator lint_off UNDRIVEN
  wire [31:0] rd_addr;
  // verilator lint_on UNDRIVEN
  wire [31:0] wr_data;
  wire [31:0] wr_addr;
  wire [31:0] pc_data;
  wire [31:0] pc_addr;

  cpu cpui (
    clk,
    i_reset,
    cpu_running,
    rd_data,
    rd_addr,
    wr_data,
    wr_addr,
    pc_data,
    pc_addr,
    wr_valid
  );

  uart uarti (
    clk,
    i_reset,
    wr_valid,
    wr_data,
    wr_addr,
    uart_rxd_out
  );

  rx_uart rx_uarti (
    clk,
    i_reset,
    rx_valid,
    rx_data,
    uart_txd_in
  );

  ram rami (
    clk,
    i_reset,
    cpu_running,
    rx_valid,
    rx_data,
    rd_addr,
    wr_addr,
    pc_addr,
    rd_data,
    wr_data,
    pc_data,
    wr_valid
  );
endmodule
