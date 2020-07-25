how to debug:
1. setup kernel debugging
2. run the following
```windbg
sxe ld driver.sys
g # load the driver here
bp driver.sys!DriverEntry
```

----
Refrences:
1. https://not-matthias.github.io/kernel-driver-with-rust/
2. https://github.com/Trantect/win_driver_example
3. https://github.com/Trantect/winapi-rs.git