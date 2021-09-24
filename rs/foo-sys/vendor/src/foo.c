#include "foo.h"

int FooFill(FOO *Foo)
{
    static int i = 0;
    Foo->Value = i;
    i++;
    return i;
}
