# ToyFOC-Ctrl

基于 `esp32-c3` 实现的 `toyfoc` 控制端程序

接收 `MQTT` 指令后转换为 `I2C` 指令，发送到 `ToyFOC`

可以配合 `toyfoc-web` 使用

## 关联库

##### `ToyFOC-Ctrl` 的 web 控制端程序

https://github.com/kisy/toyfoc-web

#####  `toyfoc` 的 `rp2040` 实现

https://github.com/kisy/toyfoc-rp2040

##### rust `embedded-hal` 无刷直流电机驱动库

https://github.com/kisy/toyfoc
