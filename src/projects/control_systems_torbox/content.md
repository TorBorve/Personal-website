
As part of my master's thesis, I’ve been working on a control systems library in Rust. The goal was to make a substitue for MATLAB's Control Systems Toolbox in Rust. In my master's thesis I used the library to implement a numerical tuning method for linear controllers. The library can be found on [GitHub](https://github.com/TorBorve/Control-Systems-TorBox) and [crates.io](https://crates.io/crates/control_systems_torbox).

<br/>
<div align="center">
<img src="../static/auto_tuning_nyquist.png" width="40%">
<br>
<figcaption align="center">Fig. 1: Resulting Nyuist plot for controllers after numerical tuning</figcaption>
</div>
<br/>

## 🚀 What is Control Systems TorBox?
---

Control Systems TorBox is a Rust library for designing, analyzing, simulating, and implementing linear control systems. The name comes from MATLAB's Control Systems Toolbox and my name, Tor.

The library includes features like:

- Transfer function and state-space models
- Frequency response, poles, and zeros
- Bode and Nyquist plots
- Calculation of H2 and H-inf norms

It’s all open source, and you can find it on my GitHub.

## 🦀 Why Rust?
---

The numerical tuning algorithm from my thesis was originally implemented in MATLAB, but it was slow. The algorithm relies on computing H2 and H-inf norms many times and further more it contains a lot of loops to test other tunings. The computation of H2 and H-inf norms in MATLAB is likley fast, as they are implemented in C. However, the loops around these function calls may be very slow compared to a compiled language like Rust. By switching to rust I gained an approximate speed up of 50x.

I have also been trying to learn more rust lately, and this project definetly helped with that. However, what I must say I don't like about the rust eco-system is the lack of mature crates for scientific computing and even things like plotting. This was also a reason that I wanted to do it in rust, since I did not find any other crate which had the features I needed.

## 📦 Getting Started
---

To use Control Systems TorBox in your Rust project, just add it to your Cargo.toml:

```toml
control_systems_torbox = "*"
```

## ✨ Examples
---

**Transfer Functions**

```rust
use control_systems_torbox::Tf;

let model = 1.0 / Tf::s() * Tf::pade(1.0, 2);
let controller = 1.0 + 1.0 / Tf::s(); // PI controller
let open_loop = model * controller;
let sensitivity = Tf::new_from_scalar(1.0).feedback(&open_loop);
```

**State-Space Models**

```rust
use control_systems_torbox::Ss;
use nalgebra::dmatrix;

let a = dmatrix![0., 1.; 0., 0.];
let b = dmatrix![0.; 1.];
let c = dmatrix![1., 0.];
let d = dmatrix![0.];
let sys = Ss::<Continuous>::new(a, b, c, d).unwrap();

let sys_tf = sys.to_tf().unwrap(); // Convert to transfer function
```

**System Norms**

```rust
use control_systems_torbox::Tf;

let low_pass = 1.0 / (Tf::s().powi(2) + 0.2*Tf::s() + 1.0);
let low_pass = low_pass.to_ss().unwrap();
let h2 = low_pass.norm_h2().unwrap();
let hinf = low_pass.norm_hinf().unwrap();
println!("H2 norm: {}, H-inf norm: {}", h2, hinf);
```

**Poles and Zeros**

```rust
let sys = (Tf::s() - 1.0) * (Tf::s() + 3.0) / (Tf::s() * (Tf::s() + 2.0));
let sys = sys.to_ss().unwrap();
let poles = sys.poles();
let zeros = sys.zeros().unwrap();
```

**Plotting**

```rust
use control_systems_torbox::*;

let sys = (1.0/Tf::s()) * (1.0 + 1.0/Tf::s());
let sys_clp = sys.feedback(Tf::new_from_scalar(1.0));

let mut bode_plot = BodePlot::new(BodePlotOptions::default());
bode_plot.add_system(BodePlotData::new(sys_clp.bode(0.1, 10.)));
bode_plot.show(800, 600, "Bode plot demo").unwrap();

let mut nyq_plot = NyquistPlot::new(NyquistPlotOptions::default());
nyq_plot.add_system(NyquistPlotData::new(sys.nyquist(0.1, 100.)));
nyq_plot.show(600, 600, "Nyquist plot demo").unwrap();
```

For more examples see the [documentation](https://docs.rs/control_systems_torbox)

## 🛠️ Still a Work in Progress
---

There's still a lot more I want to implement, and I'm more than happy to receive feedback or contributions. If you're working in control systems and are into Rust, or just curious, feel free to check it out or open an issue on GitHub.
