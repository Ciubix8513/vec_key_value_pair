# vec_key_value_pair

A drop in replacement for `std::HashMap` and `std::HashSet` that use `Vec` on the backend.
Meant for small maps, where you need the key-value pair structure, but don't want to pay for the expensive hashing.

Can also be used for types that don't implement `Hash`

In the worst case scenario the performance of this data structure is O(n)
