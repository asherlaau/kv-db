# kv-db

This is my first Database related project. It takes reference of bitcask database. 
- reference link 
https://riak.com/assets/bitcask-intro.pdf

This project achieve two things: 

1. To get familiar with Rust and Go, I implemented two versions.

2. To understand how key-value database work, and how database uses the disk efficiently.

# Implementation 

There mainly two parts of the paper, how to store and organize the data in memory and disk. I follow the exactly same way of the paper, and choose to use B+ Tree as keydir's data structure [reason : rust's BTreeMap is great and fast, I want to focus on the design, instead of the minor things, however I will try to implement a B+ tree in rust later].  

<h2> Disk design </h2>
Only support the system file IO, but i provided the IO manager interface for flexibility.
