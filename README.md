## pixel-weaver
A CPU multithreaded pixel-by-pixel image renderer.

## How to use
To use this crate, you will also need to import the [`simple-canvas`](https://crates.io/crates/simple-canvas) crate, since `pixel-weaver` uses the `Canvas` struct to create the image. You can also import the [`rusty-ppm`](https://crates.io/crates/rusty-ppm) crate to encode the canvas into a `.ppm` image, which also relies on [`simple-canvas`](https://crates.io/crates/simple-canvas).

The idea behind this crate is to use it a bit like you would code a shader for a GPU. You create a function that will act upon every pixel, using only the pixel coordinate or UV to determine the final pixel color.

## Example Benchmarks:
#### Testing methodology
Tests were run 10 times each and the results were averaged.

#### Machine specs:
```
OS:   Pop!_OS 22.04 LTS x86_64
Host: 20QNS00Q00 ThinkPad P53
CPU:  Intel i5-9400H (8) @ 4.300GHz
RAM:  8GB
GPU1: Intel CoffeLake-H GT2 [UHD Graphics 630]
GPU2: NVIDIA Quadro T1000 Mobile
```
#### Simulated workload benchmark results:
```
Image size
Wdith:  500 pixels
Height: 500 pixels
Total pixels: 250000
Simulated workload time per pixel: 1ns
------------------
Single thread:	 14.824894062s
Multi thread(10): 1.252904065s
```
#### UV Square benchmark results:
```
Image size
Wdith:  500 pixels
Height: 500 pixels
Total pixels: 250000 pixels
------------------
Single thread:	  2.473724ms
Multi thread(10): 1.922870ms
```
#### Conclusion
It seems multithreaded consistently runs better than single threaded, but this might vary by the content of the pixel function, or by the machine running the code. But for more computationally heavy functions, multuthreaded will vastly improve performances.
