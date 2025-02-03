# Personal Webpage


## Ideas

- [ ] Mandelbrot set plotting
- [ ] Link to CV
- [X] Link to Linkedin and Github
- [X] Short information about me
- [X] Contact form
- [X] List of projects
- [X] Not found page
- [X] Github actions for auto deploy.


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

The docker container needs permision to read the certificates. This can be done by running:

```terminal
sudo chown -R www-data:www-data /etc/letsencrypt/live/ /etc/letsencrypt/archive/
sudo chmod -R 755 /etc/letsencrypt/
```

## Resources

- use plotters in yew: [LINK](https://steven-anker.nl/blog/?p=454)
- github actions with rust: [LINK](https://github.com/simbleau/website)