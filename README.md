# landonbin
A client and server implementation of code/text snippet storage, similar to Pastebin. Features a CLI tool to easily upload the contents of files to your own webserver, so you can use your own domain. Supports syntax highlighting, paste expiry, and API keys. 


## Server
The server is written in JavaScript, and uses NodeJS and the Express framework. It uses MongoDB as a database, and the mongoose library to interact with the database. The API endpoint is `/pastes`, and accepts POST requests with the following JSON body:

```json
{
    "content": string,
    "expiry": string
}
```

The `content` field is the content of the snippet, and the `expiry` field is the expiry date of the snippet. Content should be base64 encoded. When retrieving a paste, it will be base64 decoded. The expiry date is formatted like "1d", "1h", "1m", etc. for days, hours, and minutes respectively. You can also use "Never" to set the expiry date to never.

Additionally, the header `X-API-Key` must be provided, otherwise the server will return a 401 Unauthorized error. Authorized API keys can be defined in `server/routes/api.js`. 

The server will return a JSON response with the following format:

```json
{
    "status": int,
    "id": string,
    "expiry": date,
    "url": string
}
```

If an error occurs, the response will be the following:

```json
{
    "status": int,
    "error": string
}
```

Pastes can be retrieved by sending a GET request to `/pastes/:id`, where `:id` is the ID of the paste. The server uses the highlight.js library to provide syntax highlighting for snippets. Additionally, you can specify the language syntax to use, by appending an extension to the GET request.

```
GET /pastes/:id.js => JavaScript
GET /pastes/:id.py => Python
GET /pastes/:id.cpp => C++
```

Raw paste data can be retrieved by sending a GET request to `/pastes/raw/:id`. This will return the raw paste data, without any syntax highlighting.

The MongoDB database uses an index on the `expiry` field to automatically delete expired pastes. This is done by setting the `expireAfterSeconds` option to 0. This will cause MongoDB to automatically delete documents when the `expiry` field is less than the current date. This is done by the mongod cronjob automatically.

### Building
To build the server, you must have NodeJS and npm installed. You can install NodeJS and npm by following the instructions [here](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). Once you have NodeJS and npm installed, you can build the server by running `npm install` in the `server` directory. Then, the server can be started by running `npm start`. The server will be running on port 3000 by default-- this can be changed by setting the `PORT` environment variable.

You will also need to have MongoDB installed. You can install MongoDB by following the instructions [here](https://docs.mongodb.com/manual/installation/). You must also configure MongoDB to run the deletion cron. This can be done by running the following command:
```
mongo
> use pastes
> db.pastes.createIndex({ "expiry": 1 }, { expireAfterSeconds: 0 })
```

It's recommended to [enable authentication](https://medium.com/mongoaudit/how-to-enable-authentication-on-mongodb-b9e8a924efac) on the MongoDB database.

## Client (CLI)
The client is a command-line interface (CLI) that allows you to upload snippets. It is written in Rust, and uses the reqwest crate to make HTTP requests to the server. The client can be configured by editing the `client/src/landonbin.rs` file. The default server URL is `http://api.example.com/pastes`, and the default API key is `my-secret-api-key`.

### Building
To build the client, you must have Rust installed. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). Once you have Rust installed, you can build the client by running `cargo build --release` in the `client` directory. The binary will be located at `client/target/release/landonbin`.

### Usage
The client has two ways to upload snippets. You can either use the `--file` argument or `--text`. Additionally, you can pass an expiration date at the end of the command. If no expiration date is passed, it defaults to no expiry. It's recommended to add the binary to your PATH so you can run it from anywhere.

```
./landonbin --file example.txt # Expires never
./landonbin --text "Hello, world!" 7d # Expires in 7 days
./landonbin --file /home/user/Desktop/flag.txt 1h # Expires in 1 hour
```
