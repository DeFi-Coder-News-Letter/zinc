# The Zinc framework

## Design background

The goal of Zinc is to make writing safe zero-knowledge programs and ZKP-based
smart contracts easy. It has been designed with the following principles in mind:

- **Security**. It should be easy to write deterministic and secure programs.
Conversely, it should be hard to write code to exploit some possible
vulnerabilities found in other programming languages.
- **Safety**. The language must enforce the most strict semantics available,
such as a strong static explicit type system.
- **Efficiency**. The code should compile to the most efficient circuit possible.
- **Cost-exposition**. Performance costs that cannot be optimized efficiently
must be made explicit to the developers. An example is the requirement to
explicitly specify the loop range with constants.
- **Simplicity**. Anyone familiar with C-like languages (Javascript, Java,
Golang, C++, Rust, Solidity, Move) should be able to learn Zinc quickly and
with minimum effort.
- **Readability**. The code in Zinc should be easily readable to anybody
familiar with the C++ language family. There should be no counter-intuitive concepts.
- **Minimalism**. Less code is better. There should ideally be only one way to
do something efficiently. Complexity should be reduced.
- **Expressiveness**. The language should be powerful enough to make building
complex programs easy.
- **Turing incompleteness**. Unbounded looping and recursion are not permitted
in Zinc. This not only allows more efficient R1CS circuit construction but
also makes formal verifiability about the call and stack safety easier and
eliminates the gas computation problem inherent to Turing-complete smart
contract platforms, such as EVM.

## Key features

- Type safety
- Type inference
- Immutability
- Movable resources as a first-class citizen
- Module definition and import
- Expressive syntax
- Industrial-grade compiler optimizations
- Turing incompleteness: no recursion or unbounded looping
- Flat learning curve for Rust/JS/Solidity/C++ developers

## Comparison to Rust

Zinc is designed specifically for ZK-circuits and ZKP-based smart contract
development, so some differences from Rust are inevitable.

#### Type system

We need to adapt the type system to be efficiently representable in
finite fields, which are the basic building block of R1CS. The current type
system mostly follows Rust, but some aspects are borrowed from smart contract
languages. For example, Zinc provides integer types with 1-byte step sizes,
like those in Solidity.

#### Ownership and borrowing

Memory management is very different in R1CS
circuits compared to the von Neumann architecture. Also, since R1CS does not
imply parallel programming patterns, a lot of elements of the Rust design would
be unnecessary and redundant. Zinc has no ownership mechanism found in Rust
because all variables will be passed by value. The borrowing mechanism is still
being designed, but probably, only immutable references will be allowed shortly.

#### Loops and recursion

Zinc is a Turing-incomplete language, as it does not allow recursion and
variable loop indexes. Every loop range must be bounded with constant literals
or expressions.

## Installation

1. Download the latest release for your machine from https://github.com/matter-labs/zinc/releases.
2. Unpack its contents to some folder and add the folder to your `PATH` environment variable.
3. Use the binaries via your favorite terminal.

#### Quick setup

Download the Shell script for your OS and run it with `bash <name>.sh` to install
all the binaries and generate a local folder with examples ready for hacking.

[linux.sh](./install/linux.sh)

[macos.sh](./install/macos.sh)

## Documentation

The official Zinc book: https://zinc.matterlabs.dev

## Gitter

Please discuss here: https://gitter.im/matter-labs/zinc

## Example

At first, you should install the following binaries into your `PATH`:
- `zargo` - the circuit management tool
- `znc` - the Zinc compiler
- `zvm` - the Zinc virtual machine
- `schnorr` - the Schnorr signature tool (optional)

Then, follow the example to create and use your first circuit:

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build the circuit
zargo build

# fill the witness input JSON usually located at ./data/witness.json with values

# runs the circuit to check it without input data
zargo run

# generate the prover parameters
zargo setup

# generate the proof
zargo prove > './data/proof.txt'

# verify the proof
zargo verify < './data/proof.txt'
```

**OR**

```bash
# create a new circuit called 'zircuit'
zargo new zircuit
cd zircuit/

# write some code in the circuit

# build & run & setup & prove & verify
zargo proof-check

# fill the witness input JSON usually located at ./data/witness.json with values

# repeat the sequence
zargo proof-check
```
