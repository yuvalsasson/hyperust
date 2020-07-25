# Hyperust
Type 2 Hypervisor written entirely in rust
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
2. https://github.com/Trantect/winapi-rs.git
3. https://rayanfam.com/topics/hypervisor-from-scratch-part-1/
4. Intel manual (https://software.intel.com/en-us/articles/intel-sdm)
