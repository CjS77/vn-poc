# Simulating randomised epoch shuffling for Cerberus validator nodes

_rough cut: This report is not of the quality and rigour that would typically be expected of a Tari research note, 
but is presented in this rough form in order to share results as quickly as possible._

## Question:
For a given growth rate of validator nodes in the Tari DAN, assuming we want to avoid more than 33% of new nodes 
entering a VN committee at every epoch shuffle,
* How often should a single node rotate (i.e. how many epochs should a single node be in a committee before rotating out)?
* How large should hte committee size be?

## Approach

1. Define a Shard address as some 256-bit integer.
2. All validator nodes have an immutable identifier, the `vn_key` that lets us track a node from epoch to epoch.
3. All validator nodes have a `vn_address`  defined as `H(vn_key | (vn_key + i)/n)` where `H` is a 256-byte Blake2b 
   hash function. `i` is the current epoch and `n` is the number of epochs each node will wait before changing their 
   address. This formula guarantees that each node changes its address exactly every _n_ nodes.
4. A committee for a given shard is defined as the `committee_size` shards with address greater than or equal to the 
   shard address, wrapping to the beginning of the vn list if necessary.
5. It's assumed that all nodes are honest, i.e. the only reason that nodes won't vote on nistructions is because 
   they're new to a committee and are syncing state.

In the simulations, the shard addresses is kept constant. This helps identify exactly how a given committee changes 
over time.

Although only `100/n`% nodes change addresses every epoch, the number of new nodes in a given committee could be 
larger or smaller than this value depending on:
* whether a node displaces another node in a committee due to the ordering semantics, resulting in 2 changes in a 
  committee for 1 address change.
* a node _might_ shuffle back into the same committee, resulting in no effective change to a committee, even though 
  one of the node's address has changed.

If more than 33% of nodes in a committee after an epoch change are new, this potentially means that the committee 
must pause processing instructions until sufficient nodes have synced state to avoid byzantine conditions. 

The primary focus of these simulations is to map the prevalence of these syncing committees as we vary the committee 
size, shuffle rate, and number of nodes in the network.

## Conclusions

Even though the number of nodes that are moved every epoch is perfectly regular, the that that new addresses are 
uniformly and randomly distributed across the shard space means that it is inevitable that some committees will 
have to pause and sync at some point.

One can control the average frequency of committee pauses. Notably, increasing the committee size and increasing the 
epoch rotation period both reduce the probability of a given committee needing to pause after an epoch change.

A committee size in excess of 100 and a rotation period of 20 epochs showed no pauses over a simulation of 20,000 
instructions.

The results below provide the simulation results. 20 epochs are simulated in each case, with the committees for 1,000 
shards tracked across the simulation. Sample statistics, including the mean number of new nodes in each committee, 
the standard deviation, variance and extrema are provided. A histogram, plotting the distribution of the number of 
new nodes in each committee, with the byzantine limit of 33% is also included for visual reference.

## Results

### Case 1

A medium-to-large committee size (61 nodes), with nodes rotating every 8 epochs (12.5% of the population every epoch).

We simulate the network growing from 50 nodes through to 100,000 nodes. A committee can continue to process 
instructions with up t 20 new nodes joining at each epoch.

#### 50 nodes
In the trivial case, all nodes are in a single committee, and there are never new nodes in a committee at epoch 
changes.

```text
Each node covers 122.00% space. Nodes rotate every 8 epochs.
# Number of samples = 20000
# Min = 0
# Max = 0
#
# Mean = 0
# Standard deviation = 0
# Variance = 0
#
# Each ∎ is a count of 400
#
0 .. 1 [ 20000 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
1 .. 2 [     0 ]:
2 .. 3 [     0 ]:
3 .. 4 [     0 ]:
4 .. 5 [     0 ]: 
```

#### 100 nodes

100 Nodes population. 61 committee size. 2 committees. Each node covers 61.00% space. Nodes rotate every 8 epochs.
Expect approx 7 nodes to join each committee each epoch.

```text
# Number of samples = 20000
# Min = 0
# Max = 9
#
# Mean = 3.784300000000009
# Standard deviation = 1.3405497044123358
# Variance = 1.7970735100000006
#
# Each ∎ is a count of 109
#
0 ..  1 [   32 ]:
1 ..  2 [  643 ]: ∎∎∎∎∎
2 ..  3 [ 2609 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
3 ..  4 [ 5461 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
4 ..  5 [ 5236 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
5 ..  6 [ 4232 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
6 ..  7 [ 1265 ]: ∎∎∎∎∎∎∎∎∎∎∎
7 ..  8 [  442 ]: ∎∎∎∎
8 ..  9 [   66 ]:
9 .. 10 [   14 ]: 
```

