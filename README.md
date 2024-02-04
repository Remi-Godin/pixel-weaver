# pixel-weaver
A CPU multithreaded pixel-by-pixel image renderer.

## Foreword
The crate currently only supports 24 bit color space and was mostly designed to be used in conjunction with the [`simple-canvas`](https://crates.io/crates/simple-canvas) crate and the [`rusty-ppm`](https://crates.io/crates/rusty-ppm) crate, to create `.ppm` format images.

## How to use
#### Dependencies
Required:
- [`cgmath`](https://crates.io/crates/cgmath) 
- [`simple-canvas`](https://crates.io/crates/simple-canvas) 

Optional:
- [`rusty-ppm`](https://crates.io/crates/rusty-ppm)

To use this crate, you will need to import the [`simple-canvas`](https://crates.io/crates/simple-canvas) crate, since `pixel-weaver` uses the `simple-canvas::Canvas` struct to create the image. The pixel data is wrapped inside [`cgmath`](https://crates.io/crates/cgmath) `Vector3<_>` struct, the `x`, `y` and `z` field acting as RGB color, so you will also need to import this one. You can also import the [`rusty-ppm`](https://crates.io/crates/rusty-ppm) crate to encode the canvas into a `.ppm` image, which also relies on [`simple-canvas`](https://crates.io/crates/simple-canvas), but this is optional.

The idea behind this crate is to use it a bit like you would code a shader for a GPU. You create a function that will act upon every pixel, using only the pixel coordinate or UV to determine the final pixel color.

## Example Benchmarks:
### Testing methodology
Tests were run 10 times each and the results were averaged.

### Machine specs:
```
OS:   Pop!_OS 22.04 LTS x86_64
Host: 20QNS00Q00 ThinkPad P53
CPU:  Intel i5-9400H (8) @ 4.300GHz
RAM:  8GB
GPU1: Intel CoffeLake-H GT2 [UHD Graphics 630]
GPU2: NVIDIA Quadro T1000 Mobile
```
---

### Benchmark render image specs 
```
Width: 1_000 pixels
Height: 1_000 pixels
Total: 1_000_000 pixels
```
#### 3D Spheres benchmark results:
![Sphere](https://github.com/Remi-Godin/pixel-weaver/assets/129818497/d501f894-a9a5-43a9-8a16-57e39b3187d6)
```
Single thread:	  380.341435ms
Multi thread(10):  85.168631ms
```
#### 2D UV Square benchmark results:
![UVSquare](https://github.com/Remi-Godin/pixel-weaver/assets/129818497/ee194d29-f422-4303-a7d8-7726dbd8ef19)
```
Single thread:	  2.473724ms
Multi thread(10): 1.922870ms
```

#### Conclusion
It seems multithreaded consistently runs better than single threaded, but this might vary by the content of the pixel function, or by the machine running the code. But for more computationally heavy functions, multithreaded will vastly improve performances.
