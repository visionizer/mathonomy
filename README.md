# Mathonomy - Mathematics for Rust
Mathonomy is a physics & mathematics library for the Rust programming language.

## Naming

Most formulas are implement at least twice. Once, complex and once simple. Simple  formulas provide
some abstraction over the raw, complex alternatives. Often, there are multiple simple functions
providing different levels of abstraction. Let's take, as an example, the calculation of the lorentz
factor.

The following functions are available for said calculation:

- `slorentz`
- `slorentzt`
- `slorentztr`
- `clorentz`

Now, let's take a look at those functions

1. `slorentz` is short for `Simple Lorentz`. A 'S' always signifies that this function belongs to the simple
functions. `slorentz` just takes in the speed as `t` is often 1. This is an abstraction over `slorentzt`
2. `slorentzt` is the same as `slorentz`, but, you can pass in a `t` parameter. This is signified by the `t` at the end of the name. This simply abstracts `slorentztr`
3. `slorentztr` is the same as the function before, but, you can pass in the ratio of `v` to `c`. This is the final layer of abstraction over `clorentz`.
4. `clorentz` is the complex function, as signified by the `c` at the beginning of its name. Here, you have to pass in the coordinate time. 

## Units
All units used are SI-Units. We also use the official symbols only.
**All units are also stated in a comment above the function**
Additionally f64's can always be converted into other units.

### Kilo, Mega and more.
All units can be converted into their kilo, mega, ... counterparts by simply calling `.kilo()` or similar methods.

#### Example

```rs
let big_joule = mathonomy::relativity::skinetic_energy(mathonomy::consts::SPEED_OF_LIGHT - 1, 1);
let giga_joule = big_joule.giga()
```


## How to read the comments

Comments are *always* structured the same way. This is to help the coder know which function to use.

```md
# <Function Name>
<Short description of what the function does>

(Optional)
## Comparison to another similar formula
<When should you use which one? Example: Newtonian vs Relative kinetic energy formula>

## Variables
<A list of the variables that are passed into the function. Those functions **always** follow the same scheme>
- <name in function> (<symbol> :: <Unit>) = <Description>
[Examples (From relativistic kinetic energy)]
- speed (v :: m/s)
- mass (m :: kg) = The mass of the body

## Related Functions
<Functions that are related, such as, the complex alternative, other simple alternatives>

## Read more
<A link to a (usually) wikipedia page about the topic>

(Optional)
## Safety
<A note about the safety of the function>
```

#### Example
```md
# Simple Kinetic Energy
Calculates the relativistic kinetic energy

## Relativistic vs Newtonian
Use this formula if v is bigger than 1% of c
## Variables
- speed (v :: m/s)
- mass (m :: kg) = The mass of the body
## Related Functions
kinetic_energy => The same, but you have to calculate the lorentz factor yourself
## Read more
https://en.wikipedia.org/wiki/Kinetic_energy#Relativistic_kinetic_energy_of_rigid_bodies
```