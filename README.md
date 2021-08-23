# Application Development with Rust
## Abstract
The purpose of this article is to make it easier for the reader to learn Rust. This article will provide use-case, and example design and implementation using Rust. The hope is that the reader will be able to explore the example implementation and learn Rust. The reader can extend this and apply their learning to further solidify their understanding of Rust language.

## Introduction
I developed this code as part of my learning Rust. I had already developed the same in Python and C as part of my class assignment. I didn't have to spend time in developing logic, rather I focussed on developing the same using Rust. This expedited my learning process. In this article, I would want to sharer the use-cases, and the design that I based this code on. My hope was that this would help the reader not to worry about the use-case or design, rather the reader would explore the implementation using Rust, and this could expedite the Rust learning for the reader.

The use-case section of this article would link to the page with problem description. The design section would details the code structure. The run and extend section would detail the process to run this code, prerequisities, and options to extend this code. I hope that this would make it easier for the reader to learn Rust.
## Use-cases
The use-cases have been borrowed from NYU Cyberfellow class Introduction to Operating Systems. This class has a number of assignments to help the students to better understand various concepts in Operating System. This specific code has implementation for two of those assignments: 
* [Memory allocation lab](https://github.com/a759116/mos_rust/wiki/Use-Cases#memory-allocation-lab): This lab project addresses the various memory-management schemes in an operating system.
* [Virtual lab](https://github.com/a759116/mos_rust/wiki/Use-Cases#virtual-memory-lab): This lab project addresses the implementation of page-replacement algorithms in a demand-paging system.

## Design
The solution was developed as two different modules a) Memory for Memory Allocation, and b) Virtual for pagination algorithms in a system that utilized virtual memory. 

In Rust, the modules were to be defined either in main.rs or lib.rs. I had chosen to use lib.rs. You would notice that virtual was prefixed with r# in lib.rs. This was because virtual was a reserved keyword in Rust, and the prefix would help to escape the reserved keywords and use them as identifiers. For both modules defined in lib.rs, I had to create two .rs extension files: a) memory.rs, and b) virtual.rs. I again defined two modules in each of these files. For example, I had memory and test_memory in memory.rs. With this, I had a folder named memory that contained memory.rs, and test_memory.rs. The memory.rs had implementation code for Memory Allocation lab use-case. As the name suggests, test_memory had unit test code for the functionality implemented in memory.rs. Similar design was followed for implementing Virtual Lab use-case.

I used "serde" crate to enable serialization and desearilization of structured data. This was done to be able to expose the functionality as REST APIs. The details for API development would be covered in a separate [repo](https://github.com/a759116/mos_web). These external libraries or crates were to be defined as dependencies in Cargo.toml.

## Run and Extend
Before downloading and running this code, please follow [Rust Getting Started](https://www.rust-lang.org/learn/get-started). At this point, you would have Rust installed. Now you would clone or download this code. Then go to the code directory, and run the command "cargo test --lib". Upon successful run, you would see a message like "test result: ok. 37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s". 

If you would want to extend this code, you could implement functionality for [Disk Scheduling](https://github.com/a759116/mos_rust/wiki/Use-Cases#disk-scheduling-lab) and [CPU Scheduling](https://github.com/a759116/mos_rust/wiki/Use-Cases#cpu-scheduling-lab) labs following the design stated above and using the code here as examples.

## Conclusion
This was an attempt to provide an example design and implementation to help the reader to learn Rust. The reader was expected to explore the code on their own. This would be revised based on feedback from readers to achieve the goal of making it easier to learn Rust.
