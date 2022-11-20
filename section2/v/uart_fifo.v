`timescale 1ns / 1ps

module uart_fifo #(
    parameter                     BW = 9,
    parameter                     TIMER_BITS = 32,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868,

    localparam 	[2:0]	IDLE  = 3'b000,
    localparam 	[2:0]	START = 3'b001,
		localparam 	[2:0]	TXING = 3'b010,
		localparam 	[2:0]	DONE  = 3'b011,
		localparam 	[2:0]	NEXT  = 3'b100
  ) (
    input wire            clk,
    input wire            i_reset,

    output wire           led0_b,
    output wire           led3_r,

    output wire [7:0]     fifo_data,
    output wire [3:0]     out_bit_rx,
    output wire [3:0]     out_bit_tx,
    output wire           out_start_tx,
    output wire           o_empty,
    output wire [2:0]     out_state,

    output wire [6:0]     out_wr_addr,
    output wire [6:0]     out_rd_addr,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  wire        o_full;
  wire        ready_tx;
  wire        start_tx;

  /* verilator lint_off UNUSEDSIGNAL */
  wire [BW:0] out_data;
  /* verilator lint_on UNUSEDSIGNAL */

  assign led0_b   = uart_rxd_out;
  assign ready_tx = out_bit_tx == 15;
  assign start_tx = state == START;
  assign out_state = state;

  reg       r_wr;
  reg       r_rd;
  reg [2:0] state;

  always @(posedge clk)
    if (state == NEXT)
      state <= IDLE;
    else if (state == START)
      state <= TXING;
    else if (state == TXING)
    begin
      if (!start_tx && ready_tx)
        state <= DONE;
    end
    else if (state == DONE)
      state <= NEXT;
    else if (state == IDLE)
      if (ready_tx && !o_empty && out_wr_addr == 3)
        state <= START;

  always @(posedge clk)
    if (r_wr)
      r_wr <= 0;
    else if (!r_wr && !o_full && out_start_tx)
      r_wr <= 1;

  always @(posedge clk)
    if (r_rd)
      r_rd <= 0;
    else if (state == DONE)
      r_rd <= 1;

  fifo #(8, 7) fifo_inst (
    clk,
    r_wr,
    r_rd,
    out_data[8:1],
    out_wr_addr,
    out_rd_addr,
    fifo_data,
    o_full,
    o_empty
  );

  tx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) tx_uart_inst (
    clk,
    i_reset,
    start_tx,
    {1'b1, fifo_data, 1'b0},
    out_bit_tx,
    uart_rxd_out
  );

  rx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) rx_uart_inst (
    clk,
    i_reset,
    out_start_tx,
    led3_r,
    out_data,
    out_bit_rx,
    uart_txd_in
  );
endmodule
