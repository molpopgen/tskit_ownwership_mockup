#include "types.h"
#include <stdlib.h>

void
init_tree_sequence(table_collection* tables, tree_sequence* trees)
{
    trees->tables = tables;
}

void
free_tree_sequence(tree_sequence* trees)
{
    // cargo valgrind test fails if you comment out this line:
    free(trees->tables);
}
