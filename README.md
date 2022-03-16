# client-info

A very simple cli tool to show basic info of the client's web request

## Usage

Run `client_info`, it will automatically start a web server on `0.0.0.0:8080`. 

Open the url `http://0.0.0.0:8080/` in your web browser. It will show a simple web page with basic request information, including your IP, UserAgent and Remote Address.

You can also use curl to send a POST request to it. It will give you request information in JSON format.

If you want to specify the address, use the `--addr` option, e.g.

```bash
$ client-info --addr 127.0.0.1:8080
```
