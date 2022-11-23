`timescale 1ns / 1ps

module tx #(
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
    input wire            i_valid,
    input wire  [7:0]     i_data,

    output wire           uart_rxd_out
  );

  wire        ready_tx;
  wire        start_tx;
  wire [3:0]  bit_tx;
  wire        empty;
  wire        full;
  wire [7:0]  fifo_data;

  reg       r_wr;
  reg       r_rd;
  reg [2:0] state;

  assign ready_tx = bit_tx == 15;
  assign start_tx = state == START;

  fifo #(8, 3) txfifoi (
    clk,
    i_reset,
    r_wr,
    r_rd,
    i_data,
    fifo_data,
    full,
    empty
  );

  tx_uart #(BW, TIMER_BITS, CLOCKS_PER_BAUD) tx_uarti (
    clk,
    i_reset,
    start_tx,
    fifo_data,
    bit_tx,
    uart_rxd_out
  );

  always @(posedge clk)
    if (state == NEXT)
      state <= IDLE;
    else if (state == START)
      state <= TXING;
    else if (state == TXING)
      if (!start_tx && ready_tx)
        state <= DONE;
    else if (state == DONE)
      state <= NEXT;
    else if (state == IDLE)
      if (!empty && ready_tx)
        state <= START;

  always @(posedge clk)
    if (r_wr)
      r_wr <= 0;
    else if (!r_wr && !full && i_valid)
      r_wr <= 1;

  always @(posedge clk)
    if (r_rd)
      r_rd <= 0;
    else if (state == DONE)
      r_rd <= 1;
endmodule