#### 1,000 nodes

As the number of nodes increase, and the shard space each node covers shrinks, the lieklhood of a committee having 
byzantine conditions increases. With 1,000 nodes, there were 25 cases of 16 nodes being new to a committee over in 
the simulation sample of 20,000 committee changes. That's 80% of the threshold.

1000 Nodes population. 61 committee size. 17 committees. Each node covers 6.10% space.
Nodes rotate every 8 epochs.
Expect approx 7 nodes to join each committee each epoch.

```text
# Number of samples = 20000
# Min = 2
# Max = 16
#
# Mean = 8.449750000000005
# Standard deviation = 2.0603822309222135
# Variance = 4.245174937499998
#
# Each ∎ is a count of 80
#
 2 ..  3 [   12 ]: 
 3 ..  4 [   66 ]: 
 4 ..  5 [  312 ]: ∎∎∎
 5 ..  6 [  911 ]: ∎∎∎∎∎∎∎∎∎∎∎
 6 ..  7 [ 2057 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 7 ..  8 [ 3224 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 8 ..  9 [ 4025 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 9 .. 10 [ 3666 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [ 2583 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
11 .. 12 [ 1650 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
12 .. 13 [  854 ]: ∎∎∎∎∎∎∎∎∎∎
13 .. 14 [  404 ]: ∎∎∎∎∎
14 .. 15 [  179 ]: ∎∎
15 .. 16 [   32 ]: 
16 .. 17 [   25 ]: 
17 .. 18 [    0 ]: 
18 .. 19 [    0 ]: 
19 .. 20 [    0 ]: 
------------------------------ Byzantine threshold ------------------------------
20 .. 21 [    0 ]: 
21 .. 22 [    0 ]: 
```

#### 10,000 nodes

With 10,000 Nodes, and each node covering 0.61% of the shard space, we don't encounter any byzantine transitions, 
but 3 cases have the maximum number of new nodes.

```text
# Number of samples = 20000
# Min = 2
# Max = 19
#
# Mean = 8.906799999999993
# Standard deviation = 2.278094326405297
# Variance = 5.189713760000003
#
# Each ∎ is a count of 69
#
2 ..  3 [    8 ]:
3 ..  4 [   43 ]:
4 ..  5 [  274 ]: ∎∎∎
5 ..  6 [  826 ]: ∎∎∎∎∎∎∎∎∎∎∎
6 ..  7 [ 1658 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
7 ..  8 [ 2667 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
8 ..  9 [ 3492 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
9 .. 10 [ 3486 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [ 2949 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
11 .. 12 [ 2075 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
12 .. 13 [ 1238 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
13 .. 14 [  702 ]: ∎∎∎∎∎∎∎∎∎∎
14 .. 15 [  325 ]: ∎∎∎∎
15 .. 16 [  161 ]: ∎∎
16 .. 17 [   69 ]: ∎
17 .. 18 [   17 ]:
18 .. 19 [    7 ]:
19 .. 20 [    3 ]:
------------------------------ Byzantine threshold ------------------------------
20 .. 21 [    0 ]:
21 .. 22 [    0 ]:
``` 

#### 100,000 nodes

With a node population of 100,000 nodes, with each node covering 0.06% of the space, we start to encounter byzantine 
committees.

```text
# Number of samples = 20000
# Min = 2
# Max = 20
#
# Mean = 8.97050000000004
# Standard deviation = 2.3475156548998726
# Variance = 5.510829749999977
#
# Each ∎ is a count of 69
#
2 ..  3 [   12 ]:
3 ..  4 [   64 ]:
4 ..  5 [  249 ]: ∎∎∎
5 ..  6 [  797 ]: ∎∎∎∎∎∎∎∎∎∎∎
6 ..  7 [ 1731 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
7 ..  8 [ 2624 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
8 ..  9 [ 3287 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
9 .. 10 [ 3465 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [ 2932 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
11 .. 12 [ 2080 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
12 .. 13 [ 1328 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
13 .. 14 [  738 ]: ∎∎∎∎∎∎∎∎∎∎
14 .. 15 [  361 ]: ∎∎∎∎∎
15 .. 16 [  198 ]: ∎∎
16 .. 17 [   81 ]: ∎
17 .. 18 [   33 ]:
18 .. 19 [   15 ]:
19 .. 20 [    3 ]:
------------------------------ Byzantine threshold ------------------------------
20 .. 21 [    2 ]:
21 .. 22 [    0 ]: 
```

