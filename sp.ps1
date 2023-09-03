# 1. 打开powershell
# 2. 查看串口接口
[System.IO.Ports.SerialPort]::getprotnames()
# 3. 创建串口
# port number, baudrate, parity, databits, stopbits
$port = new-Object System.IO.Ports.SerialPort COM3, 9600, None, 8, one
# 4. 打开串口
$port.open()
# 5. 向串口写数据
$port.WriteLine("Hello World")
# 6. 从串口读数据
$port.ReadLine()
# 7. 关闭串口
$port.Close()
# 8. Keep reading from serial port
do {$port.ReadLine()} while($port.IsOpen)

