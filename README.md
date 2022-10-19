# Bookbuilder
Basic Nasdaq TotalView ITCH 5.0 bookbuilder for a given instrument.

# Example
Produce a `10-layer` book for the Apple instrument (`aapl`) at sequence number `483985` from the network capture `nasdaq_capture.pcap`.

```sh
> cargo run -- --symbol aapl -f nasdaq_capture.pcap -d 10 -n 483985

Book (depth: 10) : AAPL    
----- Sell -----
[0] 900 @ 94.6400
[1] 3 @ 96.0000
[2] 1000 @ 96.1700
[3] 2 @ 97.1100
[4] 1100 @ 97.4500
[5] 31 @ 98.0000
[6] 2 @ 98.2500
[7] 20 @ 98.5000
[8] 2 @ 99.4100
[9] 100 @ 100.0000
----- Buy -----
[0] 130 @ 94.2600
[1] 250 @ 94.1200
[2] 300 @ 92.1800
[3] 2 @ 91.9700
[4] 2 @ 90.9500
[5] 2 @ 89.9400
[6] 100 @ 89.0100
[7] 2 @ 88.9500
[8] 2 @ 87.9700
[9] 4 @ 86.3800

```

# Improvements
- Could use a more modular parser such as `nom` crate
- Add unit and integration tests
