## pixel-weaver
A CPU multithreaded pixel-by-pixel image renderer.

## Example Bencmarks:
Machine specs:
```
OS:   Pop!_OS 22.04 LTS x86_64
Host: 20QNS00Q00 ThinkPad P53
CPU:  Intel i5-9400H (8) @ 4.300GHz
RAM:  8GB
GPU1: Intel CoffeLake-H GT2 [UHD Graphics 630]
GPU2: NVIDIA Quadro T1000 Mobile
```
Simulated workload benchmark results:
```
Image size
Wdith:  500 pixels
Height: 500 pixels
Total pixels: 250000
Simulated workload time per pixel: 1ns
Total simulated workload: 250µs
------------------
Single thread:	 14.824894062s
Multi thread(10): 1.252904065s
```
UV Square benchmark results:
```
Image size
Wdith:  500 pixels
Height: 500 pixels
Total pixels: 250000 pixels
------------------
Single thread:	 972.981µs
Multi thread(10): 2.316842ms
```
For simple images, the single threaded version performs better, but we can see that for more complex images, multithreading can drastically reduce rendering time.
