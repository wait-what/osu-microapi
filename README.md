# osu microapi
Expose osu! stats for select users publicly as a JSON

Useful if you want to have an embed with your osu stats on your website. An example page using the API can be found in `example.html`

## Features
- Results are cached in memory and only updated on request after the interval expires
- Runs on Windows, Linux and whatever else you can compile it for

## Usage
- Download a prebuilt binary from [releases](https://github.com/wait-what/osu-microapi/releases)
- Create a `config.json`
- Create an OAuth application [here](https://osu.ppy.sh/home/account/edit#oauth)
- Copy the `Client ID` and `Client Secret`
> Do not share this information!
- Create a `config.json` in the same directory following this example:
```json
{
    "bind_address": "0.0.0.0:6969",
    "client_id": "12345",
    "client_secret": "asdfhjksadlhfkldcnkljdsfhgjkldsfhgsjkdlfg",
    "update_interval_minutes": 180,
    "mode": "osu",
    "user_ids": [
        "10040223",
        "7562902"
    ]
}
```
> Don't make it update too often or you might get rate limited by the API. 180 is 3 hours

> `user_ids` is the list of users you want to expose.

## API reference
- GET `/` - comma separated list of user IDs
- GET `/user/<id>` - JSON object with user info

## Manual build
- Install Rust
- Clone the repo
- Run `cargo build --release`

## License
This project is licensed under the [MIT license](./LICENSE)
