#include <stdio.h>
#include <stdlib.h>

enum DataType {
    INT,
    FLOAT,
    STRING
};

struct string {
    void *data;
};

struct Data {
    enum DataType type;
    union {
        int integer;
        float floating;
        struct string string;
    } data;
};


void print_str(struct string *string) {
    printf("%s\n", (char *) string->data);
}

void print(struct Data *data) {
    switch (data->type) {
        case INT:
            printf("%d\n", data->data.integer);
            break;
        case FLOAT:
            printf("%f\n", data->data.floating);
            break;
        case STRING:
            print_str(&data->data.string);
            break;
    }
}

int main() {
    printf("Data Type Test\n");

    struct Data d1 = {STRING, {.string="hello there!"}};
    print(&d1);

    struct Data d2 = {INT, {.integer=42}};
    print(&d2);

    struct Data d3 = {FLOAT, {.floating=4.2f}};
    print(&d3);

    return 0;
}
