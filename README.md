<div align="center">

# My Personal Website

**You can view my website [here](https://torborve.no).**

[![CI](https://github.com/TorBorve/Personal-website/actions/workflows/ci.yml/badge.svg)](https://github.com/TorBorve/Personal-website/actions/workflows/ci.yml) 
[![Latest Release](https://github.com/TorBorve/Personal-website/actions/workflows/release.yml/badge.svg)](https://github.com/TorBorve/Personal-website/actions/workflows/release.yml)

</div>

## About

This repository contains the source code for my website. The website is written in Rust using the frontend framework [Yew](https://yew.rs). [Trunk](https://trunkrs.dev/) is used to compile the the code to [WebAssembly](https://webassembly.org/). Github actions is used to test and deploy the code. Pushing to main automatically triggers a build which will be deployed to my server hosting the website.

## 🔧 Development

- **Dependencies**
  - [Rust](https://www.rust-lang.org/)
  - [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
  - `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`)

- **Build**

    ```bash
    trunk serve
    ```

## Hosting

Nginx docker container is used to host the website. The docker compose file and `ngnix.conf` needs to be copied to the server hosting the website. Then start the docker container using 

```terminal 
docker compose up -d
```

Then we need to copy the files over to the server. First compile to wasm using:

```terminal
trunk build --release
```

Copy contents of dist/ directory into `torborve_web` in folder used to start the docker container.


### SSL Certificates

I use Let's Encrypt for my SSL certificates:

```terminal
sudo certbot certonly --standalone -d torborve.no -d xn--brs-ula.no
```

This will create certificates. They need to be updated every 90 days. I am not sure if this is done automatically. 

The docker container needs permission to read the certificates. This can be done by running:

```terminal
sudo chown -R www-data:www-data /etc/letsencrypt/live/ /etc/letsencrypt/archive/
sudo chmod -R 755 /etc/letsencrypt/
```

## Resources

- use plotters in yew: [LINK](https://steven-anker.nl/blog/?p=454)
- github actions with rust: [LINK](https://github.com/simbleau/website)