# vec_key_value_pair

A drop in replacement for `std::HashMap` that uses `Vec` on the backend.
Meant for small maps, where you need the keyvalue pair structure, but don't want to pay for the expensive hashing.

Can also be used for types that don't iplmement `Hash`

In the worst case scenario the performance of this datastructure is O(n)
