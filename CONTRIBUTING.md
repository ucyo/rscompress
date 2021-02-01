# Contributing
This document describes the steps needed to add a feature to this library.
The library follows Test Driven Development (TDD).

0. Write test
1. Make test pass
2. Add logging
3. Add proper error handling
4. Add documentation
5. Enhance test cases (macros?)
6. Write benchmarks
7. Optimize code

**0. Write test** <br>
The first step is to write a test for the feature to be added.
The test should be structured and build with the syntax you want the interface to have.
This will help guaranteeing that the user will have a nice interface.
Additionally, it will help develop a consistent interface.

**1. Make test pass** <br>
The foremost goal is to make the test pass.
This enables fast development and increase in feature sets.
During this phase it is encouraged to use simple `println!()` statements for variable states.

**2. Add logging** <br>
After the tests are passing, proper logging should be structured.
Each logging level (`debug`, `info`, `warning` and `error`) should be used.
A good indication for proper spots for logging are the `println!()` lines from the previous steps.

**3. Add proper error handling** <br>
A proper error handling will enable faster execution and error escalation.
Each non-trivial function should return either a `Result` or an `Option`.

**4. Add documentation** <br>
Code is read more often than written.
Therefore, public functions facing the user should be documented thoroughly.
Should the implementation use not the obvious solution for a problem, the reasons for the decision should be written down. The author should decide what the `obvious solution' is.
The documentation must be extend, if questions arise during the review process.

**5. Enhance test cases (macros?)** <br>
The number of test cases for each function or struct should be extended for each possible edge case. Macros should be used extensively to make this process easier.

**6. Write benchmarks** <br>
Until now the overarching goal of each phase was to achieve correct execution of the code.
This and the next step involve in optimizing runtime and memory consumption.
Use `criterion` to write benchmarking tests.

**7. Optimize code** <br>
Well, take the benchmarks as a reference and optimize the code.
Code readability is still important.
If the performance gain or footprint reduction is not big enough, consider not adding it.
The decision is up to the developer.
But please add a remark in the documentation on how a performance gain could be achieved
and how `little' the effect is if implemented.
