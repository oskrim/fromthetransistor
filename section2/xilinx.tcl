set outputDir ./vivado_output
file mkdir $outputDir

read_verilog [ glob ./v/*.v ]
read_xdc [ glob ./xdc/*.xdc ]
synth_design -top top -part xa7a35tcsg324
opt_design
place_design
route_design
write_bitstream -force $outputDir/cpu.bit

open_hw_manager
connect_hw_server
open_hw_target
set_property PROGRAM.FILE $outputDir/cpu.bit [lindex [get_hw_devices] 0]
program_hw_devices

exit
