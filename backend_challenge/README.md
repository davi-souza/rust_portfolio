# Davi Rust Backend Challenge

This was part of a hiring process. They asked me to build a simple backend using Rust.

## How to run

Required OS: Ubuntu 18.04

### Run with docker and docker-compose

First you'll need to build the image

```bash
docker-compose build project
```

To run the project just execute the following command

```bash
docker-compose up project
```

Or if you want to enter the container without running it

```bash
bash docker-enter.sh
```

### Run without docker/docker-compose

To install rust, you'll need `curl` and `build-essential` (which is probably already installed). You can use the commands

#### If you need to install Rust

```bash
sudo apt update

sudo apt install -y curl

# And if you need to install build-essential, use
# sudo apt install -y build-essential
```

To install rust, run

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile default
```

#### With Rust already installed

The project doesn't use crates that need external libraries, so with rust installed all we need to do is run the code. To do that, just run

```bash
cargo run
```

## How to use

## Endpoints

`GET localhost:5000/market_data` to get the Market Data view data.

To filter the data, you can provide the following query parameters:

- `providers`: an array of providers. Example: `&providers=cboe,ubs`
- `pair`: a pair. Example: `&pair=eur_gbp`
- `start_date` a date in ISO format. Example: `&start_date=2020-10-10T12:00:00`
- `end_date` a date in ISO format. Example: `&end_date=2020-10-10T12:00:00`

All the parameters above are optional

GET `localhost:5000/blotter` to get the Blotter view data

To filter the data, you can provide the following query parameters:

- `pairs`: an array of pairs. Example: `&pair=eur_gbp,eur_usd`
- `start_date` a date in ISO format. Example: `&start_date=2020-10-10T12:00:00`
- `end_date` a date in ISO format. Example: `&end_date=2020-10-10T12:00:00`
- `min_price` an integer. Example: `&min_price=10`
- `max_price` an integer. Example: `&max_price=10`
- `min_qty` an integer. Example: `&min_qty=10`
- `max_qty` an integer. Example: `&max_qty=10`

All the parameters above are optional

GET `localhost:5000/market_data_link` to get the Market Date Link view data

To filter the data, you can provide the following query parameters:

- `trade_id`: the id of the trade. Example: `&trade_id=1`
- `providers`: an array of providers. Example: `&providers=cboe,ubs`

The `trade_id` is mandatory and the `providers` is optional

### Header

There is a simulation of a token authorization.

In order to not receive 401 status code on every request, you must set the header

`Authorization: Bearer 123`
