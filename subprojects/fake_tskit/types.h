#pragma once

typedef struct
{
    int table_data;
} table_collection;

typedef struct
{
    const table_collection* tables;
} tree_sequence;

void init_tree_sequence(table_collection* tables, tree_sequence* trees);
void free_tree_sequence(tree_sequence* trees);
