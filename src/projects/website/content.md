<!-- # How I Built My Website with Rust and Yew -->

## Why I Built This Website

I wanted to make my own website such that I could share my projects and learn more about web development. Given that I am a robotics engineer it may not come as a surprise that I didn't want to use javascript. Since I'm currently learning **Rust**, and because it has strong support for **WebAssembly**, I decided to build the website using Rust and the **Yew** framework.

<div align="center">
    <img src="../static/website_wide.png" width="100%">
</div>

## What I Used
---

Here’s the tech stack I went with:

- **Rust & Yew** – The frontend is written in Rust using Yew, which lets me create web apps with a component-based structure.
- **Bulma.io** – A simple and lightweight CSS framework to make things look decent without too much effort.
- **Trunk** – A handy build tool that compiles Rust to WebAssembly and handles static assets.
- **Raspberry Pi** – My tiny but mighty server hosting the website.
- **GitHub Actions** – Automates the build and deployment process.

## How It Works
---

### Writing the Code

The site is built using Rust and Yew, which means everything is written in Rust instead of JavaScript. Yew’s component-based system makes it easy to break things into reusable parts.

### Styling with Bulma

I didn’t want to spend too much time on styling, so I used [Bulma.io](https://bulma.io/). It’s a simple CSS framework that makes things look clean out of the box.

### Hosting on a Raspberry Pi

Instead of paying for hosting, I run the website on a **Raspberry Pi**. It’s cheap, fun to tinker with, and gives me full control over the setup. I used [Nginx](https://nginx.org/), running in a [docker](https://www.docker.com/) container, to sever the website, and [Let's Encrypt](https://letsencrypt.org/) to get a TLS certificate for the website.

### Automated Deployment with GitHub Actions

I set up **GitHub Actions** to handle deployment automatically. Here’s how it works:

1. I push code to the `main` branch.
2. GitHub Actions checks out the repository.
3. It compiles the Rust code to WebAssembly using `Trunk`.
4. The built files get transferred to my Raspberry Pi.
5. The site updates automatically.


## Check Out the Code
---

If you’re curious, the full source code is on GitHub:
[GitHub Repository](https://github.com/your-repo-url)

## Wrapping Up
---

The website is by no means perfect, especially on mobile. I plan to keep improving the styling and add some features. However, more importantly I want to share my projects and experiences here. 
