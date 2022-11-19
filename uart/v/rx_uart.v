`timescale 1ns / 1ps

module rx_uart #(
    parameter                     BW=9,
    parameter                     TIMER_BITS = 10,
    parameter [(TIMER_BITS-1):0]  CLOCKS_PER_BAUD = 868,
    parameter [(TIMER_BITS-1):0]  HALF_PER_BAUD = 434
  ) (
    input wire            clk,
    input wire            i_reset,

    output wire           out_start_tx,
    output wire [BW:0]    out_data,
    output wire [3:0]     out_bit_rx,

    input wire            uart_txd_in
  );

  reg q_uart, qq_uart, ck_uart;

  reg [(BW):0]            r_data_in;
  reg [(BW):0]            r_data_out;
  reg [3:0]               r_bit_rx;
  reg [(TIMER_BITS-1):0]  clk_counter;

  reg                     r_start_rx;
  reg                     r_start_tx;
  reg                     r_prev_in;

  assign out_start_tx   = r_start_tx;
  assign out_bit_rx     = r_bit_rx;
  assign out_data       = r_data_out;

  always @(posedge clk)
    r_prev_in <= ck_uart;

  always @(posedge clk)
    if (i_reset)
      r_bit_rx <= 15;
    else if (r_start_rx)
      r_bit_rx <= 0;
    else if (r_bit_rx < BW && clk_counter == 0)
      r_bit_rx <= r_bit_rx + 1;
    else if (r_bit_rx == BW && clk_counter == 0)
      r_bit_rx <= 15;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_data_in <= 10'b1111111111;
    else if (clk_counter == HALF_PER_BAUD)
      r_data_in[r_bit_rx] <= ck_uart;

  always @(posedge clk)
    if (r_start_tx)
      r_data_out <= r_data_in;

  always @(posedge clk)
    if (i_reset || r_start_rx)
      r_start_rx <= 0;
    else if (r_bit_rx == 15 && !ck_uart && r_prev_in)
      r_start_rx <= 1;

  always @(posedge clk)
    if (i_reset)
      r_start_tx <= 1;
    else if (r_start_tx)
      r_start_tx <= 0;
    else if (r_bit_rx == BW && clk_counter == 0)
      r_start_tx <= 1;

  always @(posedge clk)
  begin
    if (clk_counter == 0 || r_start_rx)
      clk_counter <= CLOCKS_PER_BAUD - 1;
    else
      clk_counter <= clk_counter - 1;
  end

  // 2-step sync for metastability
  initial q_uart  = 1'b0;
  initial qq_uart = 1'b0;
  initial ck_uart = 1'b0;
  always @(posedge clk)
  begin
    q_uart  <= uart_txd_in;
    qq_uart <= q_uart;
    ck_uart <= qq_uart;
  end
endmodule
