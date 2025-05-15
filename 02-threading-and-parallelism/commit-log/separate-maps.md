# Added multithreading and separate hasmap option to the raw parser

## Summary
Updated `dictionary_builder` to utilize multithreading, allowing users to set the desired number of threads to distribute the workload accross (using the `--num-threads` flag). There are now two versions of the function, the default uses shared hashmaps (via the DashMap crate) between all threads, the other option which is relevant to this PR uses a hashmap per thread (by specifying via the `--single-map` flag) leading to a decrease in performance compared to the default.

## Tech Details
The new flags have been implemented in the pre-existing style of the codebase, using match blocks to parse the argument values. The `--single-map` boolean value alters the control flow to utilize the `dictionary_builder_single_map` function. The `num_threads` value is then passed through the chain of calls until it arrives at the aforementioned function as an argument.

Threading was implemented by parsing the workload (the file) into a vector of strings (each one being a line of the file). This vector of strings would then be distributed amongst the threads in an equitable manner. One caveat going from the initial implementation to the threaded implementation was the loss of continuity. The initial implementation assumed the lines would be processed as one batch from start to finish, reflected in the `prev1` and `prev2` values being used. In order to simulate continuity, I've implemented a helper function `get_last_two_tokens_of_line` which will set the `prev1` and `prev2` values in such a way so that threads that don't start at the beginnning of the set of lines can function normally.

## Testing
I used manual regression testing for this PR, ensuring that all initially present functionality and outputs were preserved accross the changes made. I know that the initial results (that I regression tested against) are correct by analyzing the logs "by hand" and comparing my results to those of the function (with the function outputting the same answers, thus validating them).

## Performance
Using `hyperfine` to test the initial implementation, separate hashmap multithreaded implementation, and shared hashmap multithreaded implementation, I observed in a lack of performance disparity between the first two, whilst the latter clearly performed better than the others. These trends remained true when using warmups and cold cache preparations in hyperfine (which I tried out due to the fact that we pull data from disk in this assignment).
