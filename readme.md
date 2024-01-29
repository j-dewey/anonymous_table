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