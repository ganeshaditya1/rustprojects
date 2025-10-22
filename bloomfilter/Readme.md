![Bloomfilter](Bloom_filter.svg)  
(Image taken from Wikipedia article on Bloomfilter. 
Attribution: "By David Eppstein - self-made, originally for a talk at WADS 2007, Public Domain, https://commons.wikimedia.org/w/index.php?curid=2609777"
)
# Bloomfilter

A probabilistic datastructure. It either says if the key is not present in the datastructure with 100% certainity. Or it would say that the key may exists in the datastructure. 

In this project I am taking the Sha256 hash of the key. And using the first 8 characters as a u64 number. Then moding it with 1001 to reduce it to a number between 0 and 1000. Then I do the same with the next 8 characters. The 8 characters after that. 3 times in total to obtain 3 numbers. And I use those 3 numbers as indices into a 1000 element boolean array and flip them those particular elements to true.