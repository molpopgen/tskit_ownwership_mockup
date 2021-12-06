#include <stdlib.h>

typedef struct
{
    int table_data;
} table_collection;

typedef struct
{
    table_collection* tables;
} tree_sequence;

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
