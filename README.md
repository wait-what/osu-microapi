# osu microapi
Expose your osu! stats without auth for the clients as a JSON

Useful if you want to have an embed with your osu stats on your website

## Features
- Results are cached in memory and only updated on request after the interval expires
- Hilariously low RAM and usage and footprint
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
    "bind": "0.0.0.0:6969",
    "client_id": "123456",
    "secret": "ghfajksdlfghsdklfjghsdkljfbhsfdghjkldsfghjkl",
    "update_interval_minutes": 180,
    "user_id": "123456789"
}
```
> Don't make it update too often or you might get rate limited by the API. 180 is 3 hours

> `user_id` is the user you want to expose. It doesn't have to be the same account.

## License
This project is licensed under the [MIT license](./LICENSE)
