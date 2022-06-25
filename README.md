# Reddit discussion on an Implementation of Minimax with alpha-beta-pruning

See original discussion here: https://www.reddit.com/r/rust/comments/vkb41h/extremely_slow_recursive_minimax/  

Note: When running the code you'll notice that each implementation returns a different result even though the prng is seeded the same:  
The main reason for this is that because of the pruning the values from the prng are moved into different parts of the search_tree changing the result. Because of this issue we unfortunately cannot compare the algoritms to confirm correctness for that we would need a way to ensure the same search tree is used for each of the algorithms, which would entail actually building a representation of the tree and adapting the algorithms to use that representation.