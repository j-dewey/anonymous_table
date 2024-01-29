# Anonymous Table
---
### What is it?
An anonymous table is a table in which can hold any form of data, regardless of type or size. 

|     |     |     |
| --- | --- | --- |
| u16 | &str | u128 |
| String | bool | CustomStruct |
---
### Why is it helpful?
Imagine you are designing a simple 2d game which is populated with different shapes which all have their own properties. For one reason or another, these properties require that different shapes be different types. In order to keep track of all the shapes you decide to store them in a [Vec]. 

Since each shape is it's own special struct, you have options to store them:
- Using some enum `enum Shape{
    Shape1,
    Shape2,
    ...
}
Vec<Shape>
`
- Or having a Shape Trait and using `Vec<Box<dyn Shape>>`

Using an enum can become a burden as adding and removing shapes to the game requires updating the enum and match statements using it. 

Using trait objects could work, but just aren't fun or easy to use.

An [AnonymousTable] allows for simple storage and retrieval of data without having to do too much worrying about lifetimes or borrows (the table owns any data passed into it). 