# List files on printer

```
=> "~M661" CR LF
<= 0x44 0xaa 0xaa 0x44
   <n: u32 BIG ENDIAN>
   [   
      0x3a 0x3a 0xa3 0xa3
      <size: u32 BIG ENDIAN>
      <byte chunk: `size` bytes>
   ] x n
```

# Read model snapshots

```
=>  "~M662 " <filename> CR LF
<=
    "CMD xxx received." CR LF
    "ok" CR LF
    0x2a 0x2a 0xa2 0xa2
    <size: u32, BIG ENDIAN>            -> consistent with value in BMP header (endian reversed)
    <BMP file: `size` bytes >                 /!\  SLIGHTLY OFF (crc or something??)
    
```