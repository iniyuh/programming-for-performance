# Parallelized CNN implementation using CUDA

## Summary
Reimplemented the functionality already present in cpu.rs using CUDA. The latter is desirable as it parallelizes a lot of the workload of performing CNN computations.

## Technical details
### Merging of convolution and RELU layers
Like in CPU, the three stages of the CNN are applicaton of filters, zeroing of negative products, and application of weights. One difference in my implementation is that the relu layer is folded into the convolution layer (if the final dot product result in the convolution layer is negative we instead write 0 to the output data buffer).
<!-- ### Higher level logic (CUDA.rs)
In my rust code, I implement an initialization function which contains all the static variables necessary to perform the compute phase in the `CudaContext` struct provided. 
Within the compute function, there are 4 key things being done. 
1. I generate device boxes of the three buffers that the initialization phase doesn't provide, the initial input matrix, the output for my convolution/relu layer, and the output for my final output layer.
2. I launch my first kernel, which performs both the convolution and RELU steps.
3. I launch my second kernel, which performs the output step.
4. I parse the output data from the data buffer and return it. -->
### Convolution/RELU layer (kernel fucnction 1 of 2)
For this layer I create 400 blocks each containing 250 threads (technically 256, with 6 of them not being utilized), I know that 256 is a good starting point for the number of threads per block so I went with that number. 

I want 400 blocks of 250 threads as 400 \* 250 = 100,000. 100,000 being my target as we have 100,000 base level operations to complete. 

What is a base level operation from my perspective? One singular multiplication within a dot product operation which is then followed by a sum (adding the product of the multiplication to the dot product total).

We have a 100 by 100 matrix. This matrix is split into a 20 by 20 matrix each containing a 5 by 5 sub-matrix. These 5 by 5 sub-matrices are dot producted with a 5 by 5 filter. That is 400 (20\*20) dot products per filter. One dot product is composed of 25 (5\*5) sub-operations (multiplications followed by additions). So accounting for 10 filters, 400 dot products per filter, and 25 sub-operations per dot product we arrive at 100,000 (10\*400\*25) base level operations/work items.

Each block has 10\*8 bytes of shared memory, this is to accomadate for an array of 10 doubles. Each of my blocks focuses on one 5x5 sub-matrix from my input and calculates all of the dot product sums resulting between said sub-matrix and each 5x5 filter in my cnn. atomicAdd's to block shared memory is much more efficient than to global memory so the doubles in the array keep track of the running totals and when the final values are calculated, they are then copied to global memory.
### Output layer (kernel function 2 of 2)
For my output layer, I knew I wanted to apply a divide and conquer approach. This is because all the output layer is effectively doing is repeatedly taking two vectors that are 4000 in length and dot producting them. This would imply 4000 sub-operations (a multiplication followed by an addition) per pair of vectors. There is a pair of vectors per set of weights contained in the output layer (10 in total), implying 40,000 total operations/work items. 

Now say I was to use 256 threads per block, this would mean that in some blocks, a portion of the threads would be performing sub-operations pertaining to one set of weights, while the rest would be doing the same for another set of weights. This would make tracking where the sub-operations should contribute their values a slight hassle. Due to this, I aimed to have blocks with a number of threads that is a divisor of 40,000. The largest divisor of 40,000 that is (presumably) a multiple of an available warp size on our GPU is 32.

By using 32 threads per block, I know that all of the threads (which each perform 1 sub-operation) in the block correspond to the same set of weights. For this reason I used 32 threads per block and therefore 1,250 (40,000/32) blocks. Due to this convenient property of 32 threads per block, each block simply has 1 double of shared memory which is a running total of the sub-sum of the set of weights/dot product operation it corresponds to. We use this shared memory for the same reason as in kernel 1, atomicAdd's to block shared memory is favorable over global.

## Correctness
I tested for correctness by repeatedly applying the following steps:
1. Generating a new input image using generate.py
2. Running the pre-existing CPU implementation in order to get the correct values.
3. Running my CUDA implementation to get my values.
4. Running compare.py to ensure I am getting the same(ish) values.

## Performance
I performance tested by repeatedly running the two implementations, averaging their times of actual work done and comparing. On average (over 50 runs each), my CUDA implementation is about 79% slower than the CPU implementation. 

I believe this performance degradation can be attributed to the decisions I made regarding my number of blocks and threads. 
By removing the requirement for atomic adds by having each block focusing on its own dot product in my Convolution/RELU phase, I think I could have saved quite a bit of time. 
With regards to my divide and conquer strategy for my output layer, I think it was fine, but maybe 32 threads per block isn't favorable in terms of performance for CUDA, I lack the domain-specific knowledge to deliver a verdict on that front.

## Declaration of discussion:
Before starting on the code for this assignment, I discussed with Ryan Hoffman (rhoffman), regarding the Lab3 manual, Lab3 video, and more specifically the math behind behind how we could parallelize a CNN for better performance. We did not share any code, we only shared ideas, all of my code is my own (exempting the NVIDIA atomicAdd implementation for doubles). Any conversations we had after starting the code (debugging) occured verbally without any sharing of code.