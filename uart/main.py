import serial

baudrate = 9600
ser = serial.Serial('/dev/ttyUSB1', baudrate, timeout=1, parity=serial.PARITY_NONE, bytesize=serial.SEVENBITS, stopbits=serial.STOPBITS_ONE)

print(baudrate)
print(ser.read())

s = 'asdf'
n = ser.write(s.encode('utf8'))
print(s, n)
print(ser.read(n))
