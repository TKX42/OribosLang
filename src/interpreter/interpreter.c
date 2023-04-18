#include <stdio.h>
#include <stdlib.h>

#define NELEMS(x)  (sizeof(x) / sizeof((x)[0]))

enum DataType {
    INT,
    FLOAT,
    STRING,
    NIL
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

void runtime_error(char *msg) {
    printf("Runtime Error: %s", msg);
    exit(1);
}

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
        case NIL:
            runtime_error("Cannot print NIL");
            break;
    }
}

enum instr_type {
    LOAD,
    ASSIGN,
    CONST,
    PRINT
};

struct instr {
    enum instr_type type;
    struct Data data;
};

void instr_run(struct instr *instruction, struct Data *stack, int *sc) {
    switch (instruction->type) {
        case LOAD:
            break;
        case ASSIGN:
            break;
        case CONST:
            stack[*sc] = instruction->data;
            break;
        case PRINT:
            print(&stack[*sc]);
            sc--;
            break;
    }
}

void vm_run(struct instr *instructions, size_t len, struct Data *stack) {
    int pc = 0;
    int sc = 0;
    while (pc < len) {
        instr_run(&instructions[pc], stack, &sc);
        pc++;
    }
}

int main() {
    printf("ORIBOS_C\n");

    int STACK_SIZE = 256;
    struct Data stack[STACK_SIZE];

    struct instr instr_1 = {CONST, {INT, {.integer=42}}};
    //struct instr instr_1 = {CONST, {NIL}};
    struct instr instr_2 = {PRINT, {NIL}};
    struct instr instr_3 = {CONST, {STRING, {.string="ez"}}};
    struct instr instr_4 = {PRINT, {NIL}};

    struct instr instructions[] = {instr_1, instr_2, instr_3, instr_4};

    vm_run(&instructions, NELEMS(instructions), stack);

    return 0;
}
