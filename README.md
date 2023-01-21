# KrakenZLCDDriver

This project is to reverse engineer my NZXT Kraken Z AIO cooler. I want to be able to make it react to events in games. 

ex: changing jobs in FFXIV to display the relevant job symbol.

Most other RGB-related things have ample SDKs created to do this kind of thing programmatically, but not the screen for the AIO cooler :(

# Learnings

## USB Reverse Engineering

### Transfer Types
There are 4 primary types of USB transfers
1. INTERRUPT
2. BULK
3. ISOCHRONOUS
4. CONTROL

### Wireshark with UsbPCap
Useful for listening to the data going over the wire, however I didn't know how to filter for my specific device. I eventually learned there was a way to filter by device in the UsbPCap interface settings.

I also learned how to find Vendor ID and Device ID via the windows device manager.


### NZXT Kraken endpoints

0x01 -> INTERRUPT IN (read)

0x81 -> INTERRUPT OUT (write)

0x02 -> BULK OUT

## Rust Language

its hard when you don't have low level programming experience.