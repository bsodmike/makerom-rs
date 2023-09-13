# makerom-rs

[Ben Eater style ROM manipilation](https://youtu.be/yl8vPW5hydQ?si=P-MB6ZbdNb3jsZi_&t=2590), instead of using Python.

Simply run `cargo r` and check your firmware file with

```
$ hexdump -C rom.bin
00000000  a9 ff 8d 02 60 a9 55 8d  00 60 a9 aa 8d 02 60 4c  |....`.U..`....`L|
00000010  05 80 ea ea ea ea ea ea  ea ea ea ea ea ea ea ea  |................|
00000020  ea ea ea ea ea ea ea ea  ea ea ea ea ea ea ea ea  |................|
*
00007ff0  ea ea ea ea ea ea ea ea  ea ea ea ea 00 80 ea ea  |................|
00008000
```

