1- Each resource can have only one mutable reference(constraint) or any number of immutable references at a time

2- References must be valid , the refenences that are being used must remain in scope.

3- a mutable reference cannot exist at the same time of any other reference , mutable or immutable .

4- For functions when there are multiple input reference or output reference parameters then compiler cannot understand/elide the lifetime of each and every reference.There developer has to inform the compiler about the lifetime of a reference.