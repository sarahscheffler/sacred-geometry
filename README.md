# Sacred Geometry

[Sacred Geometry](https://github.com/sarahscheffler/sacred-geometry.git) is a
feat that lets you add metamagic to your spells without increasing their level
(instead you raise the casting time a la spontaneous caster metamagic) if you
roll a certain way on Knowledge: Engineering.

The mechanic for doing this is the following: You roll a number of d6 equal to
your ranks in Knowledge (Engineering).  You then see if you can create an
arithmetic expression that yields certain prime numbers.  The specific target
primes are different depending on the modified level of the spell you are
trying to cast.

I can't actually imagine using this mechanic in actual gameplay, since it 
either requires you to pause the game for three minutes while you add up dice,
or write a program to do it for you.

So here's the program to do it for you.

## Download

If you just want to do sacred geometry calculations, download the latest
release from github.

If you want to build the arithmetic equations, download from source.  This code
is written in Rust (partially as an exercise in learning Rust, so please pardon
my syntax).

## Usage

```
./sacred_geometry ranks spell_level
```
or 
To use the solver, run `./sacred_geometry ranks spell_level`.  For example, for
a caster with 8 ranks in Knowledge (Engineering) casting an effective 9th-level
spell, run `./sacred_geometry 8 9`

The output will look something like this:
```
$ ./sacred_geometry 8 9
Die rolls: [6, 2, 4, 6, 4, 6, 1, 6]
101 = ((((6) * ((4) * (((6) * (6)) / (2)))) / (4)) - (6)) - (1)
```

## Contents

1. A brute-force solver for sacred geometry.  The brute force
   solution has exponential runtime in the number of ranks you have; it takes a
   prohibitively long time when you start using higher numbers of skill  ranks.  
   I used [this
   algorithm](http://www.codinghelmet.com/?path=exercises/expression-from-numbers)
   to solve this problem.

2. An analysis of how many ranks you need to make Sacred Geometry good.  You
   really don't need more than 7 ranks (8 to be safe) to make this feat succeed
   nearly every time.  **If you take Sacred Geometry, get eight ranks in
   Knowledge (Engineering) and call it a day.**

3. Eventually, this will contain an analysis of the arithmetic problem itself.
   This problem feels like it might be NP-complete, but I haven't managed a
   reduction yet.  But it feels like it should reduce to subset sum or 3SAT.

As always, comments and criticism are always welcome.  I especially welcome any
comments on my Rust, which is a new language for me.

