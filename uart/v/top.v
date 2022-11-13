`timescale 1ns / 1ps

module top #(
  parameter                     TIMER_BITS = 10,
	parameter [(TIMER_BITS-1):0]	CLOCKS_PER_BAUD = 868,
	parameter [(TIMER_BITS-1):0]	HALF_PER_BAUD = 434,
  parameter                     BW=9
  ) (
    input wire            clk,
    input wire		        i_reset,

    output wire           led0_b,
    output wire           led3_r,

    output wire [(BW):0]  out_data,
    output wire [3:0]     out_bit_rx,
    output wire [3:0]     out_bit_tx,
    output wire           out_start_tx,

    input wire            uart_txd_in,
    output wire           uart_rxd_out
  );

  reg [(BW):0]	          r_data;
  reg [3:0]               r_bit_rx;
  reg [3:0]               r_bit_tx;
  reg                     r_out;
  reg [(TIMER_BITS-1):0]  clk_counter;

  reg                     r_start_rx;
  reg                     r_start_tx;

  assign out_data     = r_data;
  assign out_bit_rx   = r_bit_rx;
  assign out_bit_tx   = r_bit_tx;
  assign out_start_tx = r_start_tx;

  assign uart_rxd_out = r_out;
  assign led0_b       = r_out;
  assign led3_r       = i_reset;

  always @(posedge clk)
    if (i_reset || r_start_tx)
      r_bit_rx <= 15;
    else if (r_start_rx)
      r_bit_rx <= 0;
    else if (r_bit_rx < BW && clk_counter == 0)
      r_bit_rx <= r_bit_rx + 1;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_bit_tx <= 15;
    else if (r_start_tx)
      r_bit_tx <= 0;
    else if (r_bit_tx < BW && clk_counter == 0)
      r_bit_tx <= r_bit_tx + 1;
    else if (r_bit_tx == BW && clk_counter == 0)
      r_bit_tx <= 15;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_data <= 10'b1111111111;
    else if (clk_counter == 0)
      r_data[r_bit_rx] <= uart_txd_in;

  always @(posedge clk)
    if (i_reset)
      r_out <= 0;
    else if (r_bit_tx != 15)
      r_out <= r_data[r_bit_tx];

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_start_rx <= 0;
    else if (r_bit_rx == 15 && !uart_txd_in)
      r_start_rx <= 1;

  always @(posedge clk)
    if (i_reset || r_start_tx)
      r_start_tx <= 0;
    else if (r_bit_rx == BW)
      r_start_tx <= 1;

  always @(posedge clk)
  begin
    if (r_start_rx || r_start_tx)
      clk_counter <= HALF_PER_BAUD;
    else if (clk_counter == 0)
      clk_counter <= CLOCKS_PER_BAUD - 1;
    else
      clk_counter <= clk_counter - 1;
  end
endmodule