### Larger committee sizes

Two simulations were run with committee sizes of 100. The first, using a short epoch rotation time of 4 epochs, 
showed occurrences of syncing nodes relatively early on, while a long epoch rotation period of 20 epochs showed no 
syncing committees up to node populations of 100,000.

#### Nodes rotate every 4 epochs

```text
100 Nodes population. 100 committee size. 1 committees. Each node covers 100.00% space.
Nodes rotate every 4 epochs.
Expect approx 25 nodes to join each committee each epoch

# Number of samples = 20000
# Min = 0
# Max = 0
#
# Mean = 0
# Standard deviation = 0
# Variance = 0
#
# Each ∎ is a count of 400
#
 0 ..  1 [ 20000 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 1 ..  2 [     0 ]: 
 2 ..  3 [     0 ]: 
 3 ..  4 [     0 ]: 
 4 ..  5 [     0 ]: 
 5 ..  6 [     0 ]: 
 6 ..  7 [     0 ]: 
 7 ..  8 [     0 ]: 
 8 ..  9 [     0 ]: 
 9 .. 10 [     0 ]: 

------------------------------------------------------------------------------------------------------------------------
1000 Nodes population. 100 committee size. 10 committees. Each node covers 10.00% space.
Nodes rotate every 4 epochs.
Expect approx 25 nodes to join each committee each epoch

# Number of samples = 20000
# Min = 15
# Max = 38
#
# Mean = 24.388250000000067
# Standard deviation = 3.551916656891025
# Variance = 12.616111937499914
#
# Each ∎ is a count of 44
#
15 .. 16 [   14 ]: 
16 .. 17 [  118 ]: ∎∎
17 .. 18 [  266 ]: ∎∎∎∎∎∎
18 .. 19 [  447 ]: ∎∎∎∎∎∎∎∎∎∎
19 .. 20 [  786 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
20 .. 21 [ 1206 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
21 .. 22 [ 1408 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
22 .. 23 [ 1824 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
23 .. 24 [ 2239 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
24 .. 25 [ 2211 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
25 .. 26 [ 2168 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
26 .. 27 [ 1801 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
27 .. 28 [ 1705 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
28 .. 29 [ 1276 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
29 .. 30 [  997 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
30 .. 31 [  637 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎
31 .. 32 [  302 ]: ∎∎∎∎∎∎
32 .. 33 [  264 ]: ∎∎∎∎∎∎
------------------------------ Byzantine threshold ------------------------------
33 .. 34 [  178 ]: ∎∎∎∎
34 .. 35 [  153 ]: ∎∎∎

------------------------------------------------------------------------------------------------------------------------
10000 Nodes population. 100 committee size. 100 committees. Each node covers 1.00% space.
Nodes rotate every 4 epochs.
Expect approx 25 nodes to join each committee each epoch

# Number of samples = 20000
# Min = 13
# Max = 43
#
# Mean = 26.83265000000008
# Standard deviation = 3.829209314923904
# Variance = 14.662843977499994
#
# Each ∎ is a count of 81
#
13 .. 15 [    2 ]: 
15 .. 17 [   14 ]: 
17 .. 19 [  176 ]: ∎∎
19 .. 21 [  659 ]: ∎∎∎∎∎∎∎∎
21 .. 23 [ 1743 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
23 .. 25 [ 3011 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
25 .. 27 [ 3905 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
27 .. 29 [ 4052 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
29 .. 31 [ 3074 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
31 .. 33 [ 1889 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
------------------------------ Byzantine threshold ------------------------------
33 .. 35 [  958 ]: ∎∎∎∎∎∎∎∎∎∎∎
35 .. 37 [  335 ]: ∎∎∎∎
37 .. 39 [  137 ]: ∎
39 .. 41 [   37 ]: 
41 .. 43 [    7 ]: 
43 .. 45 [    1 ]: 

------------------------------------------------------------------------------------------------------------------------
100000 Nodes population. 100 committee size. 1000 committees. Each node covers 0.10% space.
Nodes rotate every 4 epochs.
Expect approx 25 nodes to join each committee each epoch
# Number of samples = 20000
# Min = 13
# Max = 44
#
# Mean = 27.083650000000016
# Standard deviation = 3.8293149096803205
# Variance = 14.6636526775
#
# Each ∎ is a count of 81
#
13 .. 15 [    1 ]: 
15 .. 17 [   23 ]: 
17 .. 19 [  144 ]: ∎
19 .. 21 [  592 ]: ∎∎∎∎∎∎∎
21 .. 23 [ 1524 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
23 .. 25 [ 2856 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
25 .. 27 [ 3802 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
27 .. 29 [ 4067 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
29 .. 31 [ 3285 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
31 .. 33 [ 2082 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
------------------------------ Byzantine threshold ------------------------------
33 .. 35 [ 1039 ]: ∎∎∎∎∎∎∎∎∎∎∎∎
35 .. 37 [  406 ]: ∎∎∎∎∎
37 .. 39 [  129 ]: ∎
39 .. 41 [   40 ]: 
41 .. 43 [    8 ]: 
43 .. 45 [    2 ]: 
```

