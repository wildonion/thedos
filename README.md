

<p align="center">
    <img src="https://github.com/wildonion/thedos/blob/master/onion.png"
</p>


## ☠ TheDos

Attack networks asynchronously by getting help of workers! 

<p align="center">
    <img src="https://github.com/wildonion/thedos/blob/master/log_nginx.png"
</p>

## 🚬 Usage

> run it over VPN

> default workers are 4096

> build using: ```cargo build --release```

* start http attack with 4096 workers: ```./thedos --http-addr http://example.app --workers 4096```

* start tcp attack with 4096 workers: ```./thedos --tcp-addr 93.184.216.34:24535 --workers 4096```

* start udp attack with 4096 workers: ```./thedos --udp-addr 93.184.216.34:24535 --workers 4096```

## 🏗️ WIP

* udp attack

* dns attack