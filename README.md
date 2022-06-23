# gates-hdl
An "HDL" for the https://github.com/ncatelli/gates submission to LLJam 0001

## Dependencies
- rust 1.56+

if you want to run the example:
- docker 19.03.0+ 
- docker-compose latest

## Testing
Tests can be run via the standard cargo test sub-command.

```
cargo test
```

## Building
### Locally
```bash
cargo build --release
```

### Browser
```bash
wasm-pack build
cd www/
npm install
npm run start
```

### Running
A two gate example can be built with the following two commands

```bash
$ echo "DEFINE first AS not;
DEFINE second AS and;
LINK first -> a OF second;
LINK first -> b OF second;" > example.hdl

$ ./target/release/gates-hdl example.hdl -o docker-compose.yaml
```

The docker-compose environment can then be run with the following.
```bash
$ docker-compose up -d
```

The containers can be checked to validate they have gone healthy.

```bash
$ docker-compose ps
       Name                     Command                  State       Ports
--------------------------------------------------------------------------
gates-hdl_first_1    /opt/gates/bin/gates not - ...   Up (healthy)        
gates-hdl_second_1   /opt/gates/bin/gates and - ...   Up (healthy) 
```

A signal can be sent to the first gate with the following:

```bash
$ docker run --rm --network gates-hdl_default curlimages/curl:7.83.1 -X POST -sD - -d '{"state": false, "tick": 0}' http://first:8080/input/a 
HTTP/1.1 202 Accepted
Date: Thu, 23 Jun 2022 15:15:09 GMT
Content-Length: 0

```

And now the output from the second gate, which defaults to stdout output, can be seen in the logs.

```bash
$ docker-compose logs
Attaching to gates-hdl_second_1, gates-hdl_first_1
first_1   | 2022/06/23 15:10:45 Starting server on 0.0.0.0:8080
first_1   | 2022/06/23 15:10:45 Configured as not gate
second_1  | 2022/06/23 15:10:45 Starting server on 0.0.0.0:8080
second_1  | 2022/06/23 15:10:45 Configured as and gate
second_1  | 2022/06/23 15:15:09 tick: 0, state: true
```



## Grammar
The grammar can be found in the following representations:
- [EBNF](./docs/hdl.ebnf)
- [Browser-Friendly](./docs/hdl.xhtml) 