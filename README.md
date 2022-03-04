make a neural net in Rust

* Can we build a neural network from scratch to do MNIST (or maybe something else, like 2048)?

# ze rules of ze game
* no dependencies
* go from memory
* i guess just perform better than random?

# TODO
- [x] implement a randomized initializer (seeded)
- [ ] implement a cost function
- [ ] implement backprop
- [ ] build the infrastructure around passing MNIST data
  - [ ] read in the training data
    - [x] images
    - [ ] labels
    - [ ] pair these up
  - [ ] chunk the data into mini-batches to feed to the net

# How does neural net work (at least a multi-layer perceptron)?
* feed forward
  * successive matrix multiplications + non-linear function application
* compute cost
  * probably some parabola
* compute gradient
  * compute last layer gradient by zeroing cost
    * propagate backward one layer at a time

# matrix multiplication
|a b c||g h i j|   |ag+bk+co ah+bl+cp ai+bm+cq aj+bn+cr|
|d e f||k l m n| = |dg+ek+fo dh+el+fp di+em+fq dj+en+fr|
       |o p q r|
