<h1 align="center">
	Tic Tac Toe
</h1>

**[Source Code](src/main.rs)**

Tic Tac Toe is a game made of a 3x3 grid where 2 players take turns marking vacant cells in attempts to form a line using 3 of their markers. X's and O's are usually used for the markers. Any row, column, or diagonal of the grid may be used to form a 3 marker line win condition. If there are no remaining vacant cells and neither player has 3 markers in a line, it is a draw.

#### New Game (Empty Grid)

||0|1|2|
|-|-|-|-|
|**0**||||
|**1**||||
|**2**||||

#### Win Condition Examples

X Wins (Row):

||0|1|2|
|-|-|-|-|
|**0**|O||O|
|**1**|O|X||
|**2**|~~X~~|~~X~~|~~X~~|

O Wins (Column):

||0|1|2|
|-|-|-|-|
|**0**|||~~O~~|
|**1**|X||~~O~~|
|**2**|X|X|~~O~~|

X Wins (Diagonal):

||0|1|2|
|-|-|-|-|
|**0**|O|O|~~X~~|
|**1**||~~X~~||
|**2**|~~X~~|||

#### Draw Condition Examples

||0|1|2|
|-|-|-|-|
|**0**|X|O|X|
|**1**|O|O|X|
|**2**|X|X|O|

## Input

_todo_