#### Nodes rotate every 20 epochs
```text
------------------------------------------------------------------------------------------------------------------------
100 Nodes population. 100 committee size. 1 committees. Each node covers 100.00% space.
Nodes rotate every 20 epochs.

# Number of samples = 20000
# Min = 0
# Max = 0
#
# Mean = 0
# Standard deviation = 0
# Variance = 0
#
# Each ∎ is a count of 400
#
 0 ..  1 [ 20000 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 1 ..  2 [     0 ]: 
 2 ..  3 [     0 ]: 
 3 ..  4 [     0 ]: 
 4 ..  5 [     0 ]: 

------------------------------------------------------------------------------------------------------------------------
1000 Nodes population. 100 committee size. 10 committees. Each node covers 10.00% space.
Nodes rotate every 20 epochs.

# Number of samples = 20000
# Min = 1
# Max = 13
#
# Mean = 5.76855000000004
# Standard deviation = 1.939324856103276
# Variance = 3.7609808974999925
#
# Each ∎ is a count of 83
#
 1 ..  2 [   77 ]: 
 2 ..  3 [  550 ]: ∎∎∎∎∎∎
 3 ..  4 [ 1778 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 4 ..  5 [ 2781 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 5 ..  6 [ 4198 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 6 ..  7 [ 3981 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 7 ..  8 [ 2940 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 8 ..  9 [ 2012 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 9 .. 10 [  969 ]: ∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [  485 ]: ∎∎∎∎∎
11 .. 12 [  142 ]: ∎
12 .. 13 [   80 ]: 
13 .. 14 [    7 ]: 
14 .. 15 [    0 ]: 

------------------------------------------------------------------------------------------------------------------------
10000 Nodes population. 100 committee size. 100 committees. Each node covers 1.00% space.
Nodes rotate every 20 epochs.

# Number of samples = 20000
# Min = 1
# Max = 14
#
# Mean = 6.094249999999963
# Standard deviation = 1.9129994609251706
# Variance = 3.659566937499993
#
# Each ∎ is a count of 85
#
 1 ..  2 [   39 ]: 
 2 ..  3 [  262 ]: ∎∎∎
 3 ..  4 [ 1081 ]: ∎∎∎∎∎∎∎∎∎∎∎∎
 4 ..  5 [ 2634 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 5 ..  6 [ 4091 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 6 ..  7 [ 4253 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 7 ..  8 [ 3202 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 8 ..  9 [ 2296 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 9 .. 10 [ 1173 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [  611 ]: ∎∎∎∎∎∎∎
11 .. 12 [  241 ]: ∎∎
12 .. 13 [   67 ]: 
13 .. 14 [   34 ]: 
14 .. 15 [   16 ]: 
15 .. 16 [    0 ]: 

------------------------------------------------------------------------------------------------------------------------
100000 Nodes population. 100 committee size. 1000 committees. Each node covers 0.10% space.
Nodes rotate every 20 epochs.

# Number of samples = 20000
# Min = 0
# Max = 16
#
# Mean = 6.18165000000007
# Standard deviation = 1.9938538756639037
# Variance = 3.97545327749997
#
# Each ∎ is a count of 81
#
 0 ..  1 [    1 ]: 
 1 ..  2 [   33 ]: 
 2 ..  3 [  309 ]: ∎∎∎
 3 ..  4 [ 1178 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 4 ..  5 [ 2495 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 5 ..  6 [ 3717 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 6 ..  7 [ 4050 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 7 ..  8 [ 3464 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 8 ..  9 [ 2279 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
 9 .. 10 [ 1393 ]: ∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎∎
10 .. 11 [  615 ]: ∎∎∎∎∎∎∎
11 .. 12 [  267 ]: ∎∎∎
12 .. 13 [  140 ]: ∎
13 .. 14 [   36 ]: 
14 .. 15 [   16 ]: 
15 .. 16 [    5 ]: 
16 .. 17 [    2 ]: 
17 .. 18 [    0 ]: 
18 .. 19 [    0 ]: 
19 .. 20 [    0 ]: 

